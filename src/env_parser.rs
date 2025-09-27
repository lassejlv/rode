use std::collections::HashMap;
use std::env;
use std::fs;
use std::path::Path;

pub struct EnvParser {
    vars: HashMap<String, String>,
}

impl EnvParser {
    pub fn new() -> Self {
        Self {
            vars: HashMap::new(),
        }
    }

    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read env file: {}", e))?;

        self.parse_content(&content)?;
        Ok(())
    }

    pub fn parse_content(&mut self, content: &str) -> Result<(), String> {
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            self.parse_line(line, line_num + 1)?;
        }

        Ok(())
    }

    fn parse_line(&mut self, line: &str, line_num: usize) -> Result<(), String> {
        let eq_pos = line
            .find('=')
            .ok_or_else(|| format!("Invalid format at line {}: missing '='", line_num))?;

        let key = line[..eq_pos].trim();
        let value = line[eq_pos + 1..].trim();

        if key.is_empty() {
            return Err(format!("Empty key at line {}", line_num));
        }

        if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!(
                "Invalid key format at line {}: '{}'",
                line_num, key
            ));
        }

        let parsed_value = self.parse_value(value, line_num)?;

        self.vars.insert(key.to_string(), parsed_value);
        Ok(())
    }

    fn parse_value(&self, value: &str, line_num: usize) -> Result<String, String> {
        if value.is_empty() {
            return Ok(String::new());
        }

        if (value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\''))
        {
            if value.len() < 2 {
                return Err(format!("Unterminated quote at line {}", line_num));
            }

            let inner = &value[1..value.len() - 1];

            if value.starts_with('"') {
                return Ok(self.unescape_string(inner));
            } else {
                return Ok(inner.to_string());
            }
        }

        Ok(self.expand_variables(value))
    }

    fn unescape_string(&self, s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '\\' {
                match chars.peek() {
                    Some(&'n') => {
                        chars.next();
                        result.push('\n');
                    }
                    Some(&'r') => {
                        chars.next();
                        result.push('\r');
                    }
                    Some(&'t') => {
                        chars.next();
                        result.push('\t');
                    }
                    Some(&'\\') => {
                        chars.next();
                        result.push('\\');
                    }
                    Some(&'"') => {
                        chars.next();
                        result.push('"');
                    }
                    _ => {
                        result.push(ch);
                    }
                }
            } else {
                result.push(ch);
            }
        }

        self.expand_variables(&result)
    }

    fn expand_variables(&self, s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if chars.peek() == Some(&'{') {
                    chars.next(); // consume '{'
                    let mut var_name = String::new();
                    let mut found_closing = false;

                    while let Some(ch) = chars.next() {
                        if ch == '}' {
                            found_closing = true;
                            break;
                        }
                        var_name.push(ch);
                    }

                    if found_closing {
                        let value = self.get_variable(&var_name);
                        result.push_str(&value);
                    } else {
                        result.push_str("${");
                        result.push_str(&var_name);
                    }
                } else if chars
                    .peek()
                    .map_or(false, |c| c.is_alphabetic() || *c == '_')
                {
                    let mut var_name = String::new();

                    while let Some(&ch) = chars.peek() {
                        if ch.is_alphanumeric() || ch == '_' {
                            var_name.push(ch);
                            chars.next();
                        } else {
                            break;
                        }
                    }

                    let value = self.get_variable(&var_name);
                    result.push_str(&value);
                } else {
                    result.push(ch);
                }
            } else {
                result.push(ch);
            }
        }

        result
    }

    fn get_variable(&self, name: &str) -> String {
        if let Some(value) = self.vars.get(name) {
            return value.clone();
        }

        env::var(name).unwrap_or_default()
    }

    pub fn apply(&self) {
        for (key, value) in &self.vars {
            unsafe {
                env::set_var(key, value);
            }
        }
    }
}

pub fn load_env_files() -> Result<(), String> {
    let mut parser = EnvParser::new();

    let env_files = [".env.local", ".env", ".env.development", ".env.production"];

    for file in &env_files {
        if Path::new(file).exists() {
            match parser.load_file(file) {
                Ok(()) => {
                    println!("Loaded environment from {}", file);
                }
                Err(e) => {
                    eprintln!("Warning: Failed to load {}: {}", file, e);
                }
            }
        }
    }

    parser.apply();

    Ok(())
}
