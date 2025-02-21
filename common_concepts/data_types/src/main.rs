fn main() {
    _loops();
    println!("{}", _expression());
    println!("{}", _overflow(255));
    _array([1, 2, 3, 4, 5]);
    _fizz_buzz();
}

fn _loops() -> () {
    let mut counter: u8 = 0;
    loop {
        counter += 1;
        println!("woop {counter}");

        if counter == 10 {
            break;
        }
    } // infinite loop;

    let mut counter: u8 = 0;
    let res = loop {
        counter += 1;

        if counter == 100 {
            break counter + 100; // break is similar to return in this case, semicolon is optional.
        }
    };

    println!("Counter is {res}");
}

fn _expression() -> u8 {
    let x = 3;
    x+1

    // will return the value due to mising ;
    // seems confusing, rather return...
}

fn _overflow(number: u8) -> u8 {
    return number.wrapping_add(20);
    // if it overflows it will start from 0 where it overflows and continue
}

fn _array(arr : [u8; 5]) -> () {
    println!("Please enter an array index");

    let mut index = String::new();

    std::io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Please enter a valid number");

    let array_length = arr.len();

    if index > array_length {
        println!("Index out of range, lenght of array is {array_length}");

        return;
    }

    println!("The value of the index is {}", arr[index]);
}

fn _fizz_buzz() -> () {
    let array: Vec<(i8, &str)> = vec![
        (3, "Fizz"),
        (5, "Buzz"),
        (7, "Bazz"),
        (9, "Bezz"),
        (12, "Bozz"),
        (12, "Bozz"),
    ];

    for i in 1..=100 {
        let mut result = String::new();

        for &(number, word) in &array {
            if i % number == 0 {
                result.push_str(word);
            }
        }

        if result.is_empty() {
            println!("{}", i);
        } else {
            println!("{}", result);
        }

    }
}

/*
* will default to i32 if not assigned
*
* Length	Signed	Unsigned
* 8-bit	    i8	    u8
* 16-bit	i16	    u16
* 32-bit	i32	    u32
* 64-bit	i64	    u64
* 128-bit	i128	u128
* arch	    isize	    usize
*
* signed == can be negative
*/


/*
* Number literals	Example
* Decimal	        98_222
* Hex	            0xff
* Octal	            0o77
* Binary	        0b1111_0000
* Byte (u8 only)	b'A'
 */
