extern crate irc;

use irc::client::prelude::*;

fn main() {
    let config = Config {
        nickname: Some("rusty".to_owned()),
        server: Some("irc.gewis.nl".to_owned()),
        channels: Some(vec!["#test".to_owned()]),
        ..Config::default()
    };

    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.register_client_with_handler(client, |client, message| {
        print!("Incoming: {}", message);

        Ok(())
    });

    reactor.run().unwrap();
}
