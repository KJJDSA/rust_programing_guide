use rand::Rng;
use std::cmp::Ordering;
use std::io; // Rng : 난수생성기 구현 메서드를 정의한 트레이트. 사용하기 위해서는 반드시 스코프 내에 있어야함

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    loop {
        println!("Please input your guess.");

        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line");

        // parse 의 Result 결과물을 바꿀 때, expect로 무조건 프로그램을 끝내는 것이 아니라 계속 진행할 수 있도록 에러처리 하기
        // let guess: u32 = guess.trim().parse().expect("Please type a number!");
        let guess: u32 = match guess.trim().parse() {
            // match 는 parse의 Result 를 받을 것이므로, Result의 구성인 두 열거형을 추가하면 분기할 수 있다.
            Ok(number) => number,
            Err(_) => continue, // _ === 포괄값(catch-all). Err에 담긴 어떤 값이든 매칭 할 수 있게된다. 
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
