use std::io; // 스코프라고 부른다. std 표준 라이브러리의 io 라이브러리를 가져오겠다

fn main() {
    println!("Guess the number!");

    println!("Please input your guess.");

    let mut guess = String::new(); // (모듈, 타입)::(연관함수) new() 는 String 타입의 연관함수로 빈 문자열을 생성하는 인스턴스(함수의 구현체)를 반환

    io::stdin() // io 모듈의 연관함수 stdin() 은 터미널의 표준입력 핸들을 나타내는 타입인 std::io::Stdin 의 인스턴스를 반환한다.
        .read_line(&mut guess)
        // & : 인수가 복사가 아닌 참조임을 나타낸다(참조자).
        // &mut : 참조자는 기본적으로 불변이기 떄문에 가변으로 만들어 read_line 한 변수를 변경할 수 있게 한다.
        .expect("Failed to read line");
    // 앞선 메서드(read_line) 가 반환한 Result(Enum / Ok or Err)의 베리언트(variant)가 Err일 경우 (쉽게말해 read_line 이 처리 실패할 경우), 인수 속 문자열을 출력하는 메서드.
    // .expect를 호출하지 않으면 read_line이 반환한 Result가 사용되지 않게된다. 이 떄 컴파일은 되지만 경고가 나타난다.

    println!("You guessed: {guess}");
    // {} <<< placeholder
    // 빈 {}을 넣고 쉼표 뒤에 표현식으로 넣을 수도 있다. ("You guessed: {}" , x + 1) <<< x + 1 값이 {} 에 들어간다.
}
