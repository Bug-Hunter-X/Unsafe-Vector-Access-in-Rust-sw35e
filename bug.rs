fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    // this is incorrect, it will panic if index is out of bounds
    unsafe {
        *vec.get_unchecked(index) = 3;
    }
    println!("{:?}", vec);
}