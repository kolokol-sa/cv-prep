fn main() {
    
    let point = (0, 10);

    match point {
        (0, 0) => println!("The point is the origin"),
        (0, y) => println!("The point is on the Y axis at height Y = {}", y),
        (x, 0) => println!("The point is on the X axis at position X = {}", x),
        (x, y) => println!("The point is not on the axes: ({}, {})", x, y),
    }
}
