#[warn(unused_must_use)]
use futures::Future;
use hyper::Client;

fn main() {
    let client = Client::new();

    let future = client.get("http://httpbin.org/ip".parse().unwrap());
    println!("1");
    future.and_then(|res| {
        println!("{:?}", res);
        Ok(())
    }).map_err(|err|{
        println!("{:?}",err);
    });
}