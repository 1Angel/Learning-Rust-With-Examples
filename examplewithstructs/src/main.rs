fn main(){

    let structRectangle = Rectangle{
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle{
        width: 30,
        height: 100
    };

    println!("the area of the rectangle is {} pixels", rectangle2.area());
    println!("{:#?}", structRectangle);


}

#[derive(Debug)]
struct Rectangle{
    width: u32,
    height: u32
}

impl Rectangle {
    fn area(&self)->u32{
        self.width * self.height
    }
}