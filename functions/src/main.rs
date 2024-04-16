fn main() {
    println!("Hello, world!");
    // another_function();
    another_function(12,2);
    add_new(121,2);
}

// fn another_function() {
//     println!("Another function");
// }

// Add params

fn another_function(x: i32, y: i32) {
    println!("{}",x * y);
}

// this code will panick
fn add(x: i32, y: i32) -> i32 {
    x + y
}

// this will not
fn add_new(x: i32, y: i32) -> i32 {
    x + y
}
