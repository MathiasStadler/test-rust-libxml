fn main() {
    let mut name = String::from("Matt");
    println!("Name =>  {}",name);
    let _s = change_name(&mut name);
    println!("Name =>  {}",name);
 }
 
 fn change_name(s: &mut String) {
    s.push_str(" Bidewell");
    
 }