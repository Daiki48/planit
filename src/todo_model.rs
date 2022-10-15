mod todo_input;

use todo_input::{
    get_title,
    get_contents
};

use serde::{
    Serialize,
    Deserialize,
};
use std::fs::OpenOptions;

#[derive(Serialize,Deserialize,Debug)]
pub struct Todo {
    pub title: String,
    pub contents: String,
    pub status: bool,
}

impl Todo {
    pub fn new() -> Self {
        Todo {
            title: "".to_string(),
            contents: "".to_string(),
            status: true,
        }
    }

    pub fn insert(&mut self) {
        println!("Enter the title of the TODO.");
        let todo_title = get_title();
        self.title.push_str(&todo_title);
        println!("{}", &mut self.title);

        println!("Enter the contents of the TODO.");
        let todo_contents = get_contents();
        self.contents.push_str(&todo_contents);
        println!("{}", &mut self.contents);
    }

    pub fn save(self) -> Result<(), Box<dyn std::error::Error>> {
        let list = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open("./data/db.json")?;

        serde_json::to_writer_pretty(list, &self)?;
        Ok(())
    }
}


