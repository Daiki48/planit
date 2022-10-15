mod todo_model;

use todo_model::Todo;

fn main() {
    std::process::Command::new("clear").status().unwrap();

    let mut todo: Todo = Todo::new();

    todo.insert();

    println!("title is {}", &todo.title);
    println!("contents is {}", &todo.contents);
    println!("status is {}", &todo.status);

    println!("{:?}", &todo);

}
