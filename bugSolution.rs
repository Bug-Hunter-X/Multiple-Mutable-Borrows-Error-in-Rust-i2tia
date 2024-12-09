fn main() {
    let mut x = 5;
    { //This scope limits the mutable borrow of y to avoid conflicts
        let y = &mut x;
        *y = 10;
    }
    let z = &mut x; 
    *z = 15;
}