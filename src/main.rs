extern crate irc;
//#[macro_use]
extern crate futures;
extern crate time;

use irc::client::prelude::*;

use tokio::timer::Delay;
//use futures::{Future, Async, Poll};
use futures::Future;

//use std::fmt;
use std::time::{Duration, Instant};

fn send_every_minute(client : IrcClient, reactor: &mut IrcReactor) {
    let when = Instant::now() + Duration::from_millis(1000);
    let task = Delay::new(when)
        .and_then(move |_| {
            client.send_privmsg("#test", "Hi there!");
            Ok(())
        })
        .map_err(|e| panic!("delay errored; err={:?}", e));

    reactor.inner_handle().spawn(task);
}

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

    send_every_minute(client.clone(), &mut reactor);

    reactor.register_client_with_handler(client, |client, irc_msg| {
        print!("Incoming: {}", irc_msg);
        if let Command::PRIVMSG(channel, message) = irc_msg.command {
            if message.contains(client.current_nickname()) {
                if message.contains("!quit") {
                    client.send_quit(format!("Screw you guys, I'm going home"));
                } else {
                    client.send_privmsg(&channel, "Ja?");
                }
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}
