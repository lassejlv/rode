use rusty_v8 as v8;
use std::fs;
use std::path::PathBuf;

pub fn setup_module_system(scope: &mut v8::HandleScope) {
    // Add a simple require function for basic module loading
    let global = scope.get_current_context().global(scope);

    let require_key = v8::String::new(scope, "require").unwrap();
    let require_func = v8::Function::new(scope, module_require).unwrap();
    global.set(scope, require_key.into(), require_func.into());
}

fn module_require(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "require() requires a module path").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let module_path = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid module path").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    // Resolve module path
    let current_dir = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let mut resolved_path = if module_path.starts_with("./") || module_path.starts_with("../") {
        // For relative paths, resolve relative to current directory
        current_dir.join(&module_path)
    } else {
        // For absolute module names, look in current directory
        current_dir.join(&module_path)
    };

    // Add .js extension if not present
    if resolved_path.extension().is_none() {
        resolved_path.set_extension("js");
    }

    let source = match fs::read_to_string(&resolved_path) {
        Ok(content) => content,
        Err(_) => {
            let error_msg = format!("Module not found: {}", module_path);
            let error = v8::String::new(scope, &error_msg).unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    // Transform ES6 imports/exports to CommonJS
    let transformed_source = if module_path.ends_with(".js") {
        crate::typescript::convert_es6_imports(&source)
    } else if crate::typescript::is_typescript_file(&module_path) {
        crate::typescript::strip_typescript(&source)
    } else {
        transform_module_source(&source)
    };

    // Execute the module and return its exports
    let wrapped_source = format!(
        r#"
        (function() {{
            const module = {{ exports: {{}} }};
            const exports = module.exports;
            {}
            return module.exports;
        }})()
        "#,
        transformed_source
    );

    let code = v8::String::new(scope, &wrapped_source).unwrap();
    if let Some(script) = v8::Script::compile(scope, code, None) {
        if let Some(result) = script.run(scope) {
            rv.set(result);
        }
    }
}

fn transform_module_source(source: &str) -> String {
    let mut lines: Vec<String> = source.lines().map(|s| s.to_string()).collect();
    let mut exports = Vec::new();

    // Process exports
    for line in &mut lines {
        if line.trim().starts_with("export ") {
            if line.contains("export const ")
                || line.contains("export let ")
                || line.contains("export var ")
            {
                // export const name = value
                let parts: Vec<&str> = line.split('=').collect();
                if parts.len() >= 2 {
                    let declaration = parts[0].trim();
                    let name = declaration.split_whitespace().last().unwrap_or("");
                    if !name.is_empty() {
                        exports.push(name.to_string());
                    }
                }
                // Remove 'export ' from the beginning
                *line = line[7..].to_string();
            } else if line.contains("export function ") {
                // export function name() {}
                if let Some(start) = line.find("function ") {
                    let after_function = &line[start + 9..];
                    if let Some(paren_pos) = after_function.find('(') {
                        let name = after_function[..paren_pos].trim();
                        if !name.is_empty() {
                            exports.push(name.to_string());
                        }
                    }
                }
                // Remove 'export ' from the beginning
                *line = line[7..].to_string();
            } else if line.contains("export default ") {
                // export default value
                exports.push("default".to_string());
                *line = line.replace("export default ", "const __default = ");
            }
        }
    }

    // Add module exports at the end
    if !exports.is_empty() {
        lines.push("".to_string());
        lines.push("// Module exports".to_string());
        for export in &exports {
            if export == "default" {
                lines.push("module.exports.default = __default;".to_string());
            } else {
                lines.push(format!("module.exports.{} = {};", export, export));
            }
        }
    }

    lines.join("\n")
}
