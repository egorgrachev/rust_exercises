pub fn add(x: i32, y: i32) -> i32 {
    x + y
}

pub fn get_last_element_in_array(array: &[i32]) -> Option<i32> {
    match array.last() {
        Some(&e) => Some(e.clone()),
        None => None,
    }
}

// pub fn

