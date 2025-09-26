/// Fast TypeScript stripper - removes TypeScript syntax to get pure JavaScript
/// This is a simple regex-based approach for common TypeScript patterns
use regex::Regex;
use std::sync::OnceLock;

static GENERIC_REGEX: OnceLock<Regex> = OnceLock::new();
static AS_TYPE_REGEX: OnceLock<Regex> = OnceLock::new();

pub fn strip_typescript(source: &str) -> String {
    let lines: Vec<&str> = source.lines().collect();
    let mut result_lines = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Skip interface declarations
        if line.starts_with("interface ") {
            i += 1;
            let mut brace_count = 0;
            let mut found_opening = false;

            // Skip until we find the closing brace
            while i < lines.len() {
                let current_line = lines[i];
                for ch in current_line.chars() {
                    if ch == '{' {
                        found_opening = true;
                        brace_count += 1;
                    } else if ch == '}' && found_opening {
                        brace_count -= 1;
                        if brace_count == 0 {
                            i += 1;
                            break;
                        }
                    }
                }
                if brace_count == 0 && found_opening {
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Skip type alias declarations
        if line.starts_with("type ") && line.contains("=") {
            i += 1;
            continue;
        }

        // Skip enum declarations
        if line.starts_with("enum ") {
            i += 1;
            let mut brace_count = 0;
            let mut found_opening = false;

            while i < lines.len() {
                let current_line = lines[i];
                for ch in current_line.chars() {
                    if ch == '{' {
                        found_opening = true;
                        brace_count += 1;
                    } else if ch == '}' && found_opening {
                        brace_count -= 1;
                        if brace_count == 0 {
                            i += 1;
                            break;
                        }
                    }
                }
                if brace_count == 0 && found_opening {
                    break;
                }
                i += 1;
            }
            continue;
        }

        // Skip import type statements
        if line.starts_with("import type ") {
            i += 1;
            continue;
        }

        // Convert ES6 imports to CommonJS requires
        if line.starts_with("import ") && !line.starts_with("import type ") {
            let converted = convert_import_to_require(lines[i]);
            if !converted.is_empty() {
                result_lines.push(converted);
            }
            i += 1;
            continue;
        }

        // Process regular lines - remove type annotations
        let mut processed_line = lines[i].to_string();

        // Remove type annotations from function parameters and variables
        if processed_line.contains(": ") && !processed_line.trim_start().starts_with("//") {
            processed_line = remove_type_annotations(&processed_line);
        }

        // Remove 'as Type' assertions
        if processed_line.contains(" as ") {
            let as_type_regex = AS_TYPE_REGEX
                .get_or_init(|| Regex::new(r"\s+as\s+[A-Za-z_][A-Za-z0-9_<>|&\s]*").unwrap());
            processed_line = as_type_regex.replace_all(&processed_line, "").to_string();
        }

        // Remove generics from function calls
        if processed_line.contains("<") && processed_line.contains(">(") {
            let generic_regex = GENERIC_REGEX.get_or_init(|| Regex::new(r"<[^<>]*>\s*\(").unwrap());
            processed_line = generic_regex.replace_all(&processed_line, "(").to_string();
        }

        // Clean up extra spaces
        processed_line = processed_line.replace("  ", " ");

        result_lines.push(processed_line);
        i += 1;
    }

    result_lines.join("\n")
}

fn remove_type_annotations(line: &str) -> String {
    let mut result = String::new();
    let mut chars = line.chars().peekable();
    let mut in_string = false;
    let mut string_char = '"';
    let mut paren_depth = 0;

    while let Some(ch) = chars.next() {
        if ch == '"' || ch == '\'' {
            if !in_string {
                in_string = true;
                string_char = ch;
            } else if ch == string_char {
                in_string = false;
            }
            result.push(ch);
        } else if !in_string {
            if ch == '(' {
                paren_depth += 1;
                result.push(ch);
            } else if ch == ')' {
                paren_depth -= 1;
                result.push(ch);
            } else if ch == ':' && paren_depth > 0 {
                // Only remove type annotations inside parentheses (function parameters)
                // Skip until we find a delimiter
                while let Some(&next) = chars.peek() {
                    if next == ',' || next == ')' {
                        break;
                    }
                    chars.next();
                }
                // Don't consume the delimiter
            } else if ch == ':' && paren_depth == 0 && result.trim_end().ends_with(')') {
                // Function return type annotation
                while let Some(&next) = chars.peek() {
                    if next == '{' || next == ';' || next == '=' {
                        break;
                    }
                    chars.next();
                }
                // Don't consume the delimiter
            } else if ch == ':' && paren_depth == 0 {
                // Variable type annotation (const x: Type = ...)
                while let Some(&next) = chars.peek() {
                    if next == '=' || next == ';' || next == ',' {
                        break;
                    }
                    chars.next();
                }
                // Don't consume the delimiter
            } else {
                result.push(ch);
            }
        } else {
            result.push(ch);
        }
    }

    result
}

fn convert_import_to_require(line: &str) -> String {
    let line = line.trim();

    // Handle different import patterns
    if line.starts_with("import ") {
        // import './path' or import "./path"
        if line.contains("'") && !line.contains(" from ") {
            if let Some(start) = line.find("'") {
                if let Some(end) = line.rfind("'") {
                    let path = &line[start + 1..end];
                    return format!("require('{}');", path);
                }
            }
        } else if line.contains("\"") && !line.contains(" from ") {
            if let Some(start) = line.find("\"") {
                if let Some(end) = line.rfind("\"") {
                    let path = &line[start + 1..end];
                    return format!("require('{}');", path);
                }
            }
        }
        // import { name } from 'module'
        else if line.contains(" from ") {
            let parts: Vec<&str> = line.split(" from ").collect();
            if parts.len() == 2 {
                let import_part = parts[0].trim();
                let from_part = parts[1].trim();

                // Extract module path
                let module_path = if from_part.starts_with("'") && from_part.ends_with("'") {
                    &from_part[1..from_part.len() - 1]
                } else if from_part.starts_with("\"") && from_part.ends_with("\"") {
                    &from_part[1..from_part.len() - 1]
                } else {
                    from_part
                };

                // Handle different import styles
                if import_part.starts_with("import {") && import_part.ends_with("}") {
                    // Named imports: import { name1, name2 } from 'module'
                    let imports = import_part[8..import_part.len() - 1].trim();
                    return format!("const {{ {} }} = require('{}');", imports, module_path);
                } else if import_part.starts_with("import ") {
                    // Default import: import name from 'module'
                    let name = import_part[7..].trim();
                    return format!("const {} = require('{}');", name, module_path);
                }
            }
        }
    }

    // If we can't parse it, just comment it out
    format!("// {}", line)
}

fn convert_export_to_commonjs_with_name(line: &str) -> (String, Option<String>) {
    let line = line.trim();

    if line.starts_with("export const ")
        || line.starts_with("export let ")
        || line.starts_with("export var ")
    {
        // export const NAME = value -> const NAME = value
        let after_export = &line[7..]; // Remove "export "
        if let Some(equals_pos) = after_export.find('=') {
            let before_equals = after_export[..equals_pos].trim();
            let var_parts: Vec<&str> = before_equals.split_whitespace().collect();
            if var_parts.len() >= 2 {
                let var_name = var_parts[1];
                return (after_export.to_string(), Some(var_name.to_string()));
            }
        }
    } else if line.starts_with("export function ") {
        // export function name() { -> function name() {
        let after_export = &line[7..]; // Remove "export "
        if let Some(paren_pos) = after_export.find('(') {
            let func_name = after_export[9..paren_pos].trim(); // Remove "function "
            return (after_export.to_string(), Some(func_name.to_string()));
        }
    } else if line.starts_with("export default ") {
        // export default value -> module.exports = value;
        let value = &line[15..]; // Remove "export default "
        return (format!("module.exports = {};", value), None);
    }

    // Fallback: comment out
    (format!("// {}", line), None)
}

pub fn is_typescript_file(filename: &str) -> bool {
    filename.ends_with(".ts") || filename.ends_with(".tsx")
}

pub fn convert_es6_imports(source_code: &str) -> String {
    let lines: Vec<&str> = source_code.lines().collect();
    let mut result_lines = Vec::new();
    let mut exports = Vec::new();
    let mut i = 0;

    while i < lines.len() {
        let line = lines[i].trim();

        // Convert ES6 imports to CommonJS requires
        if line.starts_with("import ") && !line.starts_with("import type ") {
            let converted = convert_import_to_require(lines[i]);
            if !converted.is_empty() {
                result_lines.push(converted);
            }
        }
        // Convert ES6 exports to CommonJS exports
        else if line.starts_with("export ") {
            let (converted_line, export_name) = convert_export_to_commonjs_with_name(lines[i]);
            result_lines.push(converted_line);
            if let Some(name) = export_name {
                exports.push(name);
            }
        } else {
            // Keep regular lines as-is
            result_lines.push(lines[i].to_string());
        }

        i += 1;
    }

    // Add module.exports at the end if we have exports
    if !exports.is_empty() {
        result_lines.push(String::new());
        result_lines.push("// CommonJS exports".to_string());
        for export_name in exports {
            result_lines.push(format!("module.exports.{} = {};", export_name, export_name));
        }
    }

    result_lines.join("\n")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_strip_type_annotations() {
        let ts_code = r#"
function add(a: number, b: number): number {
    return a + b;
}

const name: string = "test";
let count: number = 0;
        "#;

        let result = strip_typescript(ts_code);
        assert!(result.contains("function add(a, b)"));
        assert!(result.contains("const name = \"test\""));
        assert!(result.contains("let count = 0"));
    }

    #[test]
    fn test_strip_interfaces() {
        let ts_code = r#"
interface User {
    name: string;
    age: number;
}

const user = { name: "John", age: 30 };
        "#;

        let result = strip_typescript(ts_code);
        assert!(!result.contains("interface User"));
        assert!(result.contains("const user = { name: \"John\", age: 30 }"));
    }

    #[test]
    fn test_strip_type_aliases() {
        let ts_code = r#"
type StringOrNumber = string | number;
type UserID = number;

const id: UserID = 123;
        "#;

        let result = strip_typescript(ts_code);
        assert!(!result.contains("type StringOrNumber"));
        assert!(!result.contains("type UserID"));
        assert!(result.contains("const id = 123"));
    }
}
