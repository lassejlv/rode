use colored::*;
use rusty_v8 as v8;

fn format_value(scope: &mut v8::HandleScope, value: v8::Local<v8::Value>) -> String {
    if value.is_string() {
        value
            .to_string(scope)
            .map(|s| s.to_rust_string_lossy(scope))
            .unwrap_or_else(|| "undefined".to_string())
    } else if value.is_number() {
        value
            .to_string(scope)
            .map(|s| s.to_rust_string_lossy(scope))
            .unwrap_or_else(|| "NaN".to_string())
    } else if value.is_boolean() {
        if value.boolean_value(scope) {
            "true".to_string()
        } else {
            "false".to_string()
        }
    } else if value.is_null() {
        "null".to_string()
    } else if value.is_undefined() {
        "undefined".to_string()
    } else if value.is_object() {
        // Try to stringify the object
        let global = scope.get_current_context().global(scope);
        let json_key = v8::String::new(scope, "JSON").unwrap();
        if let Some(json_obj) = global
            .get(scope, json_key.into())
            .and_then(|v| v.to_object(scope))
        {
            let stringify_key = v8::String::new(scope, "stringify").unwrap();
            if let Some(stringify_func) = json_obj
                .get(scope, stringify_key.into())
                .and_then(|v| v8::Local::<v8::Function>::try_from(v).ok())
            {
                let args = &[value];
                if let Some(result) = stringify_func.call(scope, json_obj.into(), args) {
                    return result
                        .to_string(scope)
                        .map(|s| s.to_rust_string_lossy(scope))
                        .unwrap_or_else(|| "[object Object]".to_string());
                }
            }
        }
        "[object Object]".to_string()
    } else {
        value
            .to_string(scope)
            .map(|s| s.to_rust_string_lossy(scope))
            .unwrap_or_else(|| "unknown".to_string())
    }
}

fn format_table_data(scope: &mut v8::HandleScope, value: v8::Local<v8::Value>) -> String {
    if !value.is_object() {
        return format_value(scope, value);
    }

    let obj = value.to_object(scope).unwrap();

    // Check if it's an array
    if obj.is_array() {
        let array = v8::Local::<v8::Array>::try_from(value).unwrap();
        let length = array.length();

        let mut output = String::new();
        output.push_str("┌─────────┬────────────────────────────────────────┐\n");
        output.push_str("│ (index) │                Values                  │\n");
        output.push_str("├─────────┼────────────────────────────────────────┤\n");

        for i in 0..length.min(50) {
            // Limit to 50 items
            let index = v8::Number::new(scope, i as f64);
            if let Some(item) = array.get(scope, index.into()) {
                let value_str = format_value(scope, item);
                let truncated = if value_str.len() > 38 {
                    format!("{}...", &value_str[..35])
                } else {
                    value_str
                };
                output.push_str(&format!("│{:>8} │ {:<38} │\n", i, truncated));
            }
        }

        if length > 50 {
            output.push_str(&format!(
                "│   ...   │ ... {} more items                    │\n",
                length - 50
            ));
        }

        output.push_str("└─────────┴────────────────────────────────────────┘");
        return output;
    }

    // Handle objects
    let mut output = String::new();
    output.push_str("┌─────────┬────────────────────────────────────────┐\n");
    output.push_str("│ (index) │                Values                  │\n");
    output.push_str("├─────────┼────────────────────────────────────────┤\n");

    let property_names = obj.get_own_property_names(scope);
    if let Some(names) = property_names {
        let length = names.length().min(50); // Limit to 50 properties

        for i in 0..length {
            let index = v8::Number::new(scope, i as f64);
            if let Some(key) = names.get(scope, index.into()) {
                let key_str = format_value(scope, key);
                if let Some(value) = obj.get(scope, key) {
                    let value_str = format_value(scope, value);
                    let truncated = if value_str.len() > 38 {
                        format!("{}...", &value_str[..35])
                    } else {
                        value_str
                    };
                    let key_truncated = if key_str.len() > 7 {
                        format!("{}...", &key_str[..4])
                    } else {
                        key_str
                    };
                    output.push_str(&format!("│{:>8} │ {:<38} │\n", key_truncated, truncated));
                }
            }
        }

        if names.length() > 50 {
            output.push_str(&format!(
                "│   ...   │ ... {} more properties              │\n",
                names.length() - 50
            ));
        }
    }

    output.push_str("└─────────┴────────────────────────────────────────┘");
    output
}

pub fn console_log(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let parts: Vec<String> = (0..args.length())
        .map(|i| format_value(scope, args.get(i)))
        .collect();

    println!("{}", parts.join(" "));
}

pub fn console_error(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let parts: Vec<String> = (0..args.length())
        .map(|i| format_value(scope, args.get(i)))
        .collect();

    eprintln!("{}", parts.join(" ").red());
}

pub fn console_warn(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let parts: Vec<String> = (0..args.length())
        .map(|i| format_value(scope, args.get(i)))
        .collect();

    println!("{}", parts.join(" ").yellow());
}

pub fn console_info(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let parts: Vec<String> = (0..args.length())
        .map(|i| format_value(scope, args.get(i)))
        .collect();

    println!("{}", parts.join(" ").blue());
}

pub fn console_table(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() == 0 {
        println!("undefined");
        return;
    }

    let data = args.get(0);
    let table_str = format_table_data(scope, data);
    println!("{}", table_str);
}

pub fn console_dir(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() == 0 {
        println!("undefined");
        return;
    }

    let value = args.get(0);
    let formatted = format_value(scope, value);
    println!("{}", formatted.cyan());
}

pub fn console_clear(
    _scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    print!("\x1B[2J\x1B[1;1H"); // ANSI escape codes to clear screen and move cursor to top
}

pub fn console_count(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let label = if args.length() > 0 {
        format_value(scope, args.get(0))
    } else {
        "default".to_string()
    };

    // Simple counter - in a real implementation you'd want to store state
    println!("{}: 1", label);
}

pub fn console_time(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let label = if args.length() > 0 {
        format_value(scope, args.get(0))
    } else {
        "default".to_string()
    };

    println!("Timer '{}' started", label);
}

pub fn console_time_end(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let label = if args.length() > 0 {
        format_value(scope, args.get(0))
    } else {
        "default".to_string()
    };

    // In a real implementation, you'd calculate the actual time difference
    println!("{}: 0.000ms", label);
}

pub fn setup_console(scope: &mut v8::HandleScope) {
    let console_obj = v8::Object::new(scope);
    let console_key = v8::String::new(scope, "console").unwrap();

    // console.log
    let log_key = v8::String::new(scope, "log").unwrap();
    let log_func = v8::Function::new(scope, console_log).unwrap();
    console_obj.set(scope, log_key.into(), log_func.into());

    // console.error
    let error_key = v8::String::new(scope, "error").unwrap();
    let error_func = v8::Function::new(scope, console_error).unwrap();
    console_obj.set(scope, error_key.into(), error_func.into());

    // console.warn
    let warn_key = v8::String::new(scope, "warn").unwrap();
    let warn_func = v8::Function::new(scope, console_warn).unwrap();
    console_obj.set(scope, warn_key.into(), warn_func.into());

    // console.info
    let info_key = v8::String::new(scope, "info").unwrap();
    let info_func = v8::Function::new(scope, console_info).unwrap();
    console_obj.set(scope, info_key.into(), info_func.into());

    // console.table
    let table_key = v8::String::new(scope, "table").unwrap();
    let table_func = v8::Function::new(scope, console_table).unwrap();
    console_obj.set(scope, table_key.into(), table_func.into());

    // console.dir
    let dir_key = v8::String::new(scope, "dir").unwrap();
    let dir_func = v8::Function::new(scope, console_dir).unwrap();
    console_obj.set(scope, dir_key.into(), dir_func.into());

    // console.clear
    let clear_key = v8::String::new(scope, "clear").unwrap();
    let clear_func = v8::Function::new(scope, console_clear).unwrap();
    console_obj.set(scope, clear_key.into(), clear_func.into());

    // console.count
    let count_key = v8::String::new(scope, "count").unwrap();
    let count_func = v8::Function::new(scope, console_count).unwrap();
    console_obj.set(scope, count_key.into(), count_func.into());

    // console.time
    let time_key = v8::String::new(scope, "time").unwrap();
    let time_func = v8::Function::new(scope, console_time).unwrap();
    console_obj.set(scope, time_key.into(), time_func.into());

    // console.timeEnd
    let time_end_key = v8::String::new(scope, "timeEnd").unwrap();
    let time_end_func = v8::Function::new(scope, console_time_end).unwrap();
    console_obj.set(scope, time_end_key.into(), time_end_func.into());

    let global = scope.get_current_context().global(scope);
    global.set(scope, console_key.into(), console_obj.into());
}
