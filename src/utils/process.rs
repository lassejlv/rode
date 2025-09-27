use rusty_v8 as v8;
use std::env;
use std::process;

pub fn setup_process(scope: &mut v8::HandleScope) {
    let global = scope.get_current_context().global(scope);

    // Get or create Rode object
    let rode_key = v8::String::new(scope, "Rode").unwrap();
    let rode_obj = if let Some(existing) = global.get(scope, rode_key.into()) {
        existing.to_object(scope).unwrap()
    } else {
        let new_obj = v8::Object::new(scope);
        global.set(scope, rode_key.into(), new_obj.into());
        new_obj
    };

    // Rode.exit(code?) - Exit the process with optional exit code
    let exit_key = v8::String::new(scope, "exit").unwrap();
    let exit_func = v8::Function::new(scope, rode_exit).unwrap();
    rode_obj.set(scope, exit_key.into(), exit_func.into());

    // Rode.args - Array of command line arguments (excluding program name)
    let args: Vec<String> = env::args().skip(1).collect();
    let args_array = v8::Array::new(scope, args.len() as i32);
    for (i, arg) in args.iter().enumerate() {
        let index = v8::Number::new(scope, i as f64);
        let arg_str = v8::String::new(scope, arg).unwrap();
        args_array.set(scope, index.into(), arg_str.into());
    }
    let args_key = v8::String::new(scope, "args").unwrap();
    rode_obj.set(scope, args_key.into(), args_array.into());

    // Rode.argv - Array of all command line arguments (including program name)
    let argv: Vec<String> = env::args().collect();
    let argv_array = v8::Array::new(scope, argv.len() as i32);
    for (i, arg) in argv.iter().enumerate() {
        let index = v8::Number::new(scope, i as f64);
        let arg_str = v8::String::new(scope, arg).unwrap();
        argv_array.set(scope, index.into(), arg_str.into());
    }
    let argv_key = v8::String::new(scope, "argv").unwrap();
    rode_obj.set(scope, argv_key.into(), argv_array.into());

    // Rode.env - Object containing environment variables
    let env_obj = v8::Object::new(scope);
    for (key, value) in env::vars() {
        let env_key = v8::String::new(scope, &key).unwrap();
        let env_value = v8::String::new(scope, &value).unwrap();
        env_obj.set(scope, env_key.into(), env_value.into());
    }
    let env_key = v8::String::new(scope, "env").unwrap();
    rode_obj.set(scope, env_key.into(), env_obj.into());
}

fn rode_exit(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    // Get exit code (default: 0)
    let exit_code = if args.length() >= 1 {
        args.get(0).int32_value(scope).unwrap_or(0)
    } else {
        0
    };

    // Exit the process
    process::exit(exit_code);
}
