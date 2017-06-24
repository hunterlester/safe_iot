extern crate safe_core;
/*
extern crate dotenv;
extern crate tokio_core;
extern crate mio;

#[macro_use]
extern crate unwrap;

use dotenv::dotenv;
use std::env;
use std::io;
use mio::channel;

use safe_core::{Client, CoreError};

use tokio_core::reactor::{Handle, PollEvented, Core};

struct Sender<T> {
    tx: channel::Sender<T>,
}

struct Receiver<T> {
    rx: PollEvented<channel::Receiver<T>>,
}

impl<T> Clone for Sender<T> {
    fn clone(&self) -> Sender<T> {
        Sender { tx: self.tx.clone() }
    }
}


fn channel2<T>(handle: &Handle) -> io::Result<(Sender<T>, Receiver<T>)>
    where T: Send + 'static,
{
    let (tx, rx) = channel::channel();
    let rx = try!(PollEvented::new(rx, handle));
    Ok((Sender { tx: tx }, Receiver { rx: rx }))
}
*/


fn main() {
/*
    dotenv().expect("Failed to read .env file");

    let mut SEED: String = String::new();

    for (key, value) in env::vars() {
      if key == "SEED" {
        SEED = value;
      }
    }

    let el = unwrap!(Core::new());
    let el_h = el.handle();

    let (core_tx, _core_rx) = unwrap!(channel2(&el_h));
    let (net_tx, _net_rx) = unwrap!(channel2(&el_h));

    match Client::registered_with_seed::<()>(&SEED,
                                             el_h.clone(),
                                             core_tx.clone(),
                                             net_tx.clone()) {
      Ok(_) => (),
      Err(err) => panic!("{:?}", err),
    }
*/
    println!("Account Created Successfully !!");

}
