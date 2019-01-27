#[derive(Debug)]
pub struct ParsedMessage {
    nick: String,
    command: String,
    args: Vec<String>
}

impl ParsedMessage {
    pub fn new(nick: String, command: String, args: Vec<String>) -> Self {
        ParsedMessage {
            nick,
            command,
            args
        }
    }
}

/// We will split a message into several parts:
/// <nick>:? <command> [<arg1> [<arg2> [<arg3> ...]]]
pub fn parse_message(message: &str) -> ParsedMessage {
    let v = vec!["a".to_string(), "b".to_string()];

    ParsedMessage::new("kokx".to_string(), "test".to_string(), v)
}
