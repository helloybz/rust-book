use std::{cmp::Ordering, io}; // 얘는 모듈
use rand::Rng; // 얘는 trait

fn main () {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);
    println!("The secret number is {secret_number}");

    loop {

        println!("Please input your guess.");

        let mut guess = String::new();
        // String은 타입.
        // new는 String에 구현된 function
        // 그래서 String::new 라고 표기.

        io::stdin() // <- returns std::io::Stdin 타입
            .read_line(&mut guess) // &는 argument가 reference임을 나타낸다. 데이터의 copy를 만들 필요가 없고, 참조하는 모든 부분에서 하나의 같은 데이터만 읽도록한다. Result 인스턴스를 반환한다.
            .expect("Failed to read line");

        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };
        println!("You guessed: {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big!"),
            Ordering::Equal => {
                println!("You win!");
                break;
            }
        }
    }
}