use regex::Regex;

/// Represents:
/// <nick>:? <command> [<arg1> [<arg2> [<arg3> ...]]]
#[derive(Debug)]
struct ParsedMessage {
    pub nick: String,
    pub command: String,
    pub args: Vec<String>
}

impl ParsedMessage {
    /// Split a message into a ParsedMessage
    /// <nick>:? <command> [<arg1> [<arg2> [<arg3> ...]]]
    fn from_string(message: &str) -> Option<ParsedMessage> {
        let re = Regex::new(r"^([0-9a-zA-Z]+):? ([0-9a-zA-Z]+)( .*)?$").unwrap();
        let cap = re.captures(message);

        if let Some(cap) = cap {
            // gather arguments only if there are arguments
            let mut parsed_args: Vec<String> = vec![];
            if let Some(args) = cap.get(3) {
                parsed_args = args.as_str().trim().split(" ").map(|s| String::from(s)).collect();
            }

            Some(ParsedMessage {
                nick: cap[1].to_string(),
                command: cap[2].to_string(),
                args: parsed_args
            })
        } else {
            None
        }
    }
}

/// Store the exact command taken
#[derive(Debug)]
pub enum Command {
    QUIT,
    TIME,
    OP(Option<String>),
}

impl Command {
    /// Create a command from 
    fn from_parsed(msg: &ParsedMessage) -> Option<Command> {
        match msg.command.as_ref() {
            "quit" => Some(Command::QUIT),
            "time" => Some(Command::TIME),
            "op" => {
                if let Some(nick) = msg.args.get(0) {
                    Some(Command::OP(Some(nick.to_string())))
                } else {
                    Some(Command::OP(None))
                }
            },
            _ => None
        }
    }
}

/// Command, and information about who called the command
#[derive(Debug)]
pub struct CommandMessage {
    pub nick: String,
    pub command: Command
}

/// Parse a message and gain information about the command
pub fn parse_command(message: &str) -> Option<CommandMessage> {
    if let Some(msg) = ParsedMessage::from_string(message) {
        if let Some(command) = Command::from_parsed(&msg) {
            Some(CommandMessage {
                nick: msg.nick,
                command
            })
        } else {
            None
        }
    } else {
        None
    }
}
