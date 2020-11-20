use std::net::{IpAddr, SocketAddr};

use rocket::*;
use rocket_contrib::json;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct PollOption {
    id: String,
    option: String
}

#[post("/api/vote", format = "application/json", data = "<poll>")]
pub fn post(remote_addr: SocketAddr, database: State<dino::Database>, poll: json::Json<PollOption>) -> json::JsonValue {
    let poll_main = database.find(poll.id.as_str());

    match poll_main {
        Ok(val) => {
            let mut value = val.to_tree();
            let mut options_tree = value.find("options").unwrap().to_tree();
            let mut poll_doc = options_tree.find("values").unwrap().to_tree();

            let ip_check: bool = value.find("settings").unwrap().to_tree().find("ip-check").unwrap().to_bool();

            if !poll_doc.contains_key(poll.option.as_str()) {
                return json!({
                    "status": "failure",
                    "error": "Cannot find the poll option specified!"
                })
            }

            let user_ip: String;

            match remote_addr.ip() {
                IpAddr::V4(ip) => {
                    user_ip = ip.to_ipv6_mapped().to_string();
                }
                IpAddr::V6(ip) => {
                    user_ip = ip.to_string();
                }
            }

            if ip_check {
                for ip in options_tree.find("ips").unwrap().to_vec().iter() {
                    if ip == &user_ip {
                        return json!({
                            "status": "failure",
                            "error": "You cannot vote twice on the same poll"
                        })
                    }
                }
            }

            let poll_cout = poll_doc.find(poll.option.as_str()).unwrap().to_number();

            poll_doc.insert_number(poll.option.as_str(), poll_cout + 1);
            options_tree.insert_tree("values", poll_doc);
            
            if ip_check {
                let mut poll_ips: Vec<String> = options_tree.find("ips").unwrap().to_vec();

                poll_ips.push(user_ip);
                
                options_tree.insert_array("ips",  poll_ips.iter().map(|s| { let s: &str = s; s }).collect());
            }

            value.insert_tree("options", options_tree);

            database.insert_tree(poll.id.as_str(), value);

            return json!({
                "status": "success"
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