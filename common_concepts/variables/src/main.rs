const _THREE_HOURS: u32 = 60 * 60 * 3; // note: - global scope

fn main() {
    // println!("Three hours are {_THREE_HOURS} seconds");
}

fn _lets() -> () {
    let mut x = 5;
    println!("value {x}");
    x = 8;
    println!("value {x}");
    x += 1;
    println!("value {x}");

    let y = 5;
    println!("Value {y}");

    // y = 5 - nope.
}

fn _shadowing() -> () {
    let mut x: u8 = 5;

    x = x + 1;

    {
        let x: u8 = x * 2;
        println!("Shadowing inner scope {x}"); // - 12
    }

    println!("Shadowing outer scope {x}"); // - 6

    let spaces = "   ";
    let spaces = spaces.len();

    println!("{spaces}"); // note: will print 3, can change type.
    // note: can't change type with 'mut'
}
