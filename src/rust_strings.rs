fn main() {
    let s_a = "asdf asdf";
    println!("s_a: {}", s_a);

    let s_a_copy = &s_a[0..s_a.len()];
    let s_a_half = &s_a[0..4];


    println!("s_a_copy: {}", s_a_copy);
    println!("s_a_half: {}", s_a_half);

}
