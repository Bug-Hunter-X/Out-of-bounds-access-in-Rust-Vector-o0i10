fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //This is where the error occurs
    let idx = vec.len();
    println!("{}", vec[idx]);
}