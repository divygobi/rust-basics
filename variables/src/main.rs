use std::io;
fn main() {
    let mut arr_len = String::new(); 

    println!("Please enter the length of the array.");
    io::stdin()
        .read_line(&mut arr_len)
        .expect("Failed to read line");
    
    let arr_len: usize = arr_len
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let a = guchungalex(arr_len);

    println!("Please enter an array index.");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    if index >= arr_len{
        println!("Index out of bounds");
        return;
    }
    let element = a.get(index);

    println!("The value of the element at index {index} is: {:?}", element);
}

fn guchungalex(arr_len: usize) -> Vec<String>{

    let mut a = Vec::new();
    let mut i: usize = 0;
    loop{

        println!("Please enter the element number {} of the vector.",i+1);
        let mut element = String::new();
        io::stdin()
            .read_line(&mut element)
            .expect("Failed to read line");
        a.push(element);
        i+=1;
        if i >= arr_len{
            break;
        }
    }
    return a;

}
