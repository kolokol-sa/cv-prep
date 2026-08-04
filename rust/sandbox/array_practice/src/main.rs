// I have an array of the amount of traffic for a week
// I want to count the sum for the first 3 days

fn main() {
    
    let weekly_traffic: [u32; 7] = [6, 10, 100, 187, 42, 101, 96];

    println!("I have traffic data for {} days", weekly_traffic.len());

    let sum_3 = weekly_traffic[0] + weekly_traffic[1] + weekly_traffic[2];
    println!("Total traffic for the first 3 days is: {} GB", sum_3);

}
