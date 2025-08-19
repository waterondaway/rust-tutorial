fn main() {
    // Question #1 | Find area of Rectangle .
    let width: f64 = 10.0;
    let height: f64 = 10.0;
    let area = width * height;
    println!("Question #1");
    println!("Width : {} Height : {}", width, height);
    println!("Answer of Question #1 'Rectangle Area' : {}\n\n", area);


    // Question #2 | Even or Odd .
    let number: i32 = 100;
    let answer: &str = if number%2 == 0 { "even" } else { "odd" };
    println!("Question #2");
    println!("Number : {}", number);
    println!("Answer of Question #2 'Even or Odd' : {}\n\n", answer);

    // Question #3 | Finding Max Values .
    let a: i32 = 56;
    let b: i32 = 64;
    let max: i32 = if a > b { a } else { b }; 
    println!("Question #3");
    println!("A Value : {} B Value : {}", a, b);
    println!("Answer of Question #3 'Finding Max Values' : {}\n\n", max);

    // Question #4 | Grade Calculate .
    let score: f64 = 83.1;
    let grade: char = if score > 80.0 { 'A' } else if score >= 70.0 { 'B' } else if score >= 60.0 { 'C' } else if score >= 50.0 { 'D' } else { 'F' };
    println!("Question #4");
    println!("Score : {}", score);
    println!("Answer of Question #4 'Grade Calculate' : {}\n\n", grade);

    // Question #5 | Block Expression .
    let result: i32 = {
        let x: i32 = 10;
        let y: i32 = 15;
        x + y
    };
    println!("Question #5");
    println!("Answer of Question #5 'Block Expression' : {}\n\n", result);
}
