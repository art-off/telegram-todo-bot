use teloxide::{prelude::*, utils::command::BotCommands};
use teloxide::types::{User, Message, MessageKind};

enum TodoItemStatus {
    Done,
    New
}

struct TodoItem {
    status: TodoItemStatus,
    text: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    log::info!("Starting command bot...");

    let bot = Bot::from_env();
    Command::repl(bot, answer).await;
}

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
enum Command {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Create new todo")]
    New { todo_text: String, },
    #[command(description = "Show list of todo")]
    List,
}

async fn answer(bot: Bot, msg: Message, cmd: Command) -> ResponseResult<()> {
    match cmd {
        Command::Help => bot.send_message(msg.chat.id, Command::descriptions().to_string()).await?,
        Command::New { todo_text } => {
            match user_by_msg(msg.clone) {
                Some(user) => bot.send_message(msg.chat.id, user.id.to_string()).await?,
                None => bot.send_message(msg.chat.id, "No user").await?,
            }
        },
        Command::List => bot.send_message(msg.chat.id, "todo_list.todo_items.len().to_string()").await?,
    };

    Ok(())
}

fn user_by_msg(msg: Message) -> Option<User> {
    match msg.kind {
        MessageKind::Common(messageCommon) => messageCommon.from,
        _ => None
    }
}