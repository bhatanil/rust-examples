fn reverse(pair: (i32, bool)) -> (bool, i32) {
    let (int_value, bool_value) = pair;
    return(bool_value, int_value)
}

#[derive(Debug)]
struct Matrix(f32, f32, f32, f32);

fn main() {
    let long_tuple = (1u8, 2u16, 3u32, 4u64,
                                        -1i8, -2i16, -3i32, -4i64,
                                        0.1f32, 0.2f64,
                                        'a', true);
    println!("long tuple first value: {}", long_tuple.0);
    println!("long tuple first value: {}", long_tuple.1);

    let tuple_of_tuples = ((1u8, 2u16, 3u32), (1u8, 2u16), 1u64);
    println!("tuple of tuples {:?}", tuple_of_tuples);

    let pair = (1, true);
    println!("Pair is {:?}", pair);
    println!("Pair is {:?}", reverse(pair));

    let tuple = (1, "hello", 4.5 ,true);
    let (a, b, c, d) = tuple;
    println!("{}, {}, {}, {}", a, b, c, d);
    println!("{:?}, {:?}, {:?}, {:?}", a, b, c, d);

    let matrix = Matrix(1.1, 1.2, 2.1, 2.2);
    println!("{:?}", matrix);
}