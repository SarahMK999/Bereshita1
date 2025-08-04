

fn hello(){
    println!("hello");
}

fn check(num:i32){
    if num %2 ==0{
        println!("num is even");
    }else{
        println!("num is odd");
    }
}

fn biggest(num1:i32, num2:i32) -> i32{
    if num1>num2{
        return num1;
    }else{
        return num2;
    }

}


fn greet(name: &str) ->String{
format!("Hello {}", name)
}

fn main() {
    hello();
    check(5);
    println!("{}", biggest(4, 5));

    let result = greet("sarah");
    println!("{}", result);


}
