fn main() {   
    let s1: String = String::from("Mathias dud dudoof");
    let s_me: String = String::from("Mathias dud dudoof");
    let (s2, len) = calculate_length(s1);
    let (s2_me, len) = calculate_length(s_me);
    println!("The length of '{}' is {}.", s2, len);
    println!("The length of '{}' is {}.", s2_me, len);

    let (s2_me1, len) = calculate_me(s2_me);
    println!("The length of '{}' is {}.", s2_me1, len);
    (s2_me1, len) = calculate_me(s2_me.clone());
    println!("The length of '{}' is {}.", s2_me1, len);

 }
 
 fn calculate_length(s: String) -> (String, usize) {
    let length = s.len();
    // return both the string and length
    (s, length)
 }

 fn calculate_me(s: String) -> (String, usize) {
    let length = s.len();
    // return both the string and length
    (s, length)
 }

