// primitive_types3.rs
// Create an array with at least 100 elements in it where the ??? is.
// Execute `rustlings hint primitive_types3` for hints!

fn main() {
    let a = generate_array();
    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
    }
}


fn generate_array () -> Vec<i32> {
    let mut index = 0;
    let mut num: Vec<i32> = Vec::new();
    while index < 100 {
        num.push(index);
        index +=1;
    }

    return num;
}
