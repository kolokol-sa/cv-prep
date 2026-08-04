const INDEX: u32 = 100;

fn main() {
    
    let mut number: u128 = 1;
    if INDEX > 2 {
        let (mut prev, mut curr) = (1, 1);
        for _ in 3..=INDEX {
            let next = prev + curr;
            prev = curr;
            curr = next;
        }
        number = curr;
    }
    println!("The Fibonacci number with index {INDEX} is {number}");

}
