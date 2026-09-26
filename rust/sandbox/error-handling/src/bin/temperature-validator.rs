// A sensor writes lines to a file, one reading per line, each in the form:

// sensor-a,21.5
// sensor-b,-4.0
// sensor-c,300.0

// Write a program that reads such a file and reports the average temperature.

// The pieces:

// - A Reading type with a sensor name and a temperature. Temperatures outside -80.0 to 60.0 are physically
// implausible and must be rejected — use the validating-constructor pattern from 9.3, with the field private.
// - A function that parses one line into a Reading. It can fail three ways: no comma, unparseable number,
// or temperature out of range. Return Result.
// - A function that reads the file and returns all readings, using ?. Decide what should happen when one line
// is bad — skip it, or fail the whole operation. Either is defensible; pick one and be ready to say why.
// - main returns Result and uses ?.

// Decisions to make deliberately:

// - What error type each function returns. You don't have custom error enums yet, so String or Box<dyn Error> —
// and the answer may differ between the parsing function and main.
// - Whether the validating constructor returns Result or Option.

// Create a small test file by hand, including at least one bad line so you can see your handling work.

use std::{fs::File, io::Read};
use std::error::Error;
pub struct Reading {
    sensor: String,
    temp: f64,
}

impl Reading {
    pub fn new(sensor: String, temp: f64) -> Result<Reading, String> {
        if temp < -80.0 || temp > 60.0 {
            Err(format!("Temperature {temp} is out of range (expected from -80 to +60)."))
        } else {
            Ok(Reading { sensor, temp })
        }
    }
}

fn line_to_reading(line: &str) -> Result<Reading, String> {
    let (sensor, temp_str) = line.split_once(',').ok_or("The line doesn't have a comma.".to_string())?;
    let temp: f64 = temp_str.parse().map_err(|_| "The temperature can't be parsed.".to_string())?;
    Reading::new(sensor.to_string(), temp)
}

fn file_to_readings(mut file: File) -> Result<Vec<Reading>, String> {
    let mut log = String::new();
    file.read_to_string(&mut log).map_err(|e| format!("The file read is unsuccessful: {e}"))?;
    let mut readings = Vec::new();
    for line in log.lines() {
        let reading = match line_to_reading(line) {
            Err(e) => {eprintln!("Skipping line: {e}"); continue;},
            Ok(x) => {eprintln!("Successful read"); x},
        };
        readings.push(reading);
    }
    Ok(readings)
}

fn main() -> Result<(), Box<dyn Error>> {
    
    let f = File::open("log.txt")?;
    let readings = file_to_readings(f)?;

    let temp_sum: f64 = readings.iter().map(|r| r.temp).sum();
    println!("The average temperature is {:.2} degrees", temp_sum / readings.len() as f64);

    Ok(())    
}