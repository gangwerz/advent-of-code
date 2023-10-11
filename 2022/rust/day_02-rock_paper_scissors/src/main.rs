use std::collections::HashMap;


fn main() { 
    let mut col_1 = HashMap::new();
    let mut col_2 = HashMap::new();

    col_1.insert(String::from("A"), String::from("Rock"));
    col_1.insert(String::from("B"), String::from("Paper"));
    col_1.insert(String::from("C"), String::from("Scissors"));

    col_2.insert(String::from("X"), String::from("Rock"));
    col_2.insert(String::from("Y"), String::from("Paper"));
    col_2.insert(String::from("Z"), String::from("Scissors"));
}
