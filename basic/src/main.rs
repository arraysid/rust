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

#[test]
fn test_arrays() {
    let mut array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", array);

    let a = array[0];
    let b = array[1];

    println!("{} {}", a, b);

    array[0] = 10;

    println!("{:?}", array);

    println!("arrays is {} long", array.len())
}

#[allow(dead_code)]
const MINIMUM: i8 = 0;

#[test]
fn test_const() {
    const MAXIMUM: i8 = 5;
    println!("Minimum: {}, Maximum: {}", MINIMUM, MAXIMUM)
}

#[test]
fn test_inner_scope() {
    println!("Minimum: {}", MINIMUM);

    {
        const AVERAGE: f32 = 2.5;
        println!("{} {}", AVERAGE, MINIMUM)
    }
}
