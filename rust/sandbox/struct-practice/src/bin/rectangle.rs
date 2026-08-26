// Define a Rectangle with width and height (f64).
// Write a function that takes a &Rectangle and returns
// its area, and one that returns its perimeter. In main,
// make two rectangles and print both values for each.

// The point: one thing passed around instead of two loose numbers.

struct Rectangle {
    width: f64,
    height: f64,
}

fn area(rect: &Rectangle) -> f64 {
    rect.width * rect.height
}

fn perimeter(rect: &Rectangle) -> f64 {
    2.0 * (rect.width + rect.height)
}

fn main() {
    let a = Rectangle {
        width: 5.0,
        height: 4.0,
    };
    let b = Rectangle {
        width: 7.0,
        height: 9.0,
    };
    println!(
        "Two rectangles given: A, {}x{}, and B, {}x{}",
        a.width, a.height, b.width, b.height
    );
    println!("A: area is {}, perimeter is {}", area(&a), perimeter(&a));
    println!("B: area is {}, perimeter is {}", area(&b), perimeter(&b));
}
