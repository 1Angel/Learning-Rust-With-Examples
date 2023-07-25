fn main() {
    let number1: i16 = -1;
    let number2: u8 =  255;
    let number3: i8 = -12;


    let x: f32 = 12.31;
    let y  =1.9;
    let z = 1.3;


    let number1sum = 12;
    let number2sum = 32;

    let sum = number1sum + number2sum;
    let difference = number1sum - number2sum;

    let product = number1sum * number2sum;
    let quotient = 1231/20;

    let f: bool = false;
    let t: bool = true;

    let caracter: char = 'a';
    let c = 'c';
    let emote = '🤑';


    let string1 = "this is my string";
    let string2: &str = "this is my second string";
    let  string3: &str;

    string3 = "la sociedad del dinero";


    // println!("The result is: {sum}");
    // println!("{difference}");
    // println!("{product}");
    // println!("{quotient}");

    // println!("{emote}");
    // println!("{string3}");

    //tuples
    let tuple: (i32, &str, bool) = (213, "hixd", true);
    let (x,y,z) = tuple;



    let mitupla: (i32, &str, bool, char) = (123, "lasociedad", false, 'a');

    println!("{x}");
    println!("{0}", tuple.1);

    let society = mitupla.1;
    println!("{society}");

    //array

    let myarray = [1,2,3,4,5,6,7,8];

    let names: [&str; 6] = [
        "peter", "juan", "bob", "hank", "maria", "bryant"
    ];

    let first = names[0];
    println!("{first}");



}
