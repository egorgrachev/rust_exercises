use itertools::izip;
use std::ops::Add;

fn main() {
    let result = vec![1, 2, 3];
    let input = vec![4, 5, 6];

    let output: Vec<i32> = izip!(result,
        input).map(|(x, y)| add(x, y)).collect();
    println!("{:?}", output);
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}
