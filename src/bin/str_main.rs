use std::ops::Add;

fn main() {
    string_add_string();
    string_add_str();
}

fn string_add_string() {
    println!("string_add_string");
    let a = "a".to_owned();
    let b = "b".to_owned();
    println!("{}\n", a.add(&b));

    println!("-------------------");

    let mut c = "c".to_owned();
    let d = "d".to_owned();
    c.push_str(&d);
    println!("{}\n", c);
}

fn string_add_str() {
    println!("string_add_str");
    let mut a = "a".to_owned();
    let b = "b";
    a.push_str(b);
    println!("{}", a);

    println!("--------------------");

    let d = "d";
    let e = "c".to_owned()+d;
    println!("{}", &e);
}