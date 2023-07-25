fn main() {

    let mut x = 5;
    println!("the value of x is: {x}");

    x = 6;
    println!("the value of x is: {x}");  

    const NAME: &str = "peter griffin";
    const THREE_HOURS_IN_SECONDS: u32 = 60*60*3;

    println!("your name is: {NAME}");  
    println!("{0}", THREE_HOURS_IN_SECONDS);


    let number = 5;
    let number = number+1;

    println!("{number}");
}
