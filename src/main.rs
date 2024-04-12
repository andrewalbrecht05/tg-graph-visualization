mod handlers;

use dotenv::dotenv;
use log::info;
use teloxide::{Bot, dptree};
use teloxide::dispatching::{HandlerExt, UpdateFilterExt};
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::{Dispatcher, Message, Requester, Update};
use teloxide::utils::command::BotCommands;
use crate::handlers::{Commands, State, answer, receive_image, start};

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init_timed();
    run().await;
}

/// Handles the core logic of the bot
async fn run() {
    let bot = Bot::from_env();
    info!("Successfully initialized token");

    // Sets the available commands for the bot
    bot.set_my_commands(Commands::bot_commands())
        .await
        .expect("Failed to set bot commands");
    info!("Bot commands have been set successfully");

    // Constructs the dialogue handler tree
    let handler = dptree::entry()
        .branch(Update::filter_message()
            .filter_command::<Commands>()
            .endpoint(answer))
        .branch(Update::filter_message()
            .enter_dialogue::<Message, InMemStorage<State>, State>()
            .branch(dptree::case![State::Start].endpoint(start))
            .branch(dptree::case![State::ReceiveImage{ msg_text }].endpoint(receive_image)));

    // Builds a dispatcher, responsible for handling Telegram updates
    Dispatcher::builder(bot, handler)
        .dependencies(dptree::deps![InMemStorage::<State>::new()])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}
