fn main() {
    let mut v: Vec<i32> = Vec::new();
    let v1 = vec![1,2,3,5,6,7,8,9];


    let tercero: &i32 = &v1[1];
    println!("{}", v1[2]);
    println!("{}", tercero);
    v.push(1);
    v.push(2);


    for a in &v1 {
        println!("{}", a);
    }


    v.pop();
    
    println!("lenght is {}", v.len());


    match v1.get(1) {
        Some(tercero) => println!("coincide con el tercer elemento"),
        None=> println!("no existe"),
    }
}
