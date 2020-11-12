use rocket::*;
use rocket_contrib::json;
use rocket::State;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Poll {
    question: String,
    description: String,
    options: Vec<String>
}

#[post("/api/poll", format = "application/json", data = "<poll>")]
pub fn post(database: State<dino::Database>, poll: json::Json<Poll>) -> json::JsonValue {
    let mut poll_main = dino::Tree::new();
    let poll_id = uuid::Uuid::new_v4().to_string();

    poll_main.insert("question", poll.question.as_str());
    poll_main.insert("description", poll.description.as_str());

    let mut poll_options = dino::Tree::new();
    let mut poll_options_values = dino::Tree::new();

    for choice in &poll.options {
        poll_options_values.insert_number(choice, 1);
    };

    poll_options.insert_tree("values", poll_options_values);
    poll_options.insert_array("ips", vec![]);

    poll_main.insert_tree("options", poll_options);

    database.insert_tree(poll_id.as_str(), poll_main);

    return json!({
        "status": "success",
        "id": poll_id
    });
}

#[get("/api/poll/<id>")]
pub fn get(database: State<dino::Database>, id: String) -> json::JsonValue {
    let poll_main = database.find(id.as_str());

    match poll_main {
        Ok(val) => {
            let result = val.to_tree();

            return json!({
                "question": result.find("question").unwrap().to_string(),
                "description": result.find("description").unwrap().to_string(),
                "options": result.find("options").unwrap().to_tree().find("values").unwrap().to_json()
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