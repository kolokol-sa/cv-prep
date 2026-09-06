// Define a Reading with a sensor name (String), a temperature (f64),
// and an hour (u32, 0–23). Build a day's worth. Write functions
// that return the highest reading of the day, the average temperature,
// and all readings above a threshold.

// The point: returning the highest reading means returning a &Reading —
// which brings chapter 4's rules straight back. Watch what the signatures have to be.

// indoor — 19.2, 19.8, 20.5, 21.3, 22.0, 21.6 at hours 6, 8, 10, 12, 14, 16
// outdoor — 8.4, 11.7, 16.2, 19.8, 21.4, 18.1 at hours 6, 8, 10, 12, 14, 16

struct Reading {
    sensor_name: String,
    temperature: f64,
    hour: u32,
}

const THRESHOLD: f64 = 20.0;

fn highest_reading(day_temp: &[Reading]) -> &Reading {
    let mut max = day_temp[0].temperature;
    let mut idx = 0;
    for i in 0..day_temp.len() {
        if day_temp[i].temperature > max {
            max = day_temp[i].temperature;
            idx = i;
        }
    }
    &day_temp[idx]
}

fn average_temp(day_temp: &[Reading], sensor: &str) -> f64 {
    let mut sum = 0.0;
    let mut count = 0.0;
    for r in day_temp {
        if r.sensor_name == sensor {
            sum += r.temperature;
            count += 1.0;
        }
    }
    sum / count
}

fn readings_above(day_temp: &[Reading], thres: f64) -> Vec<&Reading> {
    let mut high_readings = vec![];
    for r in day_temp {
        if r.temperature > thres {
            high_readings.push(r);
        }
    }
    high_readings
}

fn main() {
    
    let raw = [
        ("indoor", 19.2, 6),
        ("indoor", 19.8, 8),
        ("indoor", 20.5, 10),
        ("indoor", 21.3, 12),
        ("indoor", 22.0, 14),
        ("indoor", 21.6, 16),
        ("outdoor", 8.4, 6),
        ("outdoor", 11.7, 8),
        ("outdoor", 16.2, 10),
        ("outdoor", 19.8, 12),
        ("outdoor", 21.4, 14),
        ("outdoor", 18.1, 16),
    ];
    
    let mut temperature_today = Vec::new();
    for (sensor, temp, hour) in raw {
        temperature_today.push(Reading {
            sensor_name: String::from(sensor),
            temperature: temp,
            hour,
        });
    }

    // for r in temperature_today {
    //     println!("Sensor \"{}\" at {} measured {} degrees", r.sensor_name, r.hour, r.temperature);
    // }

    let highest = highest_reading(&temperature_today);
    println!(
        "Highest measure was on sensor \"{}\", at {}:00 it measured {} degrees",
        highest.sensor_name, highest.hour, highest.temperature
    );

    let average_outdoor = average_temp(&temperature_today, "outdoor");
    println!(
        "Average outdoor temperature was {:.2} degrees",
        average_outdoor
    );

    let high_readings = readings_above(&temperature_today, THRESHOLD);
    println!("The readings above {:.1} degrees are:", THRESHOLD);
    println!("| Sensor | Time | Temperature |");
    for r in high_readings {
        println!(
            "| {} | {}:00 | {:.1} |",
            r.sensor_name, r.hour, r.temperature
        );
    }
}
