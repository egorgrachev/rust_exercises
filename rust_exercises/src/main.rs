fn main() {
    let mut result = vec![1,2,3];
    let input = vec![4,5,6];
    result.extend(input.iter());

    println!("{:?}", result);
    println!("{:?}", input);

}

