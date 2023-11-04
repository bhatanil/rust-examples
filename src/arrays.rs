//use std::mem;

fn analyze_slice(slice: &[i32]) {
    println!("First element of the slice: {}", slice[0]);
    println!("Ths slice has {} elements", slice.len());
}
fn main () {
    let xs: [i32; 5] = [1, 2, 3, 4, 5];
    let ys: [i32; 500] = [0; 500];

    println!("First element of the array: {}", xs[0]);
    println!("Second element of the array: {}", xs[1]);
    println!("Number of elements in the array: {}", xs.len());

    analyze_slice(&xs);
    analyze_slice(&ys[1 .. 4]);

    let empty_array: [u32; 0] = [];
    assert_eq!(&empty_array, &[]);
    assert_eq!(&empty_array, &[][..]);

    for i in 0..xs.len() + 1 {
        match xs.get(i) {
            Some(xval) => println!("{}: {}", i, xval),
            None => println!("Slow Down! {} is too far!", i),
        }
    }
}