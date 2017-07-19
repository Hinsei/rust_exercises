// str is a string slice &str being its borrowed form
// string literals are stored in the binary output of the program hence there are considered to be
// string slices
//
// String type is growable, mutable, owned, UTF-8 endcoded string slice
//
fn main() {

    utf_things();
}

fn creating_a_new_string(){
    //operations normally available with Vectors are available to String


    //this creates a new empty string which we can load data into
    let s = String::new();

    //this creates a string with a value
    let s_1 = "i am a string literal".to_string();

    let s_2 = String::from("i am a string literal");
}

fn update(){

    // use push

    let mut s = String::from("po");
    s.push_str("tato");

    let mut s1 = String::from("po");
    let s2 = String::from("tato");
    s1.push_str(&s2);
    
    let add_s1 = String::from("Hello");
    let add_s2 = String::from("world");
    let add_s3 = s1 + &s2; //???????????????

}

// example_add(self, s: &str) -> String{
// }


fn what(){
    let s1 = String::from("po");
    let s2 = String::from("ta");
    let s3 = String::from("to");

//    let s_add = s1 + "-" + &s2 + "-" + &s3;

    let s = format!("{}-{}-{}", s1, s2, s3);
}

fn forbidden() {
    let s1 = String::from("hello");
//    let h = s1[0];
}

fn utf_things() {
    let len1 = String::from("hi").len();

    let len2 = String::from("Здравствуйте").len();

    println!("Length of len 1 is {}", len1);
    println!("Length of len 2 is {}", len2);
}

