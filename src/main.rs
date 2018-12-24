use std::io;
use std::process;

fn main() {
    let mut choice: String = String::new();

    println!("Do you want to covert from Fahrenheit: 'f' or Celcius: 'c'?");
    io::stdin()
        .read_line(&mut choice)
        .expect("what are you doing?");

    choice = choice.trim().to_lowercase();

    if choice != "c" && choice != "f" {
        println!("What are you playing at?");
        process::exit(1);
    }

    println!("Enter the temperature then:");

    let mut temp = String::new();

    match io::stdin().read_line(&mut temp) {
        Ok(str) => str,
        Err(_) => {
            println!("something went wrong");
            process::exit(1);
        }
    };

    let temperature: i32 = temp.trim().parse::<i32>().unwrap();

    println!("{}", convert_temp(choice, temperature))
}

fn convert_temp(unit: String, temp: i32) -> i32 {
    if unit == "f" {
        to_celcius(temp)
    } else {
        to_fahrenheit(temp)
    }
}

fn to_celcius(fahrenheit: i32) -> i32 {
    let result: i32 = (fahrenheit - 32) * (5 / 2);
    result
}

fn to_fahrenheit(celcius: i32) -> i32 {
    let result: i32 = celcius * (9 / 5) - 32;
    result
}
