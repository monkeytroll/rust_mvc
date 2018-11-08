fn main () {
    // let s_a = [1,2,3];
    // let s_b: [i8; 3] = [4,5,6];
    // // Will throw error, len exceeds 3
    // // let s_c: [i8; 3] = [7,8,9,10];

    // println!("s_a: {:?}", s_a);
    // for i in 0..s_a.len() {
    //     println!("s_a[{}]: {}", i, s_a[i]);
    // }

    // println!("s_b, which has length: {}", s_b.len());
    // for (i, elem) in s_b.iter().enumerate() {
    //     println!("i: {}\telement: {}", i, elem);
    // }

    // b/c arrays are fixed size, not used often
    // slices are like views of underlying arrays

    let n: i32 = 10;
    for i in 1..n+1 {
        for j in 1..n+1 {
            for k in 1..n+1 {
                if i.pow(2) + j.pow(2) == k.pow(2) {
                    println!("i: {}\tj: {}\tk:{}", i, j, k);
                }
            }
            let squared_sum = i.pow(2) + j.pow(2);
            let fl_squared_sum = squared_sum as f64;
            let mod_1 = fl_squared_sum.sqrt()%(1 as f64);
            println!("squared_sum = {}", mod_1);
            println!("is good: {}", mod_1 == (0 as f64))
            // if squared_sum <= n &&  {
            //     println!("i: {}\tj: {}\tT: {}", i, j, i.pow(2) + j.pow(2));
            // }
        }
    }
    

}