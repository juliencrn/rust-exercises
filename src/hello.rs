pub mod mod_hello {
    use std::io;

    pub fn demo() {
        let name = String::from("Julien");

        hello_world();
        hello_by_name(&name);
        hello_io();

        // Possession OK
        println!("var name after fn: {}", name);
    }

    fn hello_world() {
        println!("Hello, world!");
    }

    // Use ref because we don't want keep possession
    fn hello_by_name(name: &String) {
        println!("Hello {}!", name);
    }

    fn hello_io() {
        let mut input = String::new();

        println!("Hi! What's your name?");

        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                let name = input.trim();

                if name != "" {
                    println!("Hello {}!", name)
                } else {
                    println!("Hello anonym")
                }
            },
            Err(_) => println!("Hello anonym")
        }
    }
}