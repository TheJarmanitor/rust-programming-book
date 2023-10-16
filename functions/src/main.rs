fn main() {
    println!("Hello, world!");

    let x = five();
    println!("The value of x is: {x}")
}

fn print_labeled_measurement(x: i32, unit: char){
    println!("The measurement is: {x}{unit}");
}

fn five() -> i32 {
    5
}