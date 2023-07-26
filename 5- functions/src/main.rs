fn main() {
    // println!("Hello, world!");
    // hello();
    // hello_name("ozuna");
    // sum(12, 31);


    // let x = Hi();
    // println!("the result is {x}");

    // let x = plus_one(10);
    // println!("{x}");
    let f: f32 = flotante(1.2);
    let d: &str = cadena("hello");
    println!("{f}");
    
}

fn hello(){
    println!("hello");
}

fn hello_name(name: &str){
    println!("hello {name}");
}

fn sum(num1: i32, num2:i32){
    let sumar = num1 +num2;
    println!("the result is {sumar}")
}


fn Hi() -> i32{
    5
}

fn plus_one(x: i32) -> i32 {
    x+1
}

fn cadena(a: &str)-> &str{
    a
}

fn flotante(x: f32)-> f32{
    x
}