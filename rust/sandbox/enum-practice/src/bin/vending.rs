// A vending machine holds slots. Each slot is either
// empty, or holds a product with a name and a price
// in cents.
//
// Model a Slot as an enum.
//
// Write:
//   - a function taking a list of slots that returns
//     how many are occupied
//   - a function taking a list of slots and a budget
//     in cents, returning the names of everything
//     affordable
//   - a function taking a list of slots and a slot
//     number, returning the price of what's in it,
//     or nothing if the slot is empty or the number
//     is out of range
//
// Decide the signatures yourself.
//
// In main: build six or seven slots with two or three
// empty, then exercise all three functions, including
// asking for an empty slot and an out-of-range one.

// Salted crisps — 150
// empty
// Chocolate bar — 220
// Sparkling water — 95
// empty
// Energy drink — 310
// empty

type Name = String;
type Price = u32;

enum Slot {
    Empty,
    Product(Name, Price),
}

fn count_occupied(list: &[Slot]) -> usize {
    let mut count= 0;
    for slot in list {
        match slot {
            Slot::Product(_, _) => count += 1,
            Slot::Empty => {},
        }
    }
    count
}

fn can_afford(list: &[Slot], budget: u32) -> Vec<&Name> {
    let mut affordable = vec![];
    for slot in list {
        match slot {
            Slot::Product(name, price) => {
                if *price < budget {
                    affordable.push(name);
                }
            }
            _ => {}
        }
    }
    affordable
}

fn what_inside(list: &[Slot], num: usize) -> Option<Price> {
    if (num <= list.len()) && (num > 0) {
        match list[num - 1] {
            Slot::Product(_, price) => Some(price),
            _ => None
        }
    } else { None }
}

const SLOTS_TO_CHECK: [usize; 3] = [3, 1, 10];
const BUDGET: u32 = 200;

fn main() {

    let raw = [
        ("Salted crisps", 150),
        ("", 0),
        ("Chocolate bar", 220),
        ("Sparkling water", 95),
        ("", 0),
        ("Energy drink", 310),
        ("", 0),
    ];

    // build list of slots
    let mut slot_list: Vec<Slot> = vec![];

    for (name, price) in raw {
        slot_list.push(match name {
            "" => Slot::Empty,
            _ => Slot::Product(String::from(name), price),
        });
    }
    
    // print all slots
    for i in 0..slot_list.len() {
        match &slot_list[i] {
            Slot::Empty => println!("Slot {}: empty", i + 1),
            Slot::Product(name,price ) => println!("Slot {}: {name} - {price} cents", i + 1)
        }
        
    }

    // counting occupied slots
    println!("{} out of {} slots are occupied", count_occupied(&slot_list), slot_list.len());

    // printing list of items within BUDGET
    let affordable_items = can_afford(&slot_list, BUDGET);
    println!("With {BUDGET} cents you can buy:");
    for item in affordable_items {
        println!(" - {item}")
    }

    // testing some slots
    println!("Testing these slots: {:?}", SLOTS_TO_CHECK);
    for num in SLOTS_TO_CHECK {
        match what_inside(&slot_list, num) {
            Some(price) => println!("Item in slot {num} costs {price} cents"),
            None => println!("Slot {num}: item not found")
        }
    }
    
    
}