use rand::Rng; // trait
use std::cmp::Ordering;
use std::io; // prelude

fn main() {
    println!("Guess number game ~");
    let secret_number = rand::thread_rng().gen_range(1, 101);

    println!("Secret number is {}", secret_number);
    println!("Guess a number！");

    let mut guess = String::new();
    io::stdin().read_line(&mut guess).expect("Can't read");

    // shadow
    let guess: u32 = guess.trim().parse().expect("Please type a number");

    println!("Your number is {}", guess);

    match guess.cmp(&secret_number) {
        Ordering::Less => println!("Too small"),
        Ordering::Greater => println!("To big"),
        Ordering::Equal => println!("You win"),
    }
}
