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
    println!("Answer of Question #2 'Factorial Number' : {}\n\n", answer);

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

}
