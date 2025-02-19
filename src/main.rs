fn main() {
    let rand: String = "wat".to_string();
    print!("{}", test(rand));
}

fn test(arg: String) -> String {
    let mut chars: Vec<char> = arg.chars().collect();

    if chars.len() > 2 {
        chars[2] = '&';
    }

    return chars.into_iter().collect();
}
