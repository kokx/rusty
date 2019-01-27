use regex::Regex;

#[derive(Debug)]
pub struct ParsedMessage {
    pub nick: String,
    pub command: String,
    pub args: Vec<String>
}

/// Represents:
/// <nick>:? <command> [<arg1> [<arg2> [<arg3> ...]]]
impl ParsedMessage {
    pub fn new(nick: String, command: String, args: Vec<String>) -> Self {
        ParsedMessage {
            nick,
            command,
            args
        }
    }
}

/// Split a message into a ParsedMessage
/// <nick>:? <command> [<arg1> [<arg2> [<arg3> ...]]]
pub fn parse_message(message: &str) -> Option<ParsedMessage> {
    let re = Regex::new(r"^([0-9a-zA-Z]+):? ([0-9a-zA-Z]+)( .*)?$").unwrap();
    let cap = re.captures(message);

    if let Some(cap) = cap {
        // gather arguments only if there are arguments
        let mut parsed_args: Vec<String> = vec![];
        if let Some(args) = cap.get(3) {
            parsed_args = args.as_str().trim().split(" ").map(|s| String::from(s)).collect();
        }

        Some(ParsedMessage::new(cap[1].to_string(), cap[2].to_string(), parsed_args))
    } else {
        None
    }
}
