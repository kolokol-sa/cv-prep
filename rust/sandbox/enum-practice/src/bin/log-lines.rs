// Parsing log lines is a real job where the per-line enum representation is the honest one —
// a log file genuinely is a sequence of independent records, so nothing is being forced.

// Define an enum LogLine with three variants, one of each shape:

// a unit variant for a line that couldn't be parsed
// a tuple variant carrying a message String for an info line
// a struct variant with a code: u32 and a message: String for an error line

// Write four functions, signatures yours to decide:

// Format one log line as a display string. match.
// Given a slice of log lines, find the first error and return its reference. Uses if let in a loop.
// Given a slice, produce a summary string for the first error — something like "first failure:
// code 500 — timeout". If there's no error, the function has nothing to do. let...else.
// Count how many lines failed to parse. No binding needed.

enum LogLine {
    Unparseable,
    Info(String),
    Error { code: u32, message: String },
}

fn display(line: &LogLine) {
    match line {
        LogLine::Unparseable => println!("[Failed to parse]"),
        LogLine::Info(message) => println!("[Info] {message}"),
        LogLine::Error { code, message } => println!("[Error] code: {code} - {message}"),
    }
}

fn first_error(log: &[LogLine]) -> Option<&LogLine> {
    for line in log {
        if let LogLine::Error { .. } = line {
            return Some(line);
        }
    }
    None
}

fn error_summary(error: Option<&LogLine>) -> Option<String> {
    let Some(LogLine::Error { code, message }) = error else {
        return None;
    };
    let summary = format!("error code: {code}, message: {message}");
    Some(summary)
}

fn count_unparseable(log: &[LogLine]) -> usize {
    let mut count: usize = 0;
    for line in log {
        match line {
            LogLine::Unparseable => count += 1,
            _ => (),
        }
    }
    count
}

fn main() {
    // Claude-generated dummy vector
    let lines = vec![
        LogLine::Info(String::from("server started")),
        LogLine::Unparseable,
        LogLine::Error {
            code: 500,
            message: String::from("timeout"),
        },
        LogLine::Error {
            code: 404,
            message: String::from("not found"),
        },
    ];

    // displaying the log
    for line in lines.iter() {
        display(line);
    }

    // displaying the code of the first error
    let first_error = first_error(&lines);
    match first_error {
        Some(LogLine::Error { code, .. }) => println!("The first error had code {}", code),
        _ => println!("There were no errors"),
    }

    // displaying the full first error description if there is
    let error_summary = error_summary(first_error);
    if let Some(s) = error_summary {
        println!("{s}");
    }

    // counting unparseable lines
    println!("{} line(s) failed to parse", count_unparseable(&lines));
}
