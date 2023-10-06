use std::io;


fn main() {
    println!("Your name?");

    let mut name = String::new();

    // io::stdin().read_line(&mut name);    //  will yell - not managed Result
    // let _ = io::stdin().read_line(&mut name);    //  hack to ignore error
    // io::stdin().read_line(&mut name).expect("Failed to read line");
    //  OR - matching

    match io::stdin().read_line(&mut name) {
        Ok(_) => {}
        Err(e) => {
            println!("Failed to read {}", e);
            return;
        }
    }

    let name = name.trim(); // String -> &str - string slice (std. lib) ~ "view"

    println!("Hello, {}!", name);

}
