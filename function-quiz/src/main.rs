fn main() {
    // Question #1 | Function to find area of Rectangle .
    println!("Question #1");
    fn find_area_rectangle(width: f64, height: f64) -> f64 {
        width * height
    }
    let input_width: f64 = 50.0;
    let input_height: f64 = 2.0;
    let area_rectangle: f64 = find_area_rectangle(input_width, input_height);
    println!("Answer of Question #1 'Function to find area of Rectangle' : {}\n", area_rectangle);

    // Question #2 | Function to check is even number . 
    println!("Question #2");
    fn is_even_number(number: i32) -> bool {
        if number % 2 == 0 {
            true
        } else {
            false
        }
    }
    let answer_of_function: bool = is_even_number(11);
    println!("Answer of Question #2 'Function to check is even number' : {}\n", answer_of_function);
}
