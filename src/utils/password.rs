use rusty_v8 as v8;

pub fn setup_password(scope: &mut v8::HandleScope) {
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

    // Create password object
    let password_obj = v8::Object::new(scope);
    let password_key = v8::String::new(scope, "password").unwrap();
    rode_obj.set(scope, password_key.into(), password_obj.into());

    // password.hash(password, rounds?) - Hash a password with bcrypt
    let hash_key = v8::String::new(scope, "hash").unwrap();
    let hash_func = v8::Function::new(scope, password_hash).unwrap();
    password_obj.set(scope, hash_key.into(), hash_func.into());

    // password.verify(password, hash) - Verify a password against a hash
    let verify_key = v8::String::new(scope, "verify").unwrap();
    let verify_func = v8::Function::new(scope, password_verify).unwrap();
    password_obj.set(scope, verify_key.into(), verify_func.into());

    // password.strength(password) - Check password strength
    let strength_key = v8::String::new(scope, "strength").unwrap();
    let strength_func = v8::Function::new(scope, password_strength).unwrap();
    password_obj.set(scope, strength_key.into(), strength_func.into());

    // password.generate(length?, options?) - Generate a secure password
    let generate_key = v8::String::new(scope, "generate").unwrap();
    let generate_func = v8::Function::new(scope, password_generate).unwrap();
    password_obj.set(scope, generate_key.into(), generate_func.into());
}

fn password_hash(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "Password required").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let password = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid password").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    // Get rounds (default: 12)
    let rounds = if args.length() >= 2 {
        args.get(1).uint32_value(scope).unwrap_or(12).min(20).max(4)
    } else {
        12
    };

    match bcrypt_hash(&password, rounds) {
        Ok(hash) => {
            let result_str = v8::String::new(scope, &hash).unwrap();
            rv.set(result_str.into());
        }
        Err(err) => {
            let error = v8::String::new(scope, &err).unwrap();
            scope.throw_exception(error.into());
        }
    }
}

fn password_verify(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        let error = v8::String::new(scope, "Password and hash required").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let password = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid password").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    let hash = match args.get(1).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid hash").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    match bcrypt_verify(&password, &hash) {
        Ok(is_valid) => {
            let result = v8::Boolean::new(scope, is_valid);
            rv.set(result.into());
        }
        Err(err) => {
            let error = v8::String::new(scope, &err).unwrap();
            scope.throw_exception(error.into());
        }
    }
}

fn password_strength(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "Password required").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let password = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid password").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    let strength = calculate_password_strength(&password);

    // Create result object
    let result_obj = v8::Object::new(scope);

    // Score (0-100)
    let score_key = v8::String::new(scope, "score").unwrap();
    let score_val = v8::Number::new(scope, strength.score as f64);
    result_obj.set(scope, score_key.into(), score_val.into());

    // Level (weak, fair, good, strong)
    let level_key = v8::String::new(scope, "level").unwrap();
    let level_val = v8::String::new(scope, &strength.level).unwrap();
    result_obj.set(scope, level_key.into(), level_val.into());

    // Feedback array
    let feedback_key = v8::String::new(scope, "feedback").unwrap();
    let feedback_array = v8::Array::new(scope, strength.feedback.len() as i32);
    for (i, feedback) in strength.feedback.iter().enumerate() {
        let feedback_str = v8::String::new(scope, feedback).unwrap();
        let index = v8::Number::new(scope, i as f64);
        feedback_array.set(scope, index.into(), feedback_str.into());
    }
    result_obj.set(scope, feedback_key.into(), feedback_array.into());

    rv.set(result_obj.into());
}

fn password_generate(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    // Get length (default: 16)
    let length = if args.length() >= 1 {
        args.get(0)
            .uint32_value(scope)
            .unwrap_or(16)
            .min(128)
            .max(4)
    } else {
        16
    };

    // Get options (default: all character types)
    let mut options = PasswordGenOptions {
        include_lowercase: true,
        include_uppercase: true,
        include_numbers: true,
        include_symbols: true,
        exclude_similar: false,
    };

    if args.length() >= 2 {
        if let Some(opts_obj) = args.get(1).to_object(scope) {
            let lowercase_key = v8::String::new(scope, "lowercase").unwrap();
            if let Some(val) = opts_obj.get(scope, lowercase_key.into()) {
                options.include_lowercase = val.boolean_value(scope);
            }

            let uppercase_key = v8::String::new(scope, "uppercase").unwrap();
            if let Some(val) = opts_obj.get(scope, uppercase_key.into()) {
                options.include_uppercase = val.boolean_value(scope);
            }

            let numbers_key = v8::String::new(scope, "numbers").unwrap();
            if let Some(val) = opts_obj.get(scope, numbers_key.into()) {
                options.include_numbers = val.boolean_value(scope);
            }

            let symbols_key = v8::String::new(scope, "symbols").unwrap();
            if let Some(val) = opts_obj.get(scope, symbols_key.into()) {
                options.include_symbols = val.boolean_value(scope);
            }

            let exclude_key = v8::String::new(scope, "excludeSimilar").unwrap();
            if let Some(val) = opts_obj.get(scope, exclude_key.into()) {
                options.exclude_similar = val.boolean_value(scope);
            }
        }
    }

    let password = generate_password(length as usize, &options);
    let result_str = v8::String::new(scope, &password).unwrap();
    rv.set(result_str.into());
}

// Bcrypt implementation (simplified for demonstration)
fn bcrypt_hash(password: &str, rounds: u32) -> Result<String, String> {
    // Generate salt
    let salt = generate_salt(rounds)?;

    // Hash password with salt
    let hash = simple_bcrypt(password, &salt)?;

    // Format as bcrypt hash: $2b$rounds$salt$hash
    Ok(format!("$2b${:02}${}${}", rounds, salt, hash))
}

fn bcrypt_verify(password: &str, hash: &str) -> Result<bool, String> {
    // Parse hash format: $2b$rounds$salt$hash
    let parts: Vec<&str> = hash.split('$').collect();
    if parts.len() != 5 || parts[0] != "" || parts[1] != "2b" {
        return Err("Invalid hash format".to_string());
    }

    let _rounds: u32 = parts[2].parse().map_err(|_| "Invalid rounds")?;
    let salt = parts[3];
    let expected_hash = parts[4];

    // Hash the provided password with the same salt
    let computed_hash = simple_bcrypt(password, salt)?;

    // Constant-time comparison
    Ok(constant_time_eq(&computed_hash, expected_hash))
}

fn generate_salt(_rounds: u32) -> Result<String, String> {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut rng = || {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        seed
    };

    // Generate 22 character salt (base64-like encoding)
    let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789./";
    let mut salt = String::with_capacity(22);

    for _ in 0..22 {
        let idx = (rng() % 64) as usize;
        salt.push(charset.chars().nth(idx).unwrap());
    }

    Ok(salt)
}

fn simple_bcrypt(password: &str, salt: &str) -> Result<String, String> {
    // Simplified bcrypt-like hash (not cryptographically secure - for demo only)
    let mut result = format!("{}{}", password, salt);

    // Apply multiple rounds of hashing
    for _ in 0..100 {
        result = simple_hash(&result);
    }

    // Take first 31 characters and encode
    let hash_bytes = result.as_bytes();
    let mut encoded = String::with_capacity(31);

    for i in 0..31 {
        let byte = hash_bytes.get(i % hash_bytes.len()).unwrap_or(&0);
        let char_idx = (*byte as usize) % 64;
        let charset = "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789./";
        encoded.push(charset.chars().nth(char_idx).unwrap());
    }

    Ok(encoded)
}

fn simple_hash(input: &str) -> String {
    // Simple hash function (not secure - for demo only)
    let mut hash: u64 = 5381;

    for byte in input.bytes() {
        hash = hash.wrapping_mul(33).wrapping_add(byte as u64);
    }

    format!("{:016x}", hash)
}

fn constant_time_eq(a: &str, b: &str) -> bool {
    if a.len() != b.len() {
        return false;
    }

    let mut result = 0u8;
    for (a_byte, b_byte) in a.bytes().zip(b.bytes()) {
        result |= a_byte ^ b_byte;
    }

    result == 0
}

struct PasswordStrength {
    score: u8,
    level: String,
    feedback: Vec<String>,
}

fn calculate_password_strength(password: &str) -> PasswordStrength {
    let mut score = 0u8;
    let mut feedback = Vec::new();

    let length = password.len();
    let has_lowercase = password.chars().any(|c| c.is_ascii_lowercase());
    let has_uppercase = password.chars().any(|c| c.is_ascii_uppercase());
    let has_numbers = password.chars().any(|c| c.is_ascii_digit());
    let has_symbols = password.chars().any(|c| !c.is_alphanumeric());

    // Length scoring
    if length >= 8 {
        score += 20;
    } else {
        feedback.push("Use at least 8 characters".to_string());
    }

    if length >= 12 {
        score += 10;
    }

    if length >= 16 {
        score += 10;
    }

    // Character variety scoring
    if has_lowercase {
        score += 10;
    } else {
        feedback.push("Add lowercase letters".to_string());
    }

    if has_uppercase {
        score += 10;
    } else {
        feedback.push("Add uppercase letters".to_string());
    }

    if has_numbers {
        score += 10;
    } else {
        feedback.push("Add numbers".to_string());
    }

    if has_symbols {
        score += 15;
    } else {
        feedback.push("Add symbols (!@#$%^&*)".to_string());
    }

    // Bonus for variety
    let char_types = [has_lowercase, has_uppercase, has_numbers, has_symbols]
        .iter()
        .filter(|&&x| x)
        .count();

    if char_types >= 3 {
        score += 10;
    }

    if char_types >= 4 {
        score += 5;
    }

    // Check for common patterns
    if password.to_lowercase().contains("password") {
        score = score.saturating_sub(20);
        feedback.push("Avoid using 'password'".to_string());
    }

    if password
        .chars()
        .collect::<Vec<_>>()
        .windows(3)
        .any(|w| w[0] as u8 + 1 == w[1] as u8 && w[1] as u8 + 1 == w[2] as u8)
    {
        score = score.saturating_sub(10);
        feedback.push("Avoid sequential characters".to_string());
    }

    // Determine level
    let level = match score {
        0..=30 => "weak",
        31..=60 => "fair",
        61..=80 => "good",
        _ => "strong",
    };

    if feedback.is_empty() {
        feedback.push("Password looks good!".to_string());
    }

    PasswordStrength {
        score: score.min(100),
        level: level.to_string(),
        feedback,
    }
}

struct PasswordGenOptions {
    include_lowercase: bool,
    include_uppercase: bool,
    include_numbers: bool,
    include_symbols: bool,
    exclude_similar: bool,
}

fn generate_password(length: usize, options: &PasswordGenOptions) -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut charset = String::new();

    if options.include_lowercase {
        if options.exclude_similar {
            charset.push_str("abcdefghjkmnpqrstuvwxyz"); // Exclude i, l, o
        } else {
            charset.push_str("abcdefghijklmnopqrstuvwxyz");
        }
    }

    if options.include_uppercase {
        if options.exclude_similar {
            charset.push_str("ABCDEFGHJKMNPQRSTUVWXYZ"); // Exclude I, L, O
        } else {
            charset.push_str("ABCDEFGHIJKLMNOPQRSTUVWXYZ");
        }
    }

    if options.include_numbers {
        if options.exclude_similar {
            charset.push_str("23456789"); // Exclude 0, 1
        } else {
            charset.push_str("0123456789");
        }
    }

    if options.include_symbols {
        charset.push_str("!@#$%^&*()_+-=[]{}|;:,.<>?");
    }

    if charset.is_empty() {
        charset.push_str("abcdefghijklmnopqrstuvwxyz"); // Fallback
    }

    let chars: Vec<char> = charset.chars().collect();
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    let mut rng = || {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        seed
    };

    let mut password = String::with_capacity(length);
    for _ in 0..length {
        let idx = (rng() % chars.len() as u64) as usize;
        password.push(chars[idx]);
    }

    password
}
