// use std::io;

// fn main() {
//     let a = [1, 2, 3, 4, 5];

//     println!("Please enter an array index.");

//     let mut index = String::new();

//     io::stdin()
//         .read_line(&mut index)
//         .expect("Failed to read line");

//     let index: usize = index
//         .trim()
//         .parse()
//         .expect("Index entered was not a number");

//     let element = a[index];

//     println!("The value of the element at index {index} is: {element}");
// }


use core::panic;
// 95°F (35°C)
fn main() {
    

    let input = String::from("170°F");
    let result = converter(&input);
    println!("Converted Temperature: {}°C", result); // Output: 132°C

}


fn converter(temp: &String) -> i32 {

    let mut res_str = String::new();
    let mut res = 0;

    for i in temp.trim().chars() {

        if i.is_digit(10) {

            res_str.push(i);

        } else if i == '°' {

            res = match res_str.parse::<i32>() {
                Ok(num) => num,
                Err(_) => panic!("Failed to parse number: {}", res_str),
            };
            continue;

        } else if i == 'F' {

            print!("res_str {}, res {}", res_str, res);
            res = (res - 32) * 5 / 9;
            break;

        } else if i == 'C' {

            res = (res * 9 / 5) + 32;
            break;

        } else {

            panic!("Unexpected character: {}", i)
        }

   }
    res


}