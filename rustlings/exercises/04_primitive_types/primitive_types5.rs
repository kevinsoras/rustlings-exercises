fn main() {
    let cat = ("Furry McFurson", 3.5);

    // TODO: Destructure the `cat` tuple in one statement so that the println works.
    // let /* your pattern here */ = cat;
    // Only if is a primitive value you can copy automatically, for non-primitive values move the value
    let (name,age)= cat;
    println!("{name} is {age} years old");
}
