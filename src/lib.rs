use std::io::Write;

#[macro_export]
macro_rules! input {
    () => {
        {
            let mut input = std::string::String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        }
    };
    ($($arg:tt)*) => {
        {
            std::print!($($arg)*);
            std::io::stdout().flush().unwrap();
            let mut input = std::string::String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        }
    };
}
#[macro_export]
macro_rules! inputln {
    () => {
        {
            input!()
        }
    };
    ($($arg:tt)*) => {
        {
            std::println!($($arg)*);
            std::io::stdout().flush().unwrap();
            let mut input = std::string::String::new();
            std::io::stdin().read_line(&mut input).unwrap();
            input
        }
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let input = input!("test: ");
        println!("{}", input);
        assert!(true);
    }
}
