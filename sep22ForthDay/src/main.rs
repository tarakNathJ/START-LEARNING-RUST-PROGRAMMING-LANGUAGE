fn main() {
    // today learn about slice
    // println!("Hello, world!");
    // let  first_string = String::from("hello world");

    // let size_of_string = first_function_send_ownership(&first_string);
    // let firstslice = &first_string[..5];

    // println!("{size_of_string}");

    // // first_string.clear();
    // //  i under stand this problem state ment
    // println!(" this your clear  string and this is a problem  :-- {first_string} {firstslice}");
    // // first_string.clear();
    
    // println!("size of slice is {}",size_of_string);
    println!("valu is :-: {}",ownership_issue(40));
}


// slice chack program
fn first_function_send_ownership(data: &String) -> &str {
    let bytes = data.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &data[..i];
        }
    }

    return &data[..];
}




fn ownership_issue(data:i32) -> i32{

    let   power_of_data = data *data;

    return  power_of_data;
}