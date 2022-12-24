mod machine;

use actix_cors::Cors;
use actix_session::{storage::CookieSessionStore, Session, SessionMiddleware};
use actix_web::middleware::Logger;
use actix_web::{cookie::Key, get, post, web, App, Error, HttpServer, Responder};
use env_logger::Env;
use serde::{Deserialize, Serialize};

#[derive(Serialize)]
struct MyObj {
    auth: bool,
    count: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct MyOtherObj {
    #[serde(default)]
    name: String,
    #[serde(default)]
    number: i32,
}

#[derive(Deserialize)]
struct InfiniteEngineRequest {
    foo: String,
    bar: String,
    address: Option<String>,
}

#[get("/health-check")]
async fn health_check() -> Result<impl Responder, Error> {
    Ok(web::Json(machine::StateMachine {
        auth: false,
        count: 0,
        is_secure: false,
    }))
}

#[post("/")]
async fn index(
    item: web::Json<MyOtherObj>,
    info: web::Query<InfiniteEngineRequest>,
    session: Session,
) -> Result<impl Responder, Error> {
    // Create the state machine.
    let mut sm = machine::StateMachine::new();
    println!("{}", item.name.to_string());
    // Handle the init event, including session setup.
    sm.handle_event(machine::Event::Init {
        foo: info.foo.to_string(),
        bar: info.bar.to_string(),
        session,
    })
    .await;

    // If an address is provided, handle the wallet events.
    if !info.address.is_none() {
        sm.handle_event(machine::Event::CheckBalance {
            address: "0xaf045cb0dbc1225948482e4692ec9dc7bb3cd48b".to_string(),
        })
        .await;
    }
    Ok(web::Json(sm.state()))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    HttpServer::new(|| {
        let cors = Cors::permissive();
        App::new()
            .app_data(web::JsonConfig::default().limit(4096)) // <- limit size of the payload (global configuration)
            .wrap(
                // create cookie based session middleware
                // @TODO Update key generation to be more secure. (Currently using fixed array of 64 zeros.)
                SessionMiddleware::builder(CookieSessionStore::default(), Key::from(&[0; 64]))
                    .cookie_secure(true)
                    .build(),
            )
            .wrap(Logger::default())
            .wrap(cors)
            .service(index)
            .service(health_check)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
