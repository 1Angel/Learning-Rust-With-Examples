fn main(){

    let s ="hello";


    let mut s3 = String::from("hello");
    s3.push_str(", la sociedad");
    println!("{s3}");

    let x = 5;
    let y = x;

    let numero1 = true;
    let numero2 = numero1;



    let mut s1 = String::from("hello s1");
    let s2 =s1.clone();

    println!("{s1}");
    println!("{}", s2);

    let fnS = String::from("hello");

    let ua = takes_and_gives_back(fnS);
    print!("{ua}");
    //takes_ownership(fnS);


}   


fn takes_ownership(somes_string: String){
    println!("{}", somes_string);
}

fn takes_and_gives_back(a_string: String)-> String{
    a_string
}