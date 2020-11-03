use std::net::{IpAddr, SocketAddr};

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
pub fn post(remote_addr: SocketAddr, client: State<Database>, poll: json::Json<PollOption>) -> json::JsonValue {
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
            
            let user_ip: String;

            match remote_addr.ip() {
                IpAddr::V4(ip) => {
                    user_ip = ip.to_ipv6_mapped().to_string();
                }
                IpAddr::V6(ip) => {
                    user_ip = ip.to_string();
                }
            }

            if !poll_doc.types.contains_key(&poll.option) {
                return json!({
                    "status": "failure",
                    "error": "Cannot find the poll option specified!"
                })
            }

            for ip in poll_doc.ips.iter() {
                if ip == &user_ip {
                    return json!({
                        "status": "failure",
                        "error": "You cannot vote twice on the same poll"
                    })
                }
            }

            let prev_poll_val = &poll_doc.types.get(&poll.option).unwrap();
            let update_val = **prev_poll_val + 1;

            poll_doc.ips.push(user_ip);
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