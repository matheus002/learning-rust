#[allow(unused_variables)]
fn main() {
    // // imutable variable
    // let mut x: i32 = 5;
    // // mutable variable, witch you can reassign the value of that variable during the code flow
    // println!("The value of x is: {}", x);
    // x = 6;
    // println!("The value of x is: {}", x);
    // let mut x2: i32 = 5;
    // // const variable ( the common idea is to use const names with upper case and underscore )
    // // const cannot be mutted, with mut keyword, and the type HAS to be explicit, different of the let variable
    // const SUBS_COUNT: u32 = 32548;

    let x: i32 = 5;
    println!("the value of x is: {}", x);
    let x: &str = "text";
    println!("the value of x is: {}", x);

    // Data types
    //Integers
    let a: i32 = 98_222; //Decimal
    let b: i32 = 0xff; //Hex
    let c: i32 = 0o77; // Octal
    let d: i32 = 0b1111_0000; // Binary
    let e: u8 = b'A';
    //Floating-point numbers

    let f = 2.0;
    let g: f32 = 3.0;
    //Booleans
    //Character
    let value: char = 'A';

    //Compoud Types
    let tup: (&str, i32) = ("This is a sample", 100_000);
    let (channel, sub_count) = tup;
}
