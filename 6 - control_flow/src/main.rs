fn main() {
    println!("Hello, world!");

    let number = 10;


    if number < 15{
        println!("the condition was true");
    }else {
        println!("the conditionn was false");
    }

    if number !=0{
        println!("numbers is not zero")
    }else {
        println!("number is zero")
    }


    if number % 4 ==0{
        println!("number is divisible by 4");
    }
    else if number %3 ==0 {
        println!("number is divisible by 3");
    }
    else if number %2==0 {
        println!("number is divisible by 2");
    }




    let condition = true;
    let number1 =if condition {5} else {6};
}
