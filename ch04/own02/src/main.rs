fn main() {
    let name = vec![String::from("Ferris")];
    let first = &name[0];
    stringify_name_with_title(&name);
    println!("{}", first);
}

fn stringify_name_with_title(name: &Vec<String>) -> String {
    let mut full = name.join(" ");
    full.push_str("Esq.");
    full
}
