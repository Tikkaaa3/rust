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


use std::io;

fn main() {
    println!("Temperature Converter");
    println!("Enter '1' to convert Fahrenheit to Celsius");
    println!("Enter '2' to convert Celsius to Fahrenheit");

    let mut choice = String::new();
    io::stdin().read_line(&mut choice).expect("Failed to read input");
    let choice: u32 = choice.trim().parse().expect("Please enter a valid number");

    match choice {
        1 => fahrenheit_to_celsius(),
        2 => celsius_to_fahrenheit(),
        _ => println!("Invalid choice! Please select either '1' or '2'."),
    }
}

fn fahrenheit_to_celsius() {
    println!("Enter temperature in Fahrenheit:");
    let mut fahrenheit = String::new();
    io::stdin().read_line(&mut fahrenheit).expect("Failed to read input");
    let fahrenheit: f64 = fahrenheit.trim().parse().expect("Please enter a valid number");

    let celsius = (fahrenheit - 32.0) * 5.0 / 9.0;
    println!("{fahrenheit}°F is equal to {celsius:.2}°C");
}

fn celsius_to_fahrenheit() {
    println!("Enter temperature in Celsius:");
    let mut celsius = String::new();
    io::stdin().read_line(&mut celsius).expect("Failed to read input");
    let celsius: f64 = celsius.trim().parse().expect("Please enter a valid number");

    let fahrenheit = (celsius * 9.0 / 5.0) + 32.0;
    println!("{celsius}°C is equal to {fahrenheit:.2}°F");
}