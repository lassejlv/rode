use rusty_v8 as v8;
use std::io::{self, Write};

pub fn setup_prompt(scope: &mut v8::HandleScope) {
    let global = scope.get_current_context().global(scope);

    // Add prompt function to global scope
    let prompt_key = v8::String::new(scope, "prompt").unwrap();
    let prompt_func = v8::Function::new(scope, prompt_function).unwrap();
    global.set(scope, prompt_key.into(), prompt_func.into());

    // Add alert function to global scope
    let alert_key = v8::String::new(scope, "alert").unwrap();
    let alert_func = v8::Function::new(scope, alert_function).unwrap();
    global.set(scope, alert_key.into(), alert_func.into());
}

fn prompt_function(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Get the message (default: empty string)
    let message = if args.length() >= 1 {
        match args.get(0).to_string(scope) {
            Some(s) => s.to_rust_string_lossy(scope),
            None => String::new(),
        }
    } else {
        String::new()
    };

    // Get the default value (optional)
    let default_value = if args.length() >= 2 {
        match args.get(1).to_string(scope) {
            Some(s) => Some(s.to_rust_string_lossy(scope)),
            None => None,
        }
    } else {
        None
    };

    // Display the prompt
    if !message.is_empty() {
        print!("{}", message);
        if let Some(ref default) = default_value {
            print!(" [{}]", default);
        }
        print!(": ");
    } else {
        print!("> ");
    }

    // Flush stdout to ensure prompt is displayed
    if let Err(_) = io::stdout().flush() {
        let error = v8::String::new(scope, "Failed to flush stdout").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Remove trailing newline
            input = input.trim_end().to_string();

            // Use default value if input is empty and default is provided
            if input.is_empty() && default_value.is_some() {
                input = default_value.unwrap();
            }

            // Return the input as a string
            let result_str = v8::String::new(scope, &input).unwrap();
            rv.set(result_str.into());
        }
        Err(err) => {
            let error_msg = format!("Failed to read input: {}", err);
            let error = v8::String::new(scope, &error_msg).unwrap();
            scope.throw_exception(error.into());
        }
    }
}

fn alert_function(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Get the message (default: empty string)
    let message = if args.length() >= 1 {
        match args.get(0).to_string(scope) {
            Some(s) => s.to_rust_string_lossy(scope),
            None => String::new(),
        }
    } else {
        String::new()
    };

    // Display the alert message
    if !message.is_empty() {
        print!("{} ", message);
    }
    print!("(Y/n): ");

    // Flush stdout to ensure prompt is displayed
    if let Err(_) = io::stdout().flush() {
        let error = v8::String::new(scope, "Failed to flush stdout").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    // Read user input
    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            // Remove trailing newline and convert to lowercase
            let input = input.trim().to_lowercase();

            // Default to true (Y) if empty, false only if explicitly n/no
            let result = match input.as_str() {
                "n" | "no" | "false" => false,
                _ => true, // Default to true for empty input, "y", "yes", "true", or any other input
            };

            let result_bool = v8::Boolean::new(scope, result);
            rv.set(result_bool.into());
        }
        Err(err) => {
            let error_msg = format!("Failed to read input: {}", err);
            let error = v8::String::new(scope, &error_msg).unwrap();
            scope.throw_exception(error.into());
        }
    }
}
