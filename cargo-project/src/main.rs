fn main() {
    // 'println!' is calling macro. similar like a function
    println!("Hello, world!");
    println!("Nonpawit");

    /*
        Multiline Comments
    */

    // Single line Comments

    let name = "Nonpawit";
    println!("My first name is: {}", name); // {} is a placeholder to show variable values

    /*
        # Compile error because default of variable is immutable, it can not be change after declare
        let x = 5;
        x = 10;
    */

    let mut x = 5;
    println!("Before X: {}", x);
    x = 10;
    println!("After X: {}", x);

    /*
        Variable in rust do not need to be declared with a specified type
    */
    // if this is intentional, prefix it with an underscore: `_my_num`
    let _my_num = 5; // my_num is be integer
    let _my_double = 5.99; // my_double is be float
    let _my_letter = 'D'; // my_letter is character
    let _my_bool = true; // my_bool is boolean
    let _my_text = "Hello"; // my_text is string
    

    /*
        But you can also declared with type a value should be
    */
    let _my_num: i32 = 5; // integer 
    let _my_double: f64 = 5.99; // float
    let _my_letter: char = 'D'; // character
    let _my_bool: bool = true; // boolean
    let _my_text: &str = "Hello"; // string
    
    /*
        Rust Constants
        Constant variables are used to store values that never change.
        constants must be defined with a type (e.g. i32 or char).
    */
    // is that it is considered good practice to declare them with uppercase.
    const _BIRTHYEAR: i32 = 1980;


    // Operators

    /*
        Arithmetic Operators
    */

    let add = 5 + 3;
    let sub = 10 - 4;
    let mul = 6 * 2;
    let div = 12 / 3;
    let rem = 10 % 3;

    println!("Add: {}", add);
    println!("Sub: {}", sub);
    println!("Mul: {}", mul);
    println!("Div: {}", div);
    println!("Rem: {}", rem);


    /*
        Assignment Operators
    */
    let mut ao: i32 = 5; 
    println!("Start: {}", ao);
    ao += 5;
    println!("After += 5: {}", ao);
    ao -= 2;
    println!("After -= 2: {}", ao);
    ao *= 2;
    println!("After *= 2: {}", ao);
    ao /= 2;
    println!("After /= 2: {}", ao);
    ao %= 2;
    println!("After %= 2: {}", ao);

    /*
        Comparison Operators
    */
    let val1 = 5;
    let val2 = 10;

    println!("5 == 10 : {}", val1 == val2);
    println!("5 != 10 : {}", val1 != val2);
    println!("5 < 10 : {}", val1 < val2);
    println!("5 >= 10 : {}", val1 >= val2);


    /*
        Logical Operators
    */
    let mut bool1: bool = true;
    let mut bool2: bool = true;

    println!("Logical Operator: {}", bool1 && bool2);


    /*
        Basic If Else
    */
    if true {
        bool1 = false;
    } else if false {
        bool2 = true;
    } else {
        bool1 = true;
    }
    /*
        if else with expression
    */
    let name = if bool1 {
        "Nonpawit"
    } else if bool2 {
        "Silabumrungrad"
    } else {
        "Haha"
    };
    println!("{}", name);


    /*
        match
    */

    let day = 10;

    //  single matches
    match day {
        1 => println!("Monday"),
        2 => println!("Tuesday"),
        3 => println!("Wednesday"),
        4 => println!("Thursday"),
        5 => println!("Friday"),
        6 => println!("Saturday"),
        7 => println!("Sunday"),
        _ => println!("Invalid day."),
    }

    // multiple matches
    match day {
        1 | 10 => println!("Weekday"),
        2 | 15 => println!("Nonpawit"),
        _ => println!("Nonpawit Silabumrungrad")
    }

    // Basic Enumeration Variable 
    enum OrderStatus {
        Pending,
        Success
    }

    let order_status = OrderStatus::Pending;
    let _order_status2 = OrderStatus::Success;
    match order_status {
        OrderStatus::Success => println!("Success"),
        OrderStatus::Pending => println!("Pending")
    }

    // Rust Advance Enumeration Variable
    enum Message {
        Quit,
        Move { x: i32, y: i32}
    }

    let _msg2 = Message::Quit;
    let msg1 = Message::Move { x:10, y:20 };

    match msg1 {
        Message::Quit => println!("Quit"),
        Message::Move { x, y } => println!("Move to ({}, {})", x, y),
    }


    /*
        Loop 
    */

    // Unlimited Loop 
    loop {
        println!("This will repeat forever!");
        break;
    }

    /*
        Loop Count
    */

    let mut count_loop = 0;
    loop {
        println!("Count: {}", count_loop);

        if count_loop == 3 {
            break;
        }
        count_loop += 1; 
    }

    /*
        Loop with return a value
    */
    let mut count = 0;
    let _result = loop {
        println!("Hello");
        if count == 3 {
            break count;
        }
        count += 1;
    };

    // Tip !
    // Expression requires ; after end of expression
    // Statement didn't require ; after end of statement


    let mut count = 1;
    while count <= 5 {
        count += 1;
        println!("{}", count);
    }

    // Rust For Loop
    // not include the last number
    for i in 1..6 {
        println!("i is: {}", i);
    }

    // include the last number (Inclusive Range)
    for i in 1..=6 {
        println!("i is: {}", i);
    }


    // Creating a Function

    /*
    fn function_name() {
        // code to be executed; 
    }
    */
    
    // Function with not return values

    fn say_hello() {
        println!("Hello from a function!");
    }
    
    say_hello();

    fn greet(name: &str) {
        println!("Hello, {}!", name);
    }
    greet("Nonpawit Silabumrungrad");


    // Function with return values

    fn abc(a: i32, b: i32) -> i32 {
        return a + b;
    }
    println!("{}", abc(3, 4));

    // function omit the return values; 
    fn _xyz(a: i32, b: i32) -> i32 {
        a - b
    }

    // String in Rust
    let _text1 = "Hello World".to_string();
    let mut text2 = String::from("Hello");
    text2.push_str(" World");
    println!("{}", text2);

    // Concatenate Strings
    let s1 = String::from("Nonpawit");
    let s2 = String::from("Silabumrungrad");

    let result = format!("{} {}", s1, s2);
    println!("{}", result);

    // String Length
    let name = String::from("John");
    println!("Length: {}", name.len());

    // But simple types like numbers, characters and booleans are copied, not moved.
    
    // Ownership (Only String Character & Boolean can do this way)
    let a = 5;
    let b = a;
    println!("{} {}", a,b);

    // Keep a to be Owner ship
    let _a = String::from("Hello");
    let _b = a.clone();
    println!("{}", _a);


    // Borrowing
    let a = String::from("Hello");
    let b = &a;

    println!("a = {}", a);
    println!("b = {}", b);

    // Borrowing & want to change values

    let mut name = String::from("John");
    let name_ref = &mut name;
    name_ref.push_str(" Smith");

}

