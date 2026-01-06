fn main() {
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3; // 상수
    /* 
    1. 반드시 대문자를 사용하고 타입을 표기
    2. 런타임에서 값을 확인할 수 있는 값은 넣을 수 없음 (예: x + 1)
    3. 하드코딩된 값을 다른 개발자에게 유용하게 정보로 넘길 수 있음 
    4. 선언된 스코프 내에서 프로그램이 동작하는 전체 시간 동안 유효하다. 
     */
    let mut x = 5; 
    println!("The value of x is: {x}");
    x = 6; 
    println!("The value of x is: {x}");
}
