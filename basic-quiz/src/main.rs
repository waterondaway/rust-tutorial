fn reverse_name(&name: String) -> String {
    println!("{}", &name);
}

fn main() {
    let name = String::from("Nonpawit");
    reverse_name(name);
    println!("{}", name);
}
