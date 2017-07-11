// Vec<T> A collection used to store more than one value in a single data structure that places all
// values next to each other in memory
// Vec<T> can only store values of the same type
fn main() {
    why_two_different_ways_to_read_vectors();
}

#[cfg(test)]
fn create_new_vector() {
    let vector : Vec<i32> = Vec::new();
    let another_vector = vec![1, 2, 3];
}

#[cfg(test)]
fn update_a_vector() {
    let mut v = Vec::new();

    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);
    v.push(9);
}


#[cfg(test)]
fn droping_a_vector() {
    //in scope
    let vector = vec![1,2,3,4];
}
// goes out of scope


#[cfg(test)]
fn reading_elements_of_vector() {
    let vector = vec![1,2,3];

    let third: &i32 = &vector[2];
    let third: Option<&i32> = vector.get(2);
}

fn why_two_different_ways_to_read_vectors() {
    let vector = vec![1,2,3];

    //let wrong: &i32 = &vector[100];
    let dun_break: Option<&i32> = vector.get(100);

    println!("{}", i_am_option_enum( dun_break ));
}

fn i_am_option_enum( x: Option<&i32> ) -> String {
    match x {
        None => String::from("I am none"),
        _ => String::from("I am some"),
    }
}


#[cfg(test)]
enum Random {
    Int(i32),
    Word(String),
    Confirmation(bool),
}


#[cfg(test)]
fn making_a_vector_of_multiple_things() {

    let row = vec![
        Random::Int(3),
        Random::Word(String::from("hi")),
        Random:: Confirmation(true),
    ];
}
