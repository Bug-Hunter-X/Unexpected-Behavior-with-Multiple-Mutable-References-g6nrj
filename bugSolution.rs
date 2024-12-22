fn main() {
    let mut x = 5;
    { // Create a new scope to limit the lifetime of y 
        let y = &mut x; // y is a mutable reference to x
        *y = 6; 
    }
    let z = &mut x; // Now, it's okay to have z 
    *z = 7;
    println!("x = {}", x); // This correctly prints 7 
} 