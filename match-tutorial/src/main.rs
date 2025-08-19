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

    // Question #2 | Finding Type of Transport .
    enum Transport {
        Car(String, i32),
        Bike(i32),
        Airplane { model: String, capacity: i32 },
        Walking,
    }
    let my_transport = Transport::Car(String::from("Tesla"), 20);
    match my_transport {
        Transport::Car(name, speed) => println!("Car {} with top speed {} km/h", name, speed),
        Transport::Bike(speed) => println!("Bike with top speed {} km/h", speed),
        Transport::Airplane {model, capacity} => println!("Airplane {} with capacity {} passengers", model, capacity),
        Transport::Walking => println!("Just walking...")
    }
    
}
