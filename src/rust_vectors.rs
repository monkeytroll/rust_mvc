fn main() {
    // Array of tuples
    let my_tuple_arr: [(i32,i32); 5] = [(6,9); 5];
    println!("my_tuple_arr: {:?}", my_tuple_arr);

    // These two do the same.
    // let my_vec: Vec<(i32, i32)> = Vec::new();
    let mut my_vec: Vec<(i32, i32)> = vec![(1,2),(2,3)]; 
    println!("my_vec: {:?}", my_vec);

    // my_vec must be mutable
    my_vec.push((3,4));
    println!("my_vec: {:?}", my_vec);
}