
//from here
//https://doc.rust-lang.org/book/ch12-01-accepting-command-line-arguments.html
use std::env;
use std::num::ParseIntError;

fn main() -> Result<(), ParseIntError> {
    
    read_args();

 Ok(())   
}

fn read_args() -> String {
    let args: Vec<String> = env::args().collect() ;
    
    let query: &String ;
    
    if args.len() > 1 {

    query = &args[1];
    

    println!("First argument (fn) =>  {}", query);
    }

    return query;
    
}
