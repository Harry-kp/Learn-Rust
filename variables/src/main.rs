fn main() {
    let x: f64 = 2.;
    println!("{}",x);

    // int
    let int1: i32 = 123;
    let int2: i64 = 1;
    let int3: i128 = 1;
    let int4: u32 = 1;
    let int5: u64 = 1;
    let int6: u128 = 1;
    println!("{} {} {} {} {} {}",int1,int2,int3,int4,int5,int6);

    let char1: char = 'X';
    println!("{}",char1);

    // Array
    let a = [2;5];
    println!("{:?}",a);

    // Invalid array access

    println!("{}",a[5]);

}
