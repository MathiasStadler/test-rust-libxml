//from here
//https://www.educative.io/answers/how-to-use-references-in-rust

fn main() {
    let mut x = 10;
    let mut y = 15;
  
    let ref1 = &x;
    read_value(ref1); // Lifetime of ref1 ends here
  
    let ref2 = &mut x;
    change_value(ref2);
    
    println!("y= {}",y);
    y= *ref2;
    println!("y= {}",y);
    // Print new value of 'x':
    println!("New value of x = {}", *ref2);
  }
  
  fn read_value(i: &i8) {
    // Use immutable reference 'i':
    println!("read value x = {}", *i);
  }
  
  fn change_value(i: &mut i8) {
    // Use mutable reference 'i':
    *i = 100  ;
  }