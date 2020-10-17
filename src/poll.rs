use rocket::State;
use indexmap::IndexMap;

fn parse(options: String) -> IndexMap<String, isize> {
    let mut map = IndexMap::new();

    for opt in options.split(",") {
        let value: Vec<&str> = opt.split(":").collect();

        map.insert(
            value[0].trim().parse::<String>().unwrap(),
            value[1].trim().parse::<isize>().unwrap(),
        );
    }

    return map;
}

pub struct Poll {
    pub title: String,
    pub description: String,
    pub options: IndexMap<String, isize>,
}

impl Poll {
    pub fn from(database: &State<sled::Db>, id: &String) -> Poll {
        let tree = database.open_tree(id).unwrap();

        let title = String::from_utf8(tree.get("title").unwrap().unwrap().to_vec());
        let description = String::from_utf8(tree.get("description").unwrap().unwrap().to_vec());

        let options_string = String::from_utf8(tree.get("options").unwrap().unwrap().to_vec());
        let options_parsed = parse(options_string.unwrap());

        return Poll {
            title: title.unwrap(),
            description: description.unwrap(),
            options: options_parsed,
        };
    }

    pub fn tree(database: &State<sled::Db>, id: &String) -> sled::Tree {
        return database.open_tree(id).unwrap();
    }

    pub fn to(result: IndexMap<String, isize>) -> String {
        let mut res = String::new();
        let mut i = 0;
        
        for val in &result {
            if i == result.len() - 1 {
                res += format!("{}: {}", val.0.as_str(), val.1.to_string()).as_str();
            }

            else {
                res += format!("{}: {}, ", val.0.as_str(), val.1.to_string()).as_str();
            }

            i += 1;
        }

        return res;
    }
}
