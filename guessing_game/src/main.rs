use rand::Rng;
use std::cmp::Ordering;
use std::io;

fn main() {
    println!("Guess the number!");
    let secret_number = rand::thread_rng().gen_range(1..101);

    println!("The secret number is: {}", secret_number); //$BHkL)$N?t;z$O<!$NDL$j(B
    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");
        let guess: u32 = guess.trim().parse().expect("Please type a number!");

        println!("You guessed: {}", guess);

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),  //$B>.$5$9$.(B
            Ordering::Greater => println!("Too big!"), //$BBg$-$9$.(B
            Ordering::Equal => {
                println!("You win!"); //$B>!$A(B
                break;
            }
        }
    }
}
