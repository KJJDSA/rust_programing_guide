fn main() {
    let mut x = 5; // mut 으로 변수를 가변으로 바꿀 수 있다.
    // mut을 사용하지 않으면 불변하는 변수만으로 코딩해야 하므로 단순한 반복문도 재귀함수를 써야 하는 등 성능적 한계가 발생할 수 있다. 
    // 즉 mut을 사용하는 것을 최대한 지양하되, 성능적으로 향상이 불가피해보인다면 mut 을 균형있게 사용해야 한다. 
    println!("The value of x is: {x}");
    x = 6; 
    println!("The value of x is: {x}");
}
