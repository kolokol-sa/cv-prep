// Define a Book with a title, an author, a year (u32),
// and a bool for whether you've read it. Make a Vec<Book>
// with four or five entries. Write a function that takes
// &[Book] and prints only the unread ones, and one that
// returns how many were published before a given year.

// The point: a collection of structs, and the difference
// between iterating and filtering.

struct Book {
    title: String,
    author: String,
    year: u32,
    is_read: bool,
}

const GIVEN_YEAR: u32 = 1980;

fn unread_books(lib: &[Book]) {
    for b in lib.iter().filter(|b| !b.is_read) {
        println!("\"{}\" by {}, {}", b.title, b.author, b.year);
    }
}

fn books_before_year(lib: &[Book], year: u32) -> usize {
    lib.iter().filter(|b| b.year < year).count()
}

fn main() {
    let library = vec![
        Book {
            title: String::from("Dune"),
            author: String::from("Frank Herbert"),
            year: 1965,
            is_read: true,
        },
        Book {
            title: String::from("Neuromancer"),
            author: String::from("William Gibson"),
            year: 1984,
            is_read: false,
        },
        Book {
            title: String::from("The Left Hand of Darkness"),
            author: String::from("Ursula K. Le Guin"),
            year: 1969,
            is_read: true,
        },
        Book {
            title: String::from("Snow Crash"),
            author: String::from("Neal Stephenson"),
            year: 1992,
            is_read: false,
        },
        Book {
            title: String::from("Solaris"),
            author: String::from("Stanisław Lem"),
            year: 1961,
            is_read: false,
        },
    ];

    println!("Unread books:");
    unread_books(&library);

    println!(
        "There are {} books printed before {GIVEN_YEAR} in the library.",
        books_before_year(&library, GIVEN_YEAR)
    )
}
