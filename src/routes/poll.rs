use rocket::*;
use rocket_contrib::json;
use rocket::State;

use mongodb::{bson::doc, sync::Database};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Poll {
    question: String,
    description: String,
    options: Vec<String>
}

#[post("/api/poll", format = "application/json", data = "<poll>")]
pub fn post(client: State<Database>, poll: json::Json<Poll>) -> json::JsonValue {
    let poll_collection = client.collection("polls");

    let val = poll.into_inner();

    let mut document = doc!{
        "question": val.question,
        "description": val.description,
    };

    let mut options = doc!{};

    for choice in val.options {
        options.insert(choice, 1);
    };

    document.insert("options", options);

    println!("{}", &document);

    poll_collection.insert_one(document, None).unwrap();

    return json!({
        "status": "success",
    });
}