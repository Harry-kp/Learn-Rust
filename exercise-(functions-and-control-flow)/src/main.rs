fn main() {
    println!("Hello, world!");
    println!("{}", celsius_to_farenheit(1.0));
    println!("{}", nthfib(10))
}

fn celsius_to_farenheit(c: f64) -> f64 {
    (c*9.0)/5.0 + 32.0
}


fn nthfib(n: u32) -> u64 {
    let (mut a,mut b) = (0,1);

    for i in (2..=n) {
        let c = a+b;
        a = b;
        b = c;
    }
    b
}
