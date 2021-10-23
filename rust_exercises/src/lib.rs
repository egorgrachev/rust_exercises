pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn get_last_element_in_array(array: &[i32]) -> i32 {
    if array.len() == 0 {
        return 0;
    }
    array.last().unwrap().clone()
}

pub fn get_first_element_in_array(array: &[i32]) -> i32 {
    if array.len() == 0 {
        return 0;
    }
    array.first().unwrap().clone()
}

