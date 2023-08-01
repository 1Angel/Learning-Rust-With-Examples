fn main(){

    let mut post1 = Post{
        title: String::from("my post"),
        description: String::from("dklkdnekoedn"),
        published: true
    };

    post1.title = String::from("nkofeknk");

    let juan = CreatePost(String::from("juana se mala?"), String::from("joenfjonownfnew"), true);

    println!("{:#?}", post1)  ;
    println!("{}", post1.title);


    let rectangle1 = Rectangle{
        width: 50,
        height:129
    };

    println!("{:#?}", rectangle1.calArea());

}


#[derive(Debug)]
struct Post{
    title: String,
    description: String,
    published: bool
}


#[derive(Debug)]
struct Rectangle{
    width: i32,
    height: i32
}

impl Rectangle {
    fn calArea(&self)->i32{
        self.width * self.height
    }
}



fn CreatePost(title: String, description: String, published: bool)-> Post{
    Post { title: title, description: description, published: published }
    
}