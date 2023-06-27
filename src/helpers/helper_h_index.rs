pub fn get_h_index(arg_slice: &[u32]) -> u32 {
    let mut vec_sorted = Vec::from(arg_slice);
    vec_sorted.sort_by(|item_left, item_right| item_right.partial_cmp(item_left).unwrap());

    for (item_index, item_value) in vec_sorted.iter().enumerate() {
        let item_h_value = (item_index + 1) as u32;
        if item_h_value > *item_value {
            return (item_h_value - 1).into();
        }
    }
    0
}
