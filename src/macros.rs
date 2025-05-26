#[macro_export]
/// Returns the user input from terminal as `String`
macro_rules! input {
    () => {
        {
            use std::io;

            let mut user_input: String = String::new();

            // Gets user input
            io::stdin()
                .read_line(&mut user_input)
                .expect("failed to read from stdin");

            user_input
        }
    }
}