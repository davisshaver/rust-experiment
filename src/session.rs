use actix_session::Session;

pub fn get_session(session: Session) -> i32 {
    let mut new_count = 1;
    if let Ok(Some(count)) = session.get::<i32>("counter") {
        new_count = count + 1;
    }
    let session_result = session.insert("counter", new_count);
    if !session_result.is_ok() {
        println!("Failed to insert session");
    }
    return new_count;
}
