fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_value, bool_value) = pair;
    return(bool_value, int_value)
}

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                        -1i8, -2i16, -3i32, -4u64,
                                        0.1f32, 0.2f64,
                                        'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple first value: {}", long_tuple.1);

}