fn main() {
    let cat = ("Furry McFurson", 3.5); // cat.0 cat.1

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    let (name, age) = cat;

    // Même chose
    //let name = cat.0;
    //let age = cat.1;

    println!("{name} is {age} years old");
}
