extern crate irc;
//#[macro_use]
extern crate futures;
extern crate time;
extern crate regex;

use irc::client::prelude::*;

use tokio::timer::Interval;
use futures::{stream, Stream};

use std::time::Duration;

mod parse;


fn timestream() -> impl Stream<Item = time::Tm, Error = ()> {
    let interval = Interval::new_interval(Duration::from_millis(1000))
        .map_err(|_e| ());

    stream::unfold((), |_| {
        Some(Ok((time::now().to_local(), ())))
    }).zip(interval)
        .map(|(cur, _)| cur)
}

fn pwnage_wakeup(client: IrcClient) -> impl futures::Future<Item = (), Error = ()> {
    timestream()
        .filter(|cur| cur.tm_hour == 8 && cur.tm_min == 39 && cur.tm_sec == 0)
        .for_each(move |_curtime| {
            client.send_privmsg("PWNAGE", "!nieuwedag").expect("Message couldn't send");
            client.send_privmsg("kokx", "Test Wake up!").expect("Message couldn't send");
            Ok(())
        })
}

fn pipo_wakeup(client: IrcClient) -> impl futures::Future<Item = (), Error = ()> {
    timestream()
        .filter(|cur| (cur.tm_hour == 10 || cur.tm_hour == 12 || cur.tm_hour == 14 || cur.tm_hour == 16) && cur.tm_min == 0 && cur.tm_sec == 0)
        .for_each(move |_curtime| {
            client.send_privmsg("Pipo", "Ik ben niet Pipo").expect("Message couldn't send");
            Ok(())
        })
}

/// Main rusty method
fn main() {
    let config = Config::load("irc.toml").unwrap();
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.inner_handle().spawn(pwnage_wakeup(client.clone()));
    reactor.inner_handle().spawn(pipo_wakeup(client.clone()));

    reactor.register_client_with_handler(client, |client, irc_msg| {
        print!("Incoming: {}", irc_msg);
        match &irc_msg.command {
            Command::PRIVMSG(_channel, message) => {
                if let Some(parsed) = parse::parse_message(message) {
                    if parsed.nick == client.current_nickname() {
                        let pref = irc_msg.clone().prefix.unwrap();
                        let response_target = irc_msg.response_target().unwrap();

                        match parsed.command.as_ref() {
                            "quit" => {
                                client.send_quit(format!("Screw you guys, I'm going home"))
                                    .expect("Message couldn't be sent.");
                            },
                            "time" => {
                                client.send_privmsg(&response_target, format!("Current time: {}", time::now().to_local().rfc822()))
                                    .expect("Message couldn't be sent.");
                            },
                            "op" => {
                                let modes = [irc::proto::Mode::plus(irc::proto::ChannelMode::Oper, Some("kokx"))];
                                client.send_mode(&response_target, &modes)
                                    .expect("Problem with making owner op");
                            },
                            _ => {
                                client.send_privmsg(&response_target, "Ja?").expect("Message couldn't be sent.");
                            }
                        }
                    }
                }
            },
            _ => ()
        }
        Ok(())
    });

    reactor.run().unwrap();
}
