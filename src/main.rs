fn main() {
    println!("Hello, world!");
}

#[test]
fn hello_test() {
    println!("Hello unit test");
}

#[test]
fn test_variable() {
    let name = "Rasyid";
    println!("{}", name)
}

#[test]
fn test_mutable() {
    let mut name = "Al";
    println!("{}", name);

    name = "Rasyid";
    println!("{}", name)
}
