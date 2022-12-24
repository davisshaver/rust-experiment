use serde_derive::{Deserialize, Serialize};
use warp::Filter;

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
                Some(obj) => warp::reply::json(&obj),
                None => warp::reply::json(&MyObject {
                    key1: String::from("foo"),
                    key2: 42,
                    key5: String::from("bar"),
                }),
            }),
    )
    .run(([127, 0, 0, 1], 3000))
    .await
}
