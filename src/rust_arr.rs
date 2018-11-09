fn main () {
    let s_a = [1,2,3];
    let s_b: [i8; 3] = [4,5,6];
    // Will throw error, len exceeds 3
    // let s_c: [i8; 3] = [7,8,9,10];

    println!("s_a: {:?}", s_a);
    for i in 0..s_a.len() {
        println!("s_a[{}]: {}", i, s_a[i]);
    }

    println!("s_b, which has length: {}", s_b.len());
    for (i, elem) in s_b.iter().enumerate() {
        println!("i: {}\telement: {}", i, elem);
    }

    b/c arrays are fixed size, not used often
    slices are like views of underlying arrays
    

}