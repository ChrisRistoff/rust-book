fn main() {
    let mut whater: i8 = 127;

    whater += 1;

    println!("{}", whater);
}

fn _fizz_buzz() -> () {
    let array: Vec<(i8, &str)> = vec![
        (3, "Fizz"),
        (5, "Buzz"),
        (7, "Bazz"),
        (9, "Bezz"),
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
