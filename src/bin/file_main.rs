#[warn(dead_code)]
extern crate mime_guess;

fn main() {

//    stream_fold();
//    read_file();
//    write_file();
    print_exts();

}


fn stream_fold() {
    use futures::prelude::*;
    use futures::stream;
    use futures::future;

    let number_stream = stream::iter_ok::<_, ()>(0..6);
    let sum = number_stream.fold(0, |acc, x| future::ok(acc + x));
    assert_eq!(sum.wait(), Ok(15));
}

fn read_file() {
    use std::fs;
    let contents = fs::read_to_string("README.md")
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}

fn write_file() {
    use std::fs;
    let file = fs::File::open("README.md").unwrap();
    println!("metadata = {:?}", file.metadata().unwrap());

    let metadata = fs::metadata("README.md").unwrap();
    println!("{:?}", metadata.file_type());

    println!("------------");
}

fn print_exts() {
    let mime1 = "video/*";
    let mime2 = "video/x-matroska";


    println!(
        "Exts for {:?}: {:?}",
        mime1,
        mime_guess::get_mime_extensions_str(mime1)
    );
    println!(
        "Exts for {:?}: {:?}",
        mime2,
        mime_guess::get_mime_extensions_str(mime2)
    );
}