fn main(){

    let admin = Roles::Admin;
    let user = Roles::User;


    let somenumero = Some(5);
    let somechar = Some('b');
    let somestring = Some(String::from("la sociedad"));

    let nonenumero: Option<i32> = None;


    let user1 = User{
        username: String::from("angel01"),
        email: String::from("angel@gmail.com"),
        active: true,
        rol: Roles::Admin(String::from("admin"))
    };

    println!("{:#?}", user1);

}

#[derive(Debug)]
enum Roles{
    Admin(String),
    User(String)
}

#[derive(Debug)]
struct User{
    username: String,
    email: String,
    active: bool,
    rol: Roles
}


enum Option<E>{
    Some(E),
    None
}