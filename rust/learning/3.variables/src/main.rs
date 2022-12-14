fn main() {
    let mut x = 5; // with the mut, means the variable is mutable. If the declaration of the
                   // variable dont get the "mut", it's immutable
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");
}
