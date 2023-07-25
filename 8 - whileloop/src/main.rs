fn main() {


    let mut number = 20;
    let myarray = [112,21,32,43,53,61];
    let mut index =0;


    while index <myarray.len(){
        println!("{}", myarray[index]);
        index+=1;
    }

    while number<30{
        number+=1;
        println!("{number}")
    }


    while number !=0{
        println!("{number}");
        number-=1;
    }


    while number <50 {
        println!("{number}");
        number+=1;
        if number == 44{
            break;
        }
        
    }
}
