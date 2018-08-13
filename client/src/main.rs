extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;
use std::sync::{Arc, Mutex};
use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::{Api, UpdateKind, MessageKind, ChatId, MessageId, Integer, CanSendMessage};
use pool::ThreadPool;

mod pool;
mod provider;

fn main() {
    let mut core = Core::new().unwrap();
    let token = env::var("TELEGRAM_TOKEN").expect("parse TELEGRAM_TOKEN from env");
    let api = Api::configure(token).build(core.handle()).expect("build telegram api");

    let pool = ThreadPool::new(4);

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text {ref data, ..} = message.kind {
                pool.execute( || {
                    println!("sleep for 5 seconds");
                    std::thread::sleep(std::time::Duration::from_secs(5));
                    println!("<{}>: {}", &message.from.first_name, data);


//                    t_api(message.chat.text(
//                        format!("You wrote '{}'", data)
//                    ));
                });
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
