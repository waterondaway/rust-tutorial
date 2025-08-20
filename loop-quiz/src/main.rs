fn main() {
    // Question #1 | Sum 1 - 10 with Loop .
    let mut count_loop: i32 = 0;
    let mut sum: i32 = 0;
    loop {
        count_loop = count_loop + 1;
        sum = sum + count_loop;
        if count_loop == 10 {
            break;
        }
    }
    println!("Question #1");
    println!("Answer of Question #1 'Sum 1 - 10' : {}\n\n", sum);


    // Question #2 | Factorial .
    let mut factorial_number: i32 = 5;
    let mut answer: i32 = 1;
    loop {
        answer = answer * factorial_number;
        factorial_number = factorial_number - 1;
        if factorial_number == 0 {
            break;
        }
    }
    println!("Question #2");
    println!("Answer of Question #2 'Factorial Number' : {}\n", answer);

    // Question #3 | Count Down Boom! .
    println!("Question #3");
    let mut loop_count:i32 = 5;
    loop {
        println!("{}", loop_count);
        loop_count = loop_count - 1;
        if loop_count == 0 {
            println!("Boom!");
            break;
        }
    }
    println!("\n");

    // Question #4 | Print 1-10 with for loop .
    println!("Question #4");
    for i in 1..=10 {
        println!("{}", i);
    }
    println!("\n");

    // Question #5 | Summary of even number between 1 - 20 .
    println!("Question #5");
    let mut sum2:i32 = 0;
    for i in 1..=20 {
        if i%2 == 0 {
            sum2 = sum2 + i;
        }
    }
    println!("Answer of Question #5 'Summary of even number between 1 - 20' : {}\n", sum2);

    // Question #6 | Multiplication Table . 
    println!("Question #6");
    let number2: i32 = 5;
    for i in 1..=12 {
        println!("{} x {} : {}", number2, i, (number2*i));
    }

}
