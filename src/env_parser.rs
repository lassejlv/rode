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

    /// Load environment variables from a file
    pub fn load_file<P: AsRef<Path>>(&mut self, path: P) -> Result<(), String> {
        let content =
            fs::read_to_string(&path).map_err(|e| format!("Failed to read env file: {}", e))?;

        self.parse_content(&content)?;
        Ok(())
    }

    /// Parse environment variables from string content
    pub fn parse_content(&mut self, content: &str) -> Result<(), String> {
        for (line_num, line) in content.lines().enumerate() {
            let line = line.trim();

            // Skip empty lines and comments
            if line.is_empty() || line.starts_with('#') {
                continue;
            }

            self.parse_line(line, line_num + 1)?;
        }

        Ok(())
    }

    /// Parse a single line of environment variable definition
    fn parse_line(&mut self, line: &str, line_num: usize) -> Result<(), String> {
        // Find the first '=' character
        let eq_pos = line
            .find('=')
            .ok_or_else(|| format!("Invalid format at line {}: missing '='", line_num))?;

        let key = line[..eq_pos].trim();
        let value = line[eq_pos + 1..].trim();

        // Validate key
        if key.is_empty() {
            return Err(format!("Empty key at line {}", line_num));
        }

        // Check for valid key format (letters, numbers, underscores)
        if !key.chars().all(|c| c.is_alphanumeric() || c == '_') {
            return Err(format!(
                "Invalid key format at line {}: '{}'",
                line_num, key
            ));
        }

        // Parse value (handle quotes)
        let parsed_value = self.parse_value(value, line_num)?;

        self.vars.insert(key.to_string(), parsed_value);
        Ok(())
    }

    /// Parse environment variable value, handling quotes and escapes
    fn parse_value(&self, value: &str, line_num: usize) -> Result<String, String> {
        if value.is_empty() {
            return Ok(String::new());
        }

        // Handle quoted values
        if (value.starts_with('"') && value.ends_with('"'))
            || (value.starts_with('\'') && value.ends_with('\''))
        {
            if value.len() < 2 {
                return Err(format!("Unterminated quote at line {}", line_num));
            }

            let inner = &value[1..value.len() - 1];

            // Handle escape sequences in double quotes
            if value.starts_with('"') {
                return Ok(self.unescape_string(inner));
            } else {
                // Single quotes - no escape processing
                return Ok(inner.to_string());
            }
        }

        // Unquoted value - expand variables
        Ok(self.expand_variables(value))
    }

    /// Process escape sequences in double-quoted strings
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

    /// Expand ${VAR} and $VAR variable references
    fn expand_variables(&self, s: &str) -> String {
        let mut result = String::new();
        let mut chars = s.chars().peekable();

        while let Some(ch) = chars.next() {
            if ch == '$' {
                if chars.peek() == Some(&'{') {
                    // ${VAR} format
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
                        // Malformed ${VAR, treat as literal
                        result.push_str("${");
                        result.push_str(&var_name);
                    }
                } else if chars
                    .peek()
                    .map_or(false, |c| c.is_alphabetic() || *c == '_')
                {
                    // $VAR format
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

    /// Get variable value from loaded vars or system environment
    fn get_variable(&self, name: &str) -> String {
        // First check our loaded variables
        if let Some(value) = self.vars.get(name) {
            return value.clone();
        }

        // Then check system environment
        env::var(name).unwrap_or_default()
    }

    /// Apply all loaded environment variables to the current process
    pub fn apply(&self) {
        for (key, value) in &self.vars {
            unsafe {
                env::set_var(key, value);
            }
        }
    }

    /// Get all loaded variables
    pub fn get_vars(&self) -> &HashMap<String, String> {
        &self.vars
    }

    /// Clear all loaded variables
    pub fn clear(&mut self) {
        self.vars.clear();
    }
}

/// Load environment files automatically
pub fn load_env_files() -> Result<(), String> {
    let mut parser = EnvParser::new();

    // Try to load .env files in order of precedence
    let env_files = [".env.local", ".env"];

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

    // Apply all loaded variables
    parser.apply();

    Ok(())
}
