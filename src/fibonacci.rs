pub mod mod_fibonacci {
    /*
    0 1 2 3 4 5 6  7  8  9 10
    0 1 1 2 3 5 8 13 21 34 55
    */
    fn fibonacci(number: u32) -> u32 {
        if number <= 1 {
            return number;
        }
        let mut a = 0;
        let mut b = 1;
        let mut result = 0;
    
        for _n in 1..number {
            result = a + b;
            a = b;
            b = result;
        }
    
        result
    }

    pub fn print_fibonacci() {
        for n in 0..11 {
            println!("{}th number of Fibonacci -> {}", n, fibonacci(n));
        }
    }
}
