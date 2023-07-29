
struct numbers(i32, u32, bool);
struct colors(i32, i32, i32);




fn main(){

    let numeros = numbers(-12, 3113, false);
    let colores = colors(12121,21313,342432);

    println!("{}", numeros.0);

    let mut usuario1 = User{
        name: String::from("angel"),
        username: String::from("angel01"),
        active: true,
        sign_in_count: 1,
    };


    let usuario2 = User{
        name: usuario1.name,
        username: usuario1.username,
        active: true, 
        sign_in_count: 1
    };


    let usuario3 = User{
        name: String::from("music01"),
        ..usuario2
    };


    println!("{}", usuario2.name);


    let post1 = Posts{
        title: String::from("my first post"),
        description: String::from("this is my first posot"),
        user: User { name: String::from("newj"), username: String::from("dhuwow"), active: true, sign_in_count: 1 },
    };



    let userregister  = userRegister(String::from("ozuna"), String::from("ozuna01"), true);
    println!("{}", userregister.name);
}

struct User{
    name: String,
    username: String,
    active: bool,
    sign_in_count: u64
}

struct Posts{
    title: String,
    description: String,
    user: User
}



fn userRegister(name: String, username: String, active: bool)-> User{

    User { name, username: username, active: active, sign_in_count: 1 }



}