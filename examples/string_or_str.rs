fn print_me(msg: &str) { println!("msg = {}", msg); }

// In this function the `'static` specifier is *not* required, see next function
fn baz(_: &str) -> &'static str {
    "baz"
}

//from here
//http://xion.io/post/code/rust-string-args.html
fn hello(name: &str) {
    println!("Hello, {}!", name);
}

fn hello_string(name: &String) {
    println!("Hello, {}!", name);
}

fn hello_as_ref<N: AsRef<str>>(name: N) {
    println!("Hello, {}!", name.as_ref());
}

fn main() {
    let string: &str = "hello world";
    print_me(string);

    let owned_string: String = "hello world".to_string(); // or String::from_str("hello world")
    print_me(&owned_string);

    let counted_string: std::rc::Rc<String> = std::rc::Rc::new("hello world".to_string());
    print_me(&counted_string);

    let atomically_counted_string: std::sync::Arc<String> = std::sync::Arc::new("hello world".to_string());
    print_me(&atomically_counted_string);

    println!("{} ",baz("Hallo"));

    hello("world");
    hello(&String::from("Alice"));
    hello(&"Dennis Ritchie"[0..]);
    
    hello_string(&String::from("Bob"));
    hello_as_ref(&String::from("Bob"))
    }
