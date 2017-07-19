//HashMap  collection that uses keys instead of index
//Least often used among all collections
fn main() {
}

fn create_hash_map() {
    
    use std::collections::HashMap;

    let mut scores = HashMap::new();

    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);
}

//Hash Maps have similar behavior as vectors keys must be of same type and values must be of same
//type

fn create_hash_with_collect() {
    use std::collections::HashMap;

    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let score = cev![10. 50];

    let scores: HashMap<_, _> = teams.iter().zip(initiial_scores.iter()).collect();
}

// HashMap<_,_> kinda like a wildcard when you dont know what the types would be initialy

fn hash_ownership() {
    use std::collections::HashMap;

    let field = String::from("fruit");
    let value = String::from("pineapple");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
}

// types with copy trait are fine owned values are moved into the hash so field and value would be
// invalid
