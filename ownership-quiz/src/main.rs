fn main() {
    // Question #1 | Ownership Move .
    /*
        If you sending value to another variable the ownership are move to new variable .
        You need to solve by sending clone value to them .
    */
    println!("Question #1");
    let name = String::from("Nonpawit");
    fn take_ownership(s: String) {
        println!("Function Scope Value: {}", s);
    }
    take_ownership(name.clone());
    println!("Global Scope Value : {}", name);
    println!("\n");

    // Question #2 | Borrowing with Reference .
    println!("Question #2");
    fn print_string_length(string: &String) {
        println!("{}", string.len());
    }
    let name2 = String::from("Silabumrungrad");
    print_string_length(&name2);
    print_string_length(&name2);
    print_string_length(&name2);
    print_string_length(&name2);
    println!("'name2' value : {}\n", name2);

    // Question #3 | Mutable Borrow .
    println!("Question #3");
    fn add_suffix(s : &mut String){
        s.push_str("!!!");
    }

    let mut text = String::from("Hello");
    add_suffix(&mut text);
    println!("{}", text);
}
