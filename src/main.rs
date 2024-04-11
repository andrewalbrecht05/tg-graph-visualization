use std::env;
use std::fs::File;
use std::io::{Read, Write};
use std::process::{Command, Stdio};
use dotenv::dotenv;
use log::{info, trace};
use teloxide::Bot;
use teloxide::payloads::SendMessageSetters;
use teloxide::prelude::{Message, Requester, ResponseResult};
use teloxide::types::{InputFile, ParseMode};
use teloxide::utils::command::BotCommands;
use teloxide::update_listeners::webhooks;
use tg_graphviz::{Graph};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    let bot = Bot::from_env();
    info!("Successfully initialized token");

    /*// Heroku auto defines a port value
    let port: u16 = env::var("PORT")
        .expect("PORT env variable is not set")
        .parse()
        .expect("PORT env variable value is not an integer");

    let addr = ([127, 0, 0, 2], port).into();

    // Heroku host example: "heroku-ping-pong-bot.herokuapp.com"
    let host = env::var("HOST").expect("HOST env variable is not set");
    let url = format!("https://{host}/webhook").parse().unwrap();

    let listener = webhooks::axum(bot.clone(), webhooks::Options::new(addr, url))
        .await
        .expect("Couldn't setup webhook");
    */
    teloxide::repl(bot, |bot: Bot, msg: Message| async move {
        if let Some(message) = msg.text() {
            if message.starts_with('/') {
                trace!("Commands '/' detected");
                let cmd = Commands::parse(message, "graph_vizualization_bot")
                    .unwrap_or(Commands::Help);
                answer(bot, msg, cmd).await.expect("Err with commands");
                return Ok(());
            }
            let mut graph = Graph::new(false,"circo".into(), "".into(), stringify!(width=0.5 height=0.5 fontname="Arial").to_string());
            let res = graph.try_parse(message);
            // match all possible errors
            if res.is_err() {
                bot.send_message(msg.chat.id, "Input is incorrect! Try agian.").await?;
                return Ok(());
            }
            let dot = graph.to_dot();
            trace!("Starting new command to get png...");
            let output = Command::new("dot")
                .arg("-Tpng")  // Specify the output format
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()?;
            trace!("Writing dot list to stdin...");
            output.stdin.unwrap().write_all(dot.as_bytes())?;
            trace!("List write success...");
            // Read the output from the command's stdout
            trace!("Reading stdout we got...");
            let mut png_data = Vec::new();
            output.stdout.unwrap().read_to_end(&mut png_data)?;
            // here i get png from stdout
            trace!("Success reading stdout...");
            let mut file = File::create("photo.png")?;
            file.write_all(&png_data.clone()).expect("TODO: panic message");
            trace!("Making photo from ans we got...");
            let photo = InputFile::memory(png_data);

            trace!("Sending message response to user...");
            bot.send_photo(msg.chat.id, photo)
                .await?;
        }
        Ok(())
    }).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Commands {
    #[command(description = "List of supported commands")]
    Help,
    #[command(description = "Instruction how to use bot")]
    How,
    #[command(description = "Contact with creator of the bot")]
    Contact,
}
async fn answer(bot: Bot, msg: Message, cmd: Commands) -> ResponseResult<()> {
    match cmd {
        Commands::Help => bot.send_message(msg.chat.id, Commands::descriptions().to_string()),
        Commands::How => bot.send_message(msg.chat.id,
        "🤖**Graph Visualizer Bot**\n\
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
        Commands::Contact => bot.send_message(msg.chat.id, "Andrew Albrekht"),
    }.await?;
    Ok(())
}