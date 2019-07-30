use std::borrow::Borrow;

fn main() {




//    loop_collect();
}

fn splite_string() {
    let v: Vec<&str> = "Mary had a little lamb".split(' ').collect();
    assert_eq!(v, ["Mary", "had", "a", "little", "lamb"]);

    let v: Vec<&str> = "abc1def2ghi".split(char::is_numeric).collect();
    assert_eq!(v, ["abc", "def", "ghi"]);

    let v: Vec<&str> = "lionXtigerXleopard".split(char::is_uppercase).collect();
    assert_eq!(v, ["lion", "tiger", "leopard"]);

    let v: Vec<&str> = "abc1defXghi".split(|c| c == '1' || c == 'X').collect();
    assert_eq!(v, ["abc", "def", "ghi"]);

    let x = "||||a||b|c".to_string();
    let d: Vec<_> = x.split('|').collect();

    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);

    let x = "()".to_string();
    let d: Vec<_> = x.split('/').collect();

    assert_eq!(d, &["(", "", "", ")"]);

    let d: Vec<_> = "010".split("0").collect();
    assert_eq!(d, &["", "1", ""]);

    let f: Vec<_> = "rust".split("").collect();
    assert_eq!(f, &["", "r", "u", "s", "t", ""]);

    let x = "    a  b c".to_string();
    let d: Vec<_> = x.split(' ').collect();

    assert_eq!(d, &["", "", "", "", "a", "", "b", "c"]);
}


fn loop_collect() {
    let v1 = vec!["tag1", "tag2", "tag3"];
    let last = v1.last().unwrap().clone();
    for v in v1 {
        if v == last {
            println!("{}", v);
        } else {
            print!("{}-", v);
        }
    }

    println!("--------------------");

    let v2 = vec!["tag1"];

    if v2.len() > 1 {
        for i in 0..(v2.len() - 1) {
            print!("{}-", v2.get(i).unwrap());
        };
    }
    println!("{}", v2.last().unwrap());
}
