fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    let index = 1;
    // Safe alternative: uses get_mut to access the element at the given index
    // This method checks the index bounds and will return None if the index is out of bounds
    if let Some(value) = vec.get_mut(index) {
        *value = 3;
    } else {
        println!("Index out of bounds");
    }
    println!("{:?}", vec);
}