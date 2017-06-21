fn main() {
    println!("Hello, world!");

    another_function(3);
    
    statements_and_expressions();

    println!("{}", functions_with_return_values());
}

fn another_function(x : u32) {
    println!("Argument: {}", x);
}

fn statements_and_expressions() {
    let x = 5;

    let y = {
        let x = 3;
        x + 1
    };

    println!("{}", y);
}

fn functions_with_return_values() -> u32 {
    7 //this is an expression a value is returned
    //7; this is a statement a value will not be returned
}
