fn main() {
    // Question #1 | Sum 1 - 10 with Loop .
    let mut count_loop: i32 = 0;
    let mut sum: i32 = 0;
    loop {
        count_loop = count_loop + 1;
        sum = sum + count_loop;
        if count_loop == 10 {
            break;
        } else {
            continue;
        }
    }
    println!("Question #1");
    println!("Answer of Question #4 'Sum 1 - 10' : {}\n\n", sum);
}
