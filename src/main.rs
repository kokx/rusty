extern crate irc;
//#[macro_use]
extern crate futures;
extern crate time;

use irc::client::prelude::*;

use tokio::timer::Interval;
use futures::{stream, Stream};

use std::time::Duration;


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
        .filter(|cur| cur.tm_hour == 0 && cur.tm_min == 0 && cur.tm_sec == 0)
        .for_each(move |_curtime| {
            client.send_privmsg("PWNAGE", "Wake up!").expect("Message couldn't send");
            Ok(())
        })
}

fn pipo_wakeup(client: IrcClient) -> impl futures::Future<Item = (), Error = ()> {
    timestream()
        .filter(|cur| (cur.tm_hour == 15 || cur.tm_hour == 16 || cur.tm_hour == 10) && cur.tm_min == 0 && cur.tm_sec == 0)
        .for_each(move |_curtime| {
            client.send_privmsg("Pipo", "Ik ben niet Pipo").expect("Message couldn't send");
            Ok(())
        })
}

fn main() {
    let config = Config::load("irc.toml").unwrap();
    let mut reactor = IrcReactor::new().unwrap();
    let client = reactor.prepare_client_and_connect(&config).unwrap();
    client.identify().unwrap();

    reactor.inner_handle().spawn(pwnage_wakeup(client.clone()));
    reactor.inner_handle().spawn(pipo_wakeup(client.clone()));

    reactor.register_client_with_handler(client, |client, irc_msg| {
        print!("Incoming: {}", irc_msg);
        if let Command::PRIVMSG(channel, message) = irc_msg.command {
            if message.starts_with(client.current_nickname()) {
                if message.contains("!quit") {
                    client.send_quit(format!("Screw you guys, I'm going home")).expect("Message couldn't be sent.");
                } else if message.contains("!time") {
                    client.send_privmsg(&channel, format!("Current time: {}", time::now().to_local().rfc822())).expect("Message couldn't be sent.");
                } else {
                    client.send_privmsg(&channel, "Ja?").expect("Message couldn't be sent.");
                }
            }
        }
        Ok(())
    });

    reactor.run().unwrap();
}
