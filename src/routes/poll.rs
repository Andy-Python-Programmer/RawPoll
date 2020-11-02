use rocket::*;
use rocket_contrib::json;
use rocket::State;

use mongodb::{bson::doc, sync::Database};

use serde_json::{ Value, Error };

#[post("/api/poll", format = "application/json", data = "<poll>")]
pub fn post(client: State<Database>, poll: String) -> json::JsonValue {
    let poll_collection = client.collection("polls"); 

    let res: Result<Value, Error> = serde_json::from_str(&poll);

    match res {
        Ok(val) => {
            let mut document = doc!{
                "question": val["question"].as_str().unwrap(),
                "description": val["description"].as_str().unwrap(),
            };

            let mut options = doc!{};

            for choice in val["options"].as_array().unwrap() {
                options.insert(choice.as_str().unwrap(), 1);
            };

            document.insert("options", options);

            println!("{}", &document);

            poll_collection.insert_one(document, None).unwrap();

            return json!({
                "status": "success",
            });
        }

        Err(err) => {
            return json!({
                "status": "failure",
                "error": "Invalid JSON data was provided",
                "logs": err.to_string()
            });
        }
    }
}