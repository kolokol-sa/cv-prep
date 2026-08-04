use std::io;

fn main() {
    
    println!("Enter temperature in degrees Celsius");

    let mut temp_c = String::new();

    io::stdin()
        .read_line(&mut temp_c)
        .expect("Failed to read line");

    let temp_c: f64 = temp_c.trim().parse().expect("Try inputting a number next time.");

    let temp_f = temp_c * 1.8 + 32.0;

    println!("{} degrees Celsius equal {:.4} degrees Fahrenheit", temp_c, temp_f);
}
