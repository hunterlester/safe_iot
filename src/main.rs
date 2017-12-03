// Use: cargo build --target=arm-unknown-linux-gnueabihf --features="use-mock-routing"

extern crate safe_authenticator;
extern crate maidsafe_utilities;

#[macro_use]
extern crate log;

use safe_authenticator::{Authenticator};
use maidsafe_utilities::log as safe_log;

#[macro_use]
extern crate unwrap;

fn main() {

    unwrap!(safe_log::init(false));

    let username = String::from("someusernamethatmeetsrequirements");
    let password = String::from("somepasswordthatmeetsrequirements");
    let invitation = String::from("someinvitation");

    let auth = match Authenticator::create_acc(username.clone(), password.clone(), invitation.clone(), |_| ()) {
      Ok(authenticator) => authenticator,
      Err(auth_error) => {
        panic!("Authentication error: {:?}", auth_error)
      },
    };

    info!("autho info");

    println!("You are logged in!");
}
