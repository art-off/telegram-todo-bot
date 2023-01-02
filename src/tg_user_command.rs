use teloxide::utils::command::BotCommands;

#[derive(BotCommands, Clone)]
#[command(rename_rule = "lowercase", description = "These commands are supported:")]
pub enum TgUserCommand {
    #[command(description = "Display this text")]
    Help,
    #[command(description = "Create new todo")]
    New { todo_text: String, },
    #[command(description = "Show list of todo")]
    List,
}