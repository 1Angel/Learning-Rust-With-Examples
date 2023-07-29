fn main() {

    let mut texto = String::from("holla que tal");

    mutReferencia(&mut texto);
    let otrareferenciaxd = otraReferencia(&texto);
    let textoreferencia = referencia(&texto);
    
    println!("{}", textoreferencia);
    println!("{}", otrareferenciaxd);
}


fn referencia(s: &String)->usize{
    s.len()
}


fn otraReferencia(v: &String)->&String{
    v
}

fn mutReferencia(m: &mut String){
    m.push_str("la sociedad");
}