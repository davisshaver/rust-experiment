#[path = "wallet.rs"]
mod wallet;
#[path = "session.rs"]
mod session;
use actix_session::Session;
use serde::{Serialize};

pub enum Event {
    Init { foo: String, bar: String, session: Session },
    CheckBalance { address: String }
    // Successful,
    // Unsuccessful,
}


#[derive(Serialize)]
pub struct StateMachine {
    pub auth: bool,
    pub count: i32,
    #[serde(skip_serializing)]
    pub is_secure: bool,
}

impl StateMachine {

    pub fn state(self) -> StateMachine {
        return self;
    }

    pub fn new() -> StateMachine {
        StateMachine { auth: false, count: 0, is_secure: true }
    }

    pub async fn handle_event(&mut self, event: Event) {
        match event {
            Event::Init { foo, bar, session } => {
                // Use the username and password to update the authorization status
                if foo == "foo" && bar == "bar" {
                    self.auth = true;
                }
                self.count = session::get_session(session);
            }
            Event::CheckBalance { address } => {
                let balance = wallet::get_price(address).await;
                println!("Dirt balance: {balance:?}");
            }
        }
    }
}

