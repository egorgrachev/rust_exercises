use rust_exercises::get_first_element_in_array;
use rust_exercises::get_last_element_in_array;

#[test]
fn it_adds_two() {
    assert_eq!(4, 2 + 2);
}


#[test]
fn test_last_element_in_array() {
    let array = [1, 2, 3];
    assert_eq!(get_last_element_in_array(&array), Some(3));

    let empty_array = [];
    assert_eq!(get_last_element_in_array(&empty_array), None);
}

#[test]
fn test_first_element_in_array() {
    let array = [1, 2, 3];
    assert_eq!(get_first_element_in_array(&array), 1);

    let empty_array = [];
    assert_eq!(get_first_element_in_array(&empty_array), 0);
}