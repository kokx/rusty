extern crate irc;
extern crate futures;
extern crate time;
extern crate regex;
#[macro_use]
extern crate lazy_static;
extern crate rand;

use rand::prelude::*;
use irc::client::prelude::*;
use tokio::timer::Interval;
use futures::{stream, Stream};
use std::time::Duration;
mod parse;
use core::ops::Add;
use regex::Regex;


fn timestream() -> impl Stream<Item = time::Tm, Error = ()> {
    let interval = Interval::new_interval(Duration::from_millis(1000))
        .map_err(|_e| ());

    stream::unfold((), |_| {
        let time_utc = time::now_utc();
        let time_utc = time_utc.add(time::Duration::milliseconds(800));
        Some(Ok((time_utc.to_local(), ())))
    }).zip(interval)
        .map(|(cur, _)| cur)
}

fn pwnage_wakeup(client: IrcClient) -> impl futures::Future<Item = (), Error = ()> {
    timestream()
        .filter(|cur| cur.tm_hour == 0 && cur.tm_min == 0 && (cur.tm_sec == 0 || cur.tm_sec == 1))
        .for_each(move |cur| {
            client.send_privmsg("PWNAGE", "Wake up").expect("Message couldn't send");
            println!("Waking up PWNAGE at {}", cur.rfc822());
            Ok(())
        })
}

fn pipo_wakeup(client: IrcClient) -> impl futures::Future<Item = (), Error = ()> {
    let mut rng = rand::thread_rng();
    timestream()
        .filter(|cur| (cur.tm_hour == 10 || cur.tm_hour == 13 || cur.tm_hour == 16) && cur.tm_min == 0 && cur.tm_sec == 0)
        .filter(move |_curtime| rng.gen_range(0, 42) < 32)
        .for_each(move |_curtime| {
            client.send_privmsg("Pipo", "Ik ben niet Pipo").expect("Message couldn't send");
            Ok(())
        })
}

fn is_admin(prefix: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"~kokx@kokx\.org$").unwrap();
    }

    RE.is_match(prefix)
}

/// Main rusty method
fn main() {
    let config = Config::load("irc.toml").unwrap();
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.inner_handle().spawn(pwnage_wakeup(client.clone()));
    reactor.inner_handle().spawn(pipo_wakeup(client.clone()));

    let mut got_nick = false;
    let mut nickserv_pass = "".to_string();
    let orig_nickname = config.nickname.unwrap_or("".to_string());

    if let Some(options) = &config.options {
        if let Some(pass) = options.get("nickserv_password") {
            nickserv_pass = pass.clone();
        }
    }

    reactor.register_client_with_handler(client, move |client, irc_msg| {
        print!("{}", irc_msg);
        match &irc_msg.command {
            Command::PRIVMSG(_, message) => {
                if let Some(command) = parse::parse_command(&message) {
                    // verify the nickname
                    if command.nick == client.current_nickname() {
                        if let Some(pref) = &irc_msg.prefix {
                            let source_nick = irc_msg.source_nickname().unwrap();
                            let response_target = irc_msg.response_target().unwrap();

                            match command.command {
                                parse::Command::QUIT => {
                                    if is_admin(pref) {
                                        client.send_quit(format!("Screw you guys, I'm going home"))
                                            .expect("Message couldn't be sent.");
                                    }
                                },
                                parse::Command::TIME => {
                                    let current_time = time::now().to_local();
                                    let response_msg = format!("Current time: {}", current_time.rfc822());
                                    client.send_privmsg(&response_target, response_msg)
                                        .expect("Message couldn't be sent.");
                                },
                                parse::Command::OP(None) => {
                                    if is_admin(pref) {
                                        let modes = [irc::proto::Mode::plus(irc::proto::ChannelMode::Oper, Some(source_nick))];
                                        client.send_mode(&response_target, &modes)
                                            .expect("Problem with making owner op");
                                    }
                                },
                                parse::Command::OP(Some(nick)) => {
                                    if is_admin(pref) {
                                        let modes = [irc::proto::Mode::plus(irc::proto::ChannelMode::Oper, Some(&nick))];
                                        client.send_mode(&response_target, &modes)
                                            .expect("Problem with making owner op");
                                    }
                                }
                            }
                        }
                    }
                }
            },
            Command::NOTICE(_, message) => {
                // check if this notice is sent by NickServ and is about nick ownership
                if let Some(nick) = irc_msg.source_nickname() {
                    if orig_nickname == client.current_nickname() && nick == "NickServ" && message.starts_with("This nickname is owned by someone else") {

                        client.send_privmsg("NickServ", format!("IDENTIFY {}", nickserv_pass))
                            .expect("Problem with identify");
                        got_nick = true;
                    }
                }
            },
            _ => ()
        }

        // verify our nickname, and ghost if we do not have it
        if !got_nick && orig_nickname != client.current_nickname() {
            client.send_privmsg("NickServ", format!("GHOST {} {}", orig_nickname, nickserv_pass))
                .expect("Problem with ghosting");
            client.send(Command::NICK(orig_nickname.to_string()))
                .expect("Problem with renaming");
            got_nick = true;
        }

        Ok(())
    });

    reactor.run().unwrap();
}
