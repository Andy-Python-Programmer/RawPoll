use rocket::*;
use rocket_contrib::json;
use rocket::State;

use mongodb::{bson::doc, sync::Database, bson::oid::ObjectId};
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

    document.insert("options", doc! {
        "types": options,
        "ips": []
    });

    let inserted = poll_collection.insert_one(document, None).unwrap();
    let id = inserted.inserted_id.as_object_id().unwrap().to_string();

    return json!({
        "status": "success",
        "id": &id
    });
}

#[get("/api/poll/<id>")]
pub fn get(client: State<Database>, id: String) -> json::JsonValue {
    let poll_collection = client.collection("polls");

    let res = poll_collection.find_one(Some(
        doc! {
            "_id": ObjectId::with_string(id.as_str()).unwrap()
        }
    ), None);

    match res {
        Ok(val) => {
            let result = val.unwrap();

            return json!({
                "question": result.get("question"),
                "description": result.get("description"),
                "options": result.get("options").unwrap().as_document().unwrap().get("types")
            })
        }

        Err(_) => {
            return json!({
                "status": "failure",
                "error": "Cannot find the poll specified!"
            })
        }
    }
}