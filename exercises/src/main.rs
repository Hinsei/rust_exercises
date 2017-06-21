use std::io;

fn main() {
    loop {
        println!("Hi please choose what you want to do from the following");
        println!("1: F to C");
        println!("2: Fibbonaci");
        println!("3: Lyrics");

        let mut input = String::new();

        io::stdin().read_line(&mut input).expect("Failed to read line");

        let input: i32 = match input.trim().parse(){
            Ok(num) => num,
            Err(_) => continue,
        };

        if input == 1
        {
           println!("Type F for farenheit to celcius or type C for celcius to farenheit");
           let mut input_s = String::new();

           io::stdin().read_line(&mut input_s).expect("Failed to read string");

           println!("What is the number of the degree?");

           let mut input_i = String::new();

           io::stdin().read_line(&mut input_i).expect("Failed to read string");

           let input_i : i32 = match input_i.trim().parse(){
               Ok(num) => num,
               Err(_) => continue,
           };

           println!("The converted degree is {}", convert(input_i, input_s));
        } else {
            println!("Nope");
            break;
        }
    }
}

fn convert(metric: i32, degree: String) -> i32 {
   let number =
    if degree == "F" {
       celcius_to_farenheit(metric)
    } else {
       farenheit_to_celcius(metric)
    };
   number
}

fn farenheit_to_celcius(metric: i32) -> i32 {
    let conversion = (metric - 32) * 5 / 9;
    conversion
}

fn celcius_to_farenheit(metric: i32) -> i32 {
    let conversion = (metric * 9 / 5) + 32;
    conversion
}

