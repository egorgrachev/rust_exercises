use itertools::izip;

mod lib;


fn main() {
    let result = vec![1, 2, 3];
    let input = vec![4, 5, 6];

    let output: Vec<i32> = izip!(result,
        input).map(|(x, y)| lib::add(x, y)).collect();
    println!("{:?}", output);
}
