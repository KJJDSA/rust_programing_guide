use rand::Rng;
use std::cmp::Ordering;
use std::io; // Rng : 난수생성기 구현 메서드를 정의한 트레이트. 사용하기 위해서는 반드시 스코프 내에 있어야함

fn main() {
    println!("Guess the number!");

    let secret_number = rand::thread_rng().gen_range(1..=100);

    println!("The secret number is: {secret_number}");

    println!("Please input your guess.");

    let mut guess = String::new();

    io::stdin()
        .read_line(&mut guess)
        .expect("Failed to read line");

    // 섀도잉 : guess라는 변수 이름을 타입만 바꿔 재사용하고 싶을 때 자주 사용되는 기능. 이전에 있던 guess값을 새로운 값으로 가리는(shadow) 것
    let guess: u32/* 32비트 부호없는 정수(양수) = 작은 양수 */ = guess
        .trim() //read_line을 끝내기 위해 엔터를 누르고 개행문자가 추가되므로 필요
        .parse() // 문자열을 다른타입으로 바꿔줌. 변수 옆 콜론(:)과 타입 명시 필요
        .expect("Please type a number!");

    println!("You guessed: {guess}");

    // guess 와 secret_nunber를 비교 / cmp: 비교가능한 모든것을 호출 가능
    match guess.cmp(&secret_number) {
        // 1. cmp 가 비교한 값을 반환. 예) 50 vs 38 => Ordering::Greater
        Ordering::Less => println!("Too small!"), // 패턴이 매치 안되므로 첫번째 갈래 무시
        Ordering::Greater => println!("Too bug!"), // 이 갈래와 연관된 코드가 실행
        Ordering::Equal => println!("You win!"),  // 마지막 갈래는 확인하지 않음
    }
}
