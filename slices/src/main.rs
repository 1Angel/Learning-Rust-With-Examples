fn main() {
    let text = String::from("hola que tal este es mi texto");

    let texto1 = &text[0..5];
    let lodemas = &text[5..];
    println!("{texto1}");
    println!("{lodemas}");


    let textofw = String::from("holamis amigos");

    let ua = first_word(&textofw);
    println!("{}", ua);

}


fn first_word(s: &String)->&str{
    let bytes = s.as_bytes();

    for (i, &item) in bytes.into_iter().enumerate() {
        if item == b' '{
            return &s[0..i];
        }
    }
    &s[..]
}