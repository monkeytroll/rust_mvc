// use std::io::stdin;

fn main() {
    // variable
    let A = 10;
    let B = 20;

    // lambda/closure
    let basic_sum = |x: i32, y: i32| x + y;
    let closure_scope = |x: i32| x + A;

    println!("Hello world!");
    print_name("David");
    // println!(value) will not work, must provide string literal
    println!("{}", perimeter(2,3));

    println!("sum A and B: {}", basic_sum(A,B));
    println!("test closure scope by summing 2 and A: {}", closure_scope(2));

    // If a variable is NOT mutable...
    let mut c = 69;
    println!("c: {}", c);
    // This following line will be an error cannot assign twice to immutable variable
    c = c + 1;
    println!("c+1: {}", c);

    // iterating over a string
    let d = "some random string";
    let d_chars = d.chars();
    println!("d_chars: {:?}", d_chars);
    for d_char in d.chars() {
        println!("{}", d_char);
    }

    // Rust mutability
    let x = 5;
    let mut y = x;
    y = y + 5;
    println!("y: {}", y);

}

// Snake case is used
fn print_name (name: &str) {
    println!("{}", name);
}

fn perimeter (length: i32, width: i32) -> i32 {
    // no semicolon == return (statement)
    // arithmetic done in order (w/o brackets)
    length*2 + width*2
}