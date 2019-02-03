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

static  mask_l:[u64;6]=[0xaaaaaaaaaaaaaaaa,
	0xcccccccccccccccc,
	0xf0f0f0f0f0f0f0f0,
	0xff00ff00ff00ff00,
	0xffff0000ffff0000,
	0xffffffff00000000];

static  mask_r:[u64;6]=[0x5555555555555555,
	0x3333333333333333,
	0x0f0f0f0f0f0f0f0f,
	0x00ff00ff00ff00ff,
	0x0000ffff0000ffff,
	0x00000000ffffffff];

fn real_ortho(&mut data:[u64;64]){
	for i in 0..6{
		let n:u64=(1 as u64)<<i;
		let mut j:u64=0;
		while j<64{
			for k in 0..n{
				let u:u64= data[j+k] & mask_l[i];
				let v:u64= data[j+k] & mask_r[i];
				let x:u64= data[j+n+k] & mask_l[i];
				let y:u64= data[j+n+k] & mask_r[i];
				data[j + k] = u | (x >> n);
				data[j + n + k] = (v << n) | y;
			}
			j+=2*n;
		}

	} 
}

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





