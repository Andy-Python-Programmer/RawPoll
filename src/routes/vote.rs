use rocket::*;
use rocket_contrib::json;

use indexmap::IndexMap;

use mongodb::{bson::doc, bson::oid::ObjectId, sync::Database};

use serde::{Serialize, Deserialize};

#[derive(Deserialize)]
pub struct PollOption {
    id: String,
    option: String
}

#[derive(Debug, Serialize, Deserialize)]
struct PollDocument {
    types: IndexMap<String, isize>,
    ips: Vec<String>
}

#[post("/api/vote", format = "application/json", data = "<poll>")]
pub fn post(client: State<Database>, poll: json::Json<PollOption>) -> json::JsonValue {
    let poll_collection = client.collection("polls");
    
    let res = poll_collection.find_one(Some(
        doc! {
            "_id": ObjectId::with_string(poll.id.as_str()).unwrap()
        }
    ), None);

    match res {
        Ok(val) => {
            let mut result = val.unwrap();

            let mut poll_doc: PollDocument = mongodb::bson::from_bson(result.get("options").unwrap().to_owned()).unwrap();

            if !poll_doc.types.contains_key(&poll.option) {
                return json!({
                    "status": "failure",
                    "error": "Cannot find the poll option specified!"
                })
            }

            let prev_poll_val = &poll_doc.types.get(&poll.option).unwrap();
            let update_val = **prev_poll_val + 1;

            poll_doc.types.insert(String::from(&poll.option), update_val);

            let redacted_poll = mongodb::bson::to_bson(&poll_doc).unwrap();

            result.insert("options", redacted_poll);

            poll_collection.update_one(
                doc! {
                    "_id": ObjectId::with_string(poll.id.as_str()).unwrap()
                }, result, None).unwrap();

            return json!({
                "status": "success",
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