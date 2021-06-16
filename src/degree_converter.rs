pub mod mod_degree_converter {
    pub fn demo() {
        let celsius_i = 30;
        let fahrenheit_i = 30;
    
        println!("{}°C -> {}°F", celsius_i, celsius_to_fahrenheit(celsius_i));
    
        println!(
            "{}°F -> {}°C",
            fahrenheit_i,
            fahrenheit_to_celsius(fahrenheit_i)
        );
    }
    
    fn celsius_to_fahrenheit(celsius: i32) -> i32 {
        celsius / 5 * 9 + 32
    }
    
    fn fahrenheit_to_celsius(fahrenheit: i32) -> i32 {
        (fahrenheit - 32) * 5 / 9
    }
}