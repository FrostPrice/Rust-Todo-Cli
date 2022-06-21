use std::collections::HashMap;
// use std::io::Read;
// use std::str::FromStr;

fn main() {
    let action = std::env::args().nth(1).expect("Please specify an action");
    let item = std::env::args().nth(2).expect("Pelase specify an item");

    // Print the Action and Item to the console
    // println!("{:?}, {:?}", action, item);

    let mut todo = Todo::new().expect("Initialization of db failed");

    if action == "add" {
        todo.insert(item);
        match todo.save() {
            Ok(_) => println!("Todo saved"),
            Err(why) => println!("An error ocurred: {}", why), 
        }
    } else if action == "complete" {
        match todo.complete(&item) {
            None => println!("'{}' is not present in the list", item),
            Some (_) => match todo.save() {
                Ok(_) => println!("Todo saved"),
                Err(why) => println!("An error occured: {}", why),
            }
        }
    }
}

struct Todo {
    // use rust built in HashMap to store key - val pairs
    map: HashMap<String, bool>,
}

impl Todo {
    fn new() -> Result<Todo, std::io::Error> {
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .read(true)
            .open("db.json")?;
            // .open("db.txt")?;

        // let mut content = String::new();
        // f.read_to_string(&mut content)?;
        // let map: HashMap<String, bool> = content
        //     .lines()
        //     .map(|line| line.splitn(2, "\t").collect::<Vec<&str>>())
        //     .map(|v| (v[0], v[1]))
        //     .map(| (k, v) | (String::from(k), bool::from_str(v).unwrap()))
        //     .collect();
        // Ok(Todo { map })

        match serde_json::from_reader(f) {
            Ok(map) => Ok(Todo {map}),
            Err(e) if e.is_eof() => Ok(Todo {
                map: HashMap::new(),
            }),
            Err(e) => panic!("An error occured: {}", e),
        }
    }

    fn insert(&mut self, key: String) {
        // insert a new item into our map.
        // we pass true as value
        self.map.insert(key, true);
    }
    
    fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        // let mut content = String::new();
        // for (k, v) in self.map {
        //     let record = format!("{}\t{}\n", k, v);
        //     content.push_str(&record);
        // }
        // std::fs::write("db.txt", content)

        // open db.json
        let f = std::fs::OpenOptions::new()
            .write(true)
            .create(true)
            .open("db.json")?;
        // write to file with serde
        serde_json::to_writer_pretty(f,&self.map)?;
        Ok(())
    }

    fn complete(&mut self, key: &String) -> Option<()> {
        match self.map.get_mut (key) {
            Some(v) => Some(*v = false),
            None => None,
        }
    }
}