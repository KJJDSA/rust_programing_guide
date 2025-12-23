use std::io;
use rand::Rng; // Rng : 난수생성기 구현 메서드를 정의한 트레이트. 사용하기 위해서는 반드시 스코프 내에 있어야함

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new(); 

    io::stdin() 
        .read_line(&mut guess)
        .expect("Failed to read line");

    println!("You guessed: {guess}");
}
