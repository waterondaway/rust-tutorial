fn main() {
    // Question #1 | Find Season
    println!("Question #1"); 
    let month: i32 = 12;
    match month {
        12 | 1 | 2 => println!("Winter"),
        3 | 4 | 5 => println!("Spring"),
        6 | 7 | 8 => println!("Summer"),
        9 | 10 | 11 => println!("Autumn"),
        _ => println!("Invalid Month")
    }
    
}
