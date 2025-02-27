#[derive(Debug)]
enum Message {
    // TODO: Define a few types of messages as used below.
    Quit,
    Resize, 
    Move,
    Echo(String),
    ChangeColor
}

fn main() {
    println!("{:?}", Message::Resize);
    println!("{:?}", Message::Move);
    println!("{:?}", Message::Echo(String::from("Hello")));
    println!("{:?}", Message::ChangeColor);
    println!("{:?}", Message::Quit);
}
