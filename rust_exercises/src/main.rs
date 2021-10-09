use std::ops::Add;

use itertools::izip;

fn main() {
    let mut result = vec![1, 2, 3];
    let input = vec![4, 5, 6];
    // result.extend(input.iter());

    // for x in input.into_iter() {
    //     result.push(x);
    // }
    // println!("{:?}", result);
    // // println!("{:?}", input);

    let output: Vec<i32> = izip!(result, input).map(|(x, y)| add(x, y)).collect();
    println!("{:?}", output);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
