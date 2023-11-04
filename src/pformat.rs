fn main() {
    println!("{}", 31);
    println!("{2},{1},{0}",0,1,2);
    println!("{a},{b},{c}",a="A",b="B",c="C");
    println!("{},{:b},{:o},{:x},{:X}",10,10,10,10,10);
    println!("{x:>10}",x=10);
    println!("{x:0>10}",x=10);
    println!("{x:0<10}",x=10);
    //struct Structure(i32);
    //println!("This struct `{}` won't print...", Structure(3));
    let number: f64 = 1.0;
    let width: usize = 5;
    println!("{number:>width$}");
}