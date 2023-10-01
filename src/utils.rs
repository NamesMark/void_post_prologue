macro_rules! print_any {
    ($($x:expr),*) => {
        {
            let mut rng = rand::thread_rng();
            let messages = &[$($x),*];
            println!("{}", messages.choose(&mut rng).unwrap_or(&"Default message"));
        }
    };
}
