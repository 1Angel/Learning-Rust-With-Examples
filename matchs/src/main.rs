fn main(){

    let x = 10;


    match x {
        1=>println!("uno"),
        2=>println!("dos"),
        3=>println!("treh"),
        _=>println!("default")
        
    }



    fn plus_one(x: Option<i32>)->Option<i32>{
        match x {
            None => None,
            Some(i)=> Some(i+1),
        }
    }


    let five =Some(5);
    let six = plus_one(five);
    println!("{:?}", six);


    #[derive(Debug)]
    enum Cardinales {
        Norte,
        Sur,
        Este,
        Oeste
    }


    fn puntos_cardinales(c: Cardinales){
        match c {
            Cardinales::Norte=>{
                println!("norte brrr")
            }
            Cardinales::Este=>{
                println!("este")
            }
            Cardinales::Oeste=>{
                println!("oeste")
            }
            Cardinales::Sur=>{
                println!("surrr")
            }
        }
    }


    let norte = puntos_cardinales(Cardinales::Norte);
    println!("{:?}", norte);

}