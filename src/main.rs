use teloxide::prelude::*;
use teloxide::utils::command::*;
use std::error::Error;
use serde::Deserialize;
use serde_json::json;
use teloxide::types::ParseMode::MarkdownV2;
use teloxide::utils::markdown::*;
use reqwest::Client;
use std::fmt::Debug;
use std::env::var;
use std::collections::HashMap;

#[derive(Deserialize,Debug)]
struct LangList {
    name:Vec<String>,
}
#[derive(Deserialize,Debug)]
struct ProjectList {
    projects:Vec<String>,
}
#[derive(Deserialize,Debug)]
struct ProjectDetails {
    name:String,
    link:String,
}
#[derive(BotCommand)]
#[command(rename="lowercase",description="These are the list of commands available at @parrothacker1_bot",parse_with="split")]
enum Command {
	#[command(description="To display help \n")]
        Help,
        #[command(description="To check if you are alive \n")]
        Alive,
        #[command(description="To send a specific text \n")]
        Sendtxt { text:String } ,
        #[command(description="To get my source code :-P \n")]
        Source,
        #[command(description="To get the ChatID and yourID \n")]
        Id,
        #[command(description="To respond to useless messages \n")]
        WhyAsk,
        #[command(description="To start the bot \n")]
        Start
}
async fn answer(
    cx: UpdateWithCx<AutoSend<Bot>, Message>,
    command: Command,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let client=Client::new();

    match command {
        Command::Help => cx.answer(Command::descriptions()).send().await?,
        Command::Sendtxt { text }  => {
                if text.trim().is_empty() {
                    cx.reply_to("Are you  kidding with me ? Send  text along with the command dude ! ").await?
                } else {
                    cx.reply_to(text).parse_mode(MarkdownV2).await?
            } 
        }
        Command::Source => cx.reply_to(format!("My Source code is {}",link("https://gitlab.com/parrothacker1/parrothacker1-bot/","this"))).parse_mode(MarkdownV2).await?,
        Command::Id => cx.reply_to(format!("Your CHATID:`{}`\nGroup CHATID:`{}`",cx.update.from().unwrap().id.to_string(),cx.update.chat.id.to_string())).parse_mode(MarkdownV2).await?,
        Command::WhyAsk => cx.reply_to(format!("Why the hell are you asking it here.Please open the goddamn google which is inbuilt in your phone or browse into it and type the same question there.You will get alot of answers which are written by ofc professionals.Read them and understand.This place is not a school !")).await?,
        Command::Start => cx.answer(format!("Hello {} I'm @parrothacker1_bot.A bot made in rust by @parrothacker1.\n\nJust run /help to find out information about the commands",cx.update.from().unwrap().first_name)).await?,
        Command::Alive => cx.reply_to("You are alive kiddo XD !").await?,
    };

    Ok(())
}

#[tokio::main]
async fn main() {
    run().await;
}

async fn run() {
    teloxide::enable_logging!();
    log::info!("Starting parrothacker1-bot");

    let bot = Bot::from_env().auto_send();

    let bot_name: String ="parrothacker1_bot".to_string();
    teloxide::commands_repl(bot, bot_name, answer).await;
}
