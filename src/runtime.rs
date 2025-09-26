use rusty_v8 as v8;
use std::sync::Once;

static INIT: Once = Once::new();

pub struct Runtime {
    isolate: v8::OwnedIsolate,
}

impl Runtime {
    pub fn new() -> Self {
        INIT.call_once(|| {
            let platform = v8::new_default_platform(0, false).make_shared();
            v8::V8::initialize_platform(platform);
            v8::V8::initialize();
        });

        let isolate = v8::Isolate::new(Default::default());
        Self { isolate }
    }

    // pub fn execute(&mut self, code: &str) -> Result<(), String> {
    //     self.execute_with_filename(code, "script.js")
    // }

    pub fn execute_with_filename(&mut self, code: &str, filename: &str) -> Result<(), String> {
        let scope = &mut v8::HandleScope::new(&mut self.isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        crate::utils::setup_console(scope);
        crate::utils::setup_fs(scope);
        crate::utils::setup_http(scope);
        crate::utils::setup_path(scope);
        crate::modules::setup_module_system(scope);

        let code_str = v8::String::new(scope, code).unwrap();
        let filename_str = v8::String::new(scope, filename).unwrap();
        let source_map_url = v8::undefined(scope).into();
        let origin = v8::ScriptOrigin::new(
            scope,
            filename_str.into(),
            0,
            0,
            false,
            0,
            source_map_url,
            false,
            false,
            false,
        );

        let mut try_catch = v8::TryCatch::new(scope);
        let script = match v8::Script::compile(&mut try_catch, code_str, Some(&origin)) {
            Some(script) => script,
            None => {
                if let Some(exception) = try_catch.exception() {
                    let exception_str = exception.to_rust_string_lossy(&mut try_catch);
                    return Err(Self::format_error(&exception_str, code, filename));
                }
                return Err("Failed to compile script".to_string());
            }
        };

        match script.run(&mut try_catch) {
            Some(_) => Ok(()),
            None => {
                if let Some(exception) = try_catch.exception() {
                    let exception_str = exception.to_rust_string_lossy(&mut try_catch);
                    Err(Self::format_error(&exception_str, code, filename))
                } else {
                    Err("Script execution failed".to_string())
                }
            }
        }
    }

    fn format_error(error: &str, source_code: &str, filename: &str) -> String {
        // Parse the error to extract line number and message
        let error_line = Self::find_syntax_error_line(source_code, error);
        if let Some((_, message)) = Self::parse_v8_error(error) {
            let lines: Vec<&str> = source_code.lines().collect();
            let mut result = String::new();

            result.push_str(&format!("\n{}\n", message));
            result.push_str(&format!("    at {}:{}\n\n", filename, error_line));

            // Show context around the error line
            let start_line = if error_line > 2 { error_line - 2 } else { 1 };
            let end_line = std::cmp::min(error_line + 2, lines.len());

            for i in start_line..=end_line {
                if i <= lines.len() {
                    let line_content = if i == 0 { "" } else { lines[i - 1] };
                    let line_indicator = if i == error_line { ">" } else { " " };
                    result.push_str(&format!(
                        "  {} {:3} | {}\n",
                        line_indicator, i, line_content
                    ));

                    // Add arrow pointer for the error line
                    if i == error_line {
                        let indent = "      | ";
                        let pointer = if line_content.trim().is_empty() {
                            "^".to_string()
                        } else {
                            "^".repeat(line_content.trim_start().len().max(1))
                        };
                        result.push_str(&format!("{}{}\n", indent, pointer));
                    }
                }
            }

            result
        } else {
            format!("Runtime error: {}", error)
        }
    }

    fn parse_v8_error(error: &str) -> Option<(usize, String)> {
        // Just return the error message without line parsing for now
        // We'll let find_syntax_error_line handle the detection
        Some((1, error.to_string()))
    }

    fn find_syntax_error_line(source_code: &str, error: &str) -> usize {
        let lines: Vec<&str> = source_code.lines().collect();

        // Look for common syntax issues
        if error.contains("Unexpected token") {
            // Look for unclosed braces
            let mut brace_count = 0;
            for (i, line) in lines.iter().enumerate() {
                for ch in line.chars() {
                    match ch {
                        '{' => brace_count += 1,
                        '}' => brace_count -= 1,
                        _ => {}
                    }
                }

                // If we have an unmatched opening brace, the error is likely on the next meaningful line
                if brace_count > 0 && i + 1 < lines.len() {
                    let next_line = lines[i + 1].trim();
                    if !next_line.is_empty()
                        && (next_line.starts_with("return")
                            || next_line.starts_with("}")
                            || next_line.contains("const")
                            || next_line.contains("function"))
                    {
                        return i + 2; // Return line number (1-indexed)
                    }
                }
            }
        }

        // Default to line 1 if we can't detect
        1
    }
}
