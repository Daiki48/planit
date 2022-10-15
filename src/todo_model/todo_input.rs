pub fn get_title() -> String {
    let mut title = String::new();
    std::io::stdin().read_line(&mut title).ok();
    return title.trim().parse().ok().unwrap();
}

pub fn get_contents() -> String {
    let mut contents = String::new();
    std::io::stdin().read_line(&mut contents).ok();
    return contents.trim().parse().ok().unwrap();
}


