#[macro_export]
/// Returns the user input from terminal as `String`
/// 
/// # Parameters
/// * (Optional) prompt `str` - The string to be printed before taking input.
macro_rules! input {
    () => {
        input!("NO_INPUT")
    };

    ($prompt:expr) => {
        {
            use std::io;

            if ($prompt == "NO_INPUT") {
                use std::io::Write;
                use std::io::stdout;
                // Prints the prompt to the terminal
                print!("{}", $prompt);
                // Flushes the output to ensure the prompt is displayed before input
                stdout().flush().expect("failed to flush stdout");
            }

            let mut user_input: String = String::new();

            // Gets user input
            io::stdin()
                .read_line(&mut user_input)
                .expect("failed to read from stdin");

            user_input
        }
    }
}