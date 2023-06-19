
//from here
//https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env;

fn main() {
    
    read_args();
    
}

fn read_args() {
    let args: Vec<String> = env::args().collect();
    
    let query: &String ;
    
    if args.len() > 1 {

    query = &args[1];
    

    println!("First argument (fn) =>  {}", query);
    }
}
