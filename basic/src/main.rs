fn main() {
    println!("Hello, world!");
}

#[test]
fn test_hello() {
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

#[test]
fn test_shadowing() {
    let name = "Rasyid";
    println!("{}", name);

    let name = 1234;
    println!("{}", name)
}

#[allow(dead_code)]
fn unit() {
    println!("test_unit")
}

#[test]
fn test_unit() {
    let result = unit();
    println!("{:?}", result)
}
