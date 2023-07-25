fn main() {
    println!("Hello, world!");

    let  mut counter = 0;

    loop {
        print!("agains")
    }

    loop {
        counter+=1;
        println!("{counter}");
        if counter == 20{
            break;
        }
    }


    'counting: loop {
        let mut counter2 = 10;

        loop{
            println!("{counter2}");
            if counter2 == 9{
                break;
            }
            if counter == 2{
                break 'counting;
            }
            counter2 -=1;
        }
        counter+=1;
    }
    println!("{counter}")

}