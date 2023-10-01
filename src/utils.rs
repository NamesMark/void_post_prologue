use rand::seq::SliceRandom;

#[macro_export]
macro_rules! print_any {
    ($($x:expr),*) => {
        let mut rng = rand::thread_rng();
        let messages = &[$($x),*];
        println!("{}", messages.choose(&mut rng).unwrap_or(&"Hmhm"));
    };
}

#[macro_export]
macro_rules! any_of {
    ($($x:expr),*) => {
        let mut rng = rand::thread_rng();
        let messages = &[$($x),*];
        format!("{}", messages.choose(&mut rng).unwrap_or(&"Hrr"))
    };
}