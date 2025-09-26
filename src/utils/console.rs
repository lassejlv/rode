use rusty_v8 as v8;

pub fn console_log(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    let parts: Vec<String> = (0..args.length())
        .map(|i| {
            let arg = args.get(i);
            arg.to_string(scope)
                .map(|s| s.to_rust_string_lossy(scope))
                .unwrap_or_else(|| "undefined".to_string())
        })
        .collect();

    println!("{}", parts.join(" "));
}

pub fn setup_console(scope: &mut v8::HandleScope) {
    let console_obj = v8::Object::new(scope);
    let console_key = v8::String::new(scope, "console").unwrap();

    let log_key = v8::String::new(scope, "log").unwrap();
    let log_func = v8::Function::new(scope, console_log).unwrap();
    console_obj.set(scope, log_key.into(), log_func.into());

    let global = scope.get_current_context().global(scope);
    global.set(scope, console_key.into(), console_obj.into());
}
