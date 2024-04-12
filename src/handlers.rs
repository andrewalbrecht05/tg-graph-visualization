use std::io::{Read, Write};
use std::process::{Command, Stdio};
use log::{debug, trace};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::utils::command::BotCommands;
use teloxide::prelude::*;
use teloxide::RequestError;
use teloxide::types::{InputFile, ParseMode};
use tg_graphviz::{Graph, number_of_lines, GraphSyntaxError};

type MyDialogue = Dialogue<State, InMemStorage<State>>;

/// Represents the different states of the bot dialogue with the user
#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveImage { msg_text: String },
}

/// Handles the initial state, prompting the user for graph type (directed / undirected)
pub async fn start(
    bot: Bot,
    dialogue: MyDialogue,
    msg: Message)
    -> Result<(), RequestError> {
    trace!("This is message in start state: {}", msg.text().unwrap());
    bot.send_message(msg.chat.id, "Is your graph directed?(Y\\n)").await?;

    // Passes to the next state
    dialogue.update(State::ReceiveImage { msg_text: msg.text().unwrap().to_string() }).await.unwrap();
    Ok(())
}

/// Handles the state where the bot receives the graph representation
pub async fn receive_image(
    bot: Bot,
    dialogue: MyDialogue,
    msg_text: String,
    msg: Message)
    -> Result<(), RequestError> {
    let direction_choice = msg.text().unwrap().trim();
    trace!("This is message in receive state: {}", direction_choice);

    // Parses the user's input to determine if the graph is directed
    let directed: Option<bool> = match direction_choice.to_lowercase().as_str() {
        "y" => Some(true),
        "n" => Some(false),
        _ => None
    };

    if directed.is_none() {
        bot.send_message(msg.chat.id, "Invalid choice! Try again.(Y\\n)").await.unwrap();
        return Ok(());
    }

    // Configures layout based on complexity
    let layout = String::from(if number_of_lines(&msg_text) <= 10 { "circo" } else { "neato" });

    // Creates a new Graph instance
    let mut graph = Graph::new(
        directed.unwrap(),
        layout,
        String::from(""),
        stringify!(width=0.5 height=0.5 fontname="Arial").to_string(),
    );
    // Attempt to parse the graph representation from the user's message
    let res = graph.try_parse(msg_text);

    // Handles potential errors during parsing
    match res {
        Err(GraphSyntaxError::LabelTooLargeError) => {
            bot.send_message(msg.chat.id, "The number of letters in nodes and labels names should not exceed 10!\n\
            Try again.").await?;
        }
        Err(GraphSyntaxError::ListTooLargeError) => {
            bot.send_message(msg.chat.id, "The number of lines in your message should not exceed 50!\n\
            Try again.").await?;
        }
        Ok(()) => {} // No error, proceed with graph generation
    };
    if res.is_err() {
        dialogue.update(State::Start).await.unwrap();
        return Ok(());
    }
    // Converts graph into DOT format
    let dot = graph.to_dot();

    // Creates an image from the DOT representation
    let photo = create_photo(dot)?;

    trace!("Sending message response to user...");

    // Reset dialogue state to the beginning
    dialogue.update(State::Start).await.unwrap();
    bot.send_photo(msg.chat.id, photo).await?;
    
    Ok(())
}

/// Generates an image from the graph's DOT representation using Graphviz
fn create_photo(dot: String) -> Result<InputFile, RequestError> {
    let output = Command::new("dot")
        .arg("-Tpng")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    output.stdin.unwrap().write_all(dot.as_bytes())?;

    let mut png_data = Vec::new();
    output.stdout.unwrap().read_to_end(&mut png_data)?;
    trace!("Success writing photo bytes");
    let photo = InputFile::memory(png_data);

    debug!("Successfully created photo");
    Ok(photo)
}

/// Enum representation of possible commands with built-in description
#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum Commands {
    #[command(description = "List of supported commands")]
    Help,
    #[command(description = "Instruction how to use bot")]
    How,
    #[command(description = "Contact with creator of the bot")]
    Contact,
}

/// Handler that provides answer to commands questions
pub async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Help => bot.send_message(msg.chat.id, Commands::descriptions().to_string()),
        Commands::How => bot.send_message(msg.chat.id,
                                          "🤖**Graph Visualizer Bot**🤖\n\
        This bot allows you to convert a graph represented as a list of vertices into a image\\.\n\
        *How to use*:\n\
        1\\. Send a list of vertices in the following format:\n\
        ```\nA B\nB C\nC D```\
        Each line represents an edge connecting two vertices\\. Vertices are separated by a space\\.\n\
        2\\. Optionally, you can add a third parameter as the label for the edge:\n\
        ```\nA B Edge1\nB C Edge2\nC D Edge3```\
        3\\. Also, you can add just nodes in a single line:\n\
        ```\nA B\nC\nD```\
        4\\. The bot will convert the graph into a PNG image and send it back to you\\.\n\
        5\\.Enjoy\\!\n").parse_mode(ParseMode::MarkdownV2),
        Commands::Contact => bot.send_message(msg.chat.id, "My tg: @andrew055\n\
        My email: andrijalbrext@gmail.com"),
    }.await?;
    Ok(())
}