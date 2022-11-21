fn main() {
    println!("Hello, world!");

    println!("===================");
    let greeting = "Hello";
    let subject = "World";

    println!("{} {}!", greeting, subject);

    println!("===================");

    let subject = "World";
    let greeting = format!("Hello, {}!", subject);
    println!("{greeting}");

}
