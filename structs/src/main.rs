#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

#[derive(Debug)]
struct Square {
    side: u32,
}

fn main() {
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    let square1 = Square { side: rect1.height };

    println!("rect1 is {:#?}, square1 is {:#?}", rect1, square1);
}
