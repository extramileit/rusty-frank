use std::io;

fn main() {
    let mut input: String = Default::default();

    match io::stdin().read_line(&mut input) {
        Ok(n) => {
            println!("{} bytes read", n);
            println!("{}", input);
        }
        Err(error) => println!("error: {error}")
    }
}