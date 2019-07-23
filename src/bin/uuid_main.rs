extern crate rand;
extern crate rand_core;
extern crate uuid;

use rand_core::RngCore;
use uuid::Uuid;
use rand::Rng;
use rand::distributions::Alphanumeric;

fn main() {
    let my_uuid =
        Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8").unwrap();
    println!("{}", my_uuid.to_urn());

    println!("-------------------");
    let mut rng = rand::thread_rng();
    println!("{}", rng.next_u64());
    println!("-------------------");

    // 随机字符串
    let random1 = rng.sample_iter(&Alphanumeric).take(10).collect::<String>();
    println!("{}", random1);
}