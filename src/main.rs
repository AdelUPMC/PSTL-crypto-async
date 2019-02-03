/*use ferris_says::say;
use std::io::{stdout, BufWriter};

fn main() {
    let stdout = stdout();
    let out = b"Hello fellow Rustaceans!";
    let width = 24;

    let mut writer = BufWriter::new(stdout.lock());
    say(out, width, &mut writer).unwrap();
}
*/
use std::io;
use rand::prelude::*;

fn main() {
    println!("Guess the number!");
	let mut rng= rand::thread_rng();
	let mut n:f64=rng.gen();
	n=n*(100 as f64);


    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin().read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {}", guess);
	println!("generated number: {}",(n as u64));
}