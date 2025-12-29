fn main() {
    let x = 5;
    println!("The value of x is: {x}");
    x = 6; // cannot assign twice to immutable variable `x`
    // 에러! x 는 불변으로 정의되었기 때문에 두번째 값을 할당할 수 없다
    println!("The value of x is: {x}");
}
