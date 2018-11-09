fn main() {
    let n: i32 = 30;

    println!("pythagorean: {:?}", pythagorean(n));

    let mut pythagorean_n = pythagorean(n);


}

fn pythagorean (n: i32) -> Vec<(i32, i32, i32)> {
    let mut pythagorean: Vec<(i32, i32, i32)> = vec![];
    let mut singular_pythagorean: Vec<(i32, i32, i32)> = vec![];

    for i in 1..n+1 {
        for j in 1..n+1 {   
            let squared_sum = i.pow(2) + j.pow(2);
            let fl_squared_sum = squared_sum as f64;
            let mod_1 = fl_squared_sum.sqrt() % (1 as f64);
            
            if fl_squared_sum <= (n.pow(2) as f64) && mod_1 == (0 as f64) {
                pythagorean.push((i, j, fl_squared_sum.sqrt() as i32));
                // println!("{}", pythagorean.contains(&(i,j,fl_squared_sum.sqrt() as i32)));
            }
        }
    }

    // pythagorean.retain(|&x| pythagorean.contains(&(x.1, x.0, x.2)));
    // pythagorean.retain(|&x| x.0 > 15);
    pythagorean.retain(|&x| {
        let tuple = (x.1, x.0, x.2);
        let return_value = !singular_pythagorean.contains(&tuple);
        singular_pythagorean.push(x);
        return return_value;
    });

    return pythagorean
}