fn main() {
    println!("{}", if_with_let());
}

fn if_with_let() -> i32 {
    let condition = true;

    let x = if condition {
        5
    } else {
        6
    };

    x
}
