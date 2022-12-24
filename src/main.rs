use rust_fsm::*;
use serde_derive::{Deserialize, Serialize};
use warp::Filter;

#[derive(Debug)]
enum CircuitBreakerInput {
    Successful,
    Unsuccessful,
    TimerTriggered,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum CircuitBreakerState {
    Closed,
    Open,
    HalfOpen,
}
#[derive(Deserialize, Serialize)]
struct MyOtherObject {
    key1: String,
}

#[derive(Debug, PartialEq)]
struct CircuitBreakerOutputSetTimer;

#[derive(Debug)]
struct CircuitBreakerMachine;

impl StateMachineImpl for CircuitBreakerMachine {
    type Input = CircuitBreakerInput;
    type State = CircuitBreakerState;
    type Output = CircuitBreakerOutputSetTimer;
    const INITIAL_STATE: Self::State = CircuitBreakerState::Closed;

    fn transition(state: &Self::State, input: &Self::Input) -> Option<Self::State> {
        match (state, input) {
            (CircuitBreakerState::Closed, CircuitBreakerInput::Unsuccessful) => {
                Some(CircuitBreakerState::Open)
            }
            (CircuitBreakerState::Open, CircuitBreakerInput::TimerTriggered) => {
                Some(CircuitBreakerState::HalfOpen)
            }
            (CircuitBreakerState::HalfOpen, CircuitBreakerInput::Successful) => {
                Some(CircuitBreakerState::Closed)
            }
            (CircuitBreakerState::HalfOpen, CircuitBreakerInput::Unsuccessful) => {
                Some(CircuitBreakerState::Open)
            }
            _ => None,
        }
    }

    fn output(state: &Self::State, input: &Self::Input) -> Option<Self::Output> {
        match (state, input) {
            (CircuitBreakerState::Closed, CircuitBreakerInput::Unsuccessful) => {
                Some(CircuitBreakerOutputSetTimer)
            }
            (CircuitBreakerState::HalfOpen, CircuitBreakerInput::Unsuccessful) => {
                Some(CircuitBreakerOutputSetTimer)
            }
            _ => None,
        }
    }
}

#[derive(Deserialize, Serialize)]
struct MyObject {
    key1: String,
    key2: u32,
    key5: String,
}

#[tokio::main]
async fn main() {
    pretty_env_logger::init();
    warp::serve(
        warp::get()
            .and(warp::path("infinite-state-machine"))
            .and(warp::query::<MyObject>().map(Some).or_else(|_| async {
                Ok::<(Option<MyObject>,), std::convert::Infallible>((None,))
            }))
            .map(|p: Option<MyObject>| match p {
                Some(obj) => {
                    let mut machine: StateMachine<CircuitBreakerMachine> = StateMachine::new();
                    let res = machine.consume(&CircuitBreakerInput::Unsuccessful).unwrap();
                    assert_eq!(res, Some(CircuitBreakerOutputSetTimer));
                    let a = if machine.state() == &CircuitBreakerState::Open {
                        String::from("unlocked")
                    } else {
                        String::from("locked")
                    };

                    warp::reply::json(&MyOtherObject { key1: a })
                }
                None => warp::reply::json(&MyOtherObject {
                    key1: String::from("foo"),
                }),
            }),
    )
    .run(([127, 0, 0, 1], 3000))
    .await
}
