use std::io;

fn main() {
    let a = [1,2,3,4,5];

    println!("Enter the array index: ");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index.trim().parse().expect("Please Enter a Number");

    if index > 4 {
        println!("Index Out of Bounds");
        return;
    }

    let element = &a[index];
    println!("The element at index {} is: {}", index, element);
}
