mod todo_input;

use todo_input::{
    get_title,
    get_contents
};

#[derive(Debug)]
struct Todo {
    title: String,
    contents: String,
}

impl Todo {
    fn new() -> Self {
        Todo {
            title: "".to_string(),
            contents: "".to_string(),
        }
    }

    fn insert(&mut self) {
        println!("Enter the title of the TODO.");
        let todo_title = get_title();
        self.title.push_str(&todo_title);
        println!("{}", &mut self.title);

        println!("Enter the contents of the TODO.");
        let todo_contents = get_contents();
        self.contents.push_str(&todo_contents);
        println!("{}", &mut self.contents);
    }
}

fn main() {
    std::process::Command::new("clear").status().unwrap();

    let mut todo: Todo = Todo::new();

    todo.insert();

    println!("title is {}", &todo.title);
    println!("contents is {}", &todo.contents);

    println!("{:?}", &todo);

}
