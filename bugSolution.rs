fn main() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);

    //Corrected code
    let idx = vec.len() -1;
    println!("{}", vec[idx]); 
    //Or using get method to avoid the panic
    if let Some(value) = vec.get(2){
        println!("{}", value);
    }
} 