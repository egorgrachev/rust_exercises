#[test]
fn it_adds_two() {
    assert_eq!(4, 2 + 2);
}


#[test]
fn test_last_element_in_array() {
    let array = [1, 2, 3];
    assert_eq!(get_last_element_in_array(&array), 3);

    let empty_array = [];
    assert_eq!(get_last_element_in_array(&empty_array), 0);
}

fn get_last_element_in_array(array: &[i32]) -> i32 {
    if array.len() == 0 {
        return 0
    }
    array.last().unwrap().clone()
}