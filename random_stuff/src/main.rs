mod password_gen;

fn main() {
    let pwd_gen = password_gen::password_gen();

    println!("{}", pwd_gen);
}
