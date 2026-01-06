fn main() {
    let x = 5;

    // 섀도잉 : 기존의 값을 잠시 가린다. 섀도잉은 스코프가 끝나면 걷혀진다. 
    // 즉 같은 스코프에서 변한 것은 영구적으로 변한 것이라 봐야한다. 
    let x = x + 1;

    {
        let x = x * 2;
        println!("The value of x in the inner scope is: {x}"); // 12
    }

    println!("The value of x is: {x}"); // 다시 6으로 돌아왔다. 
}
