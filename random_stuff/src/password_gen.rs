use rand::Rng;

pub fn password_gen() -> String {
    let mut letters = String::new();
    let mut numbers = String::new();
    let mut symbols = String::new();

    println!("Enter number of letters: ");
    std::io::stdin()
        .read_line(&mut letters)
        .expect("Number out of range");

    println!("Enter number of numbers: ");
    std::io::stdin()
        .read_line(&mut numbers)
        .expect("Number out of range");

    println!("Enter number of symbols: ");
    std::io::stdin()
        .read_line(&mut symbols)
        .expect("Number out of range");

    let mut password = String::new();

    password.push_str(&_get_random_letters(letters.trim().parse().expect("Not a number")));
    password.push_str(&_get_random_numbers(numbers.trim().parse().expect("Not a number")));
    password.push_str(&_get_random_symbols(symbols.trim().parse().expect("Not a number")));

    let chars: Vec<char> = password.chars().collect();

    return shuffle_chars(chars);
}

fn shuffle_chars(mut chars: Vec<char>) -> String {
    let mut random = rand::rng();

    for i in 0..chars.len() {
        let random_i = random.random_range(0..chars.len());

        let temp = chars[i];
        chars[i] = chars[random_i];
        chars[random_i] = temp;
    }

    return chars.iter().collect();
}

fn _get_random_numbers(number_of_numbers: u32) -> String {
    let mut random = rand::rng();
    let mut letters = String::new();

    for _i in 1..number_of_numbers {
        let chance = random.random_range(1..10);
        let random_char: u32;

        if chance > 5 {
            random_char = random.random_range(65..90);
        } else {
            random_char = random.random_range(97..122);
        }

        letters.push(std::char::from_u32(random_char).unwrap());
    }

    return letters;
}

fn _get_random_letters(number_of_letters: u32) -> String {
    let mut random = rand::rng();
    let mut numbers = String::new();

    for _i in 1..number_of_letters {
        let random_number = random.random_range(48..57);
        numbers.push(std::char::from_u32(random_number).unwrap());
    }

    return numbers;
}

fn _get_random_symbols(number_of_symbols: u32) -> String {
    let mut random = rand::rng();
    let mut symbols = String::new();

    for _i in 1..number_of_symbols{
        let random_symbol = random.random_range(33..43);
        symbols.push(std::char::from_u32(random_symbol).unwrap());
    }

    return symbols;
}
