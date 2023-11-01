#[macro_export]
macro_rules! print_any {
    ($($x:expr),*) => {{
        let mut rng = rand::thread_rng();
        let messages: Vec<String> = vec![$($x.to_string()),*];
        println!("{}", messages.choose(&mut rng).unwrap_or(&"Hmhm".to_string()));
    }};
}

#[macro_export]
macro_rules! any_of {
    ($($x:expr),*) => {
        let mut rng = rand::thread_rng();
        let messages = &[$($x),*];
        format!("{}", messages.choose(&mut rng).unwrap_or(&"Hrr"))
    };
}