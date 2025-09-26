use rusty_v8 as v8;
use std::fmt::Write;

pub fn setup_uuid(scope: &mut v8::HandleScope) {
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

    // Create uuid object
    let uuid_obj = v8::Object::new(scope);
    let uuid_key = v8::String::new(scope, "uuid").unwrap();
    rode_obj.set(scope, uuid_key.into(), uuid_obj.into());

    // uuid.v4() - Generate random UUID v4
    let v4_key = v8::String::new(scope, "v4").unwrap();
    let v4_func = v8::Function::new(scope, uuid_v4).unwrap();
    uuid_obj.set(scope, v4_key.into(), v4_func.into());

    // uuid.v1() - Generate time-based UUID v1
    let v1_key = v8::String::new(scope, "v1").unwrap();
    let v1_func = v8::Function::new(scope, uuid_v1).unwrap();
    uuid_obj.set(scope, v1_key.into(), v1_func.into());

    // uuid.v7() - Generate time-ordered UUID v7
    let v7_key = v8::String::new(scope, "v7").unwrap();
    let v7_func = v8::Function::new(scope, uuid_v7).unwrap();
    uuid_obj.set(scope, v7_key.into(), v7_func.into());

    // uuid.nil() - Get nil UUID (all zeros)
    let nil_key = v8::String::new(scope, "nil").unwrap();
    let nil_func = v8::Function::new(scope, uuid_nil).unwrap();
    uuid_obj.set(scope, nil_key.into(), nil_func.into());

    // uuid.parse(uuid) - Parse UUID string to validate
    let parse_key = v8::String::new(scope, "parse").unwrap();
    let parse_func = v8::Function::new(scope, uuid_parse).unwrap();
    uuid_obj.set(scope, parse_key.into(), parse_func.into());

    // uuid.validate(uuid) - Validate UUID string
    let validate_key = v8::String::new(scope, "validate").unwrap();
    let validate_func = v8::Function::new(scope, uuid_validate).unwrap();
    uuid_obj.set(scope, validate_key.into(), validate_func.into());

    // uuid.version(uuid) - Get UUID version
    let version_key = v8::String::new(scope, "version").unwrap();
    let version_func = v8::Function::new(scope, uuid_version).unwrap();
    uuid_obj.set(scope, version_key.into(), version_func.into());
}

fn uuid_v4(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let uuid = generate_uuid_v4();
    let result_str = v8::String::new(scope, &uuid).unwrap();
    rv.set(result_str.into());
}

fn uuid_v1(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let uuid = generate_uuid_v1();
    let result_str = v8::String::new(scope, &uuid).unwrap();
    rv.set(result_str.into());
}

fn uuid_v7(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let uuid = generate_uuid_v7();
    let result_str = v8::String::new(scope, &uuid).unwrap();
    rv.set(result_str.into());
}

fn uuid_nil(
    scope: &mut v8::HandleScope,
    _args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let nil_uuid = "00000000-0000-0000-0000-000000000000";
    let result_str = v8::String::new(scope, nil_uuid).unwrap();
    rv.set(result_str.into());
}

fn uuid_parse(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "UUID string required").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    if let Some(uuid_str) = args.get(0).to_string(scope) {
        let uuid_string = uuid_str.to_rust_string_lossy(scope);
        match parse_uuid(&uuid_string) {
            Ok(parsed) => {
                let result_str = v8::String::new(scope, &parsed).unwrap();
                rv.set(result_str.into());
            }
            Err(err) => {
                let error = v8::String::new(scope, &err).unwrap();
                scope.throw_exception(error.into());
            }
        }
    }
}

fn uuid_validate(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result = v8::Boolean::new(scope, false);
        rv.set(result.into());
        return;
    }

    if let Some(uuid_str) = args.get(0).to_string(scope) {
        let uuid_string = uuid_str.to_rust_string_lossy(scope);
        let is_valid = validate_uuid(&uuid_string);
        let result = v8::Boolean::new(scope, is_valid);
        rv.set(result.into());
    } else {
        let result = v8::Boolean::new(scope, false);
        rv.set(result.into());
    }
}

fn uuid_version(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result = v8::null(scope);
        rv.set(result.into());
        return;
    }

    if let Some(uuid_str) = args.get(0).to_string(scope) {
        let uuid_string = uuid_str.to_rust_string_lossy(scope);
        if let Ok(version) = get_uuid_version(&uuid_string) {
            let result = v8::Number::new(scope, version as f64);
            rv.set(result.into());
        } else {
            let result = v8::null(scope);
            rv.set(result.into());
        }
    } else {
        let result = v8::null(scope);
        rv.set(result.into());
    }
}

// UUID generation functions
fn generate_uuid_v4() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Get some entropy from system time and a simple PRNG
    let mut seed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;

    // Simple LCG for random number generation
    let mut rng = || {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        seed
    };

    let mut uuid = String::with_capacity(36);

    // Generate 16 bytes of random data
    let mut bytes = [0u8; 16];
    for i in 0..16 {
        bytes[i] = (rng() >> (i % 8 * 8)) as u8;
    }

    // Set version (4) and variant bits
    bytes[6] = (bytes[6] & 0x0F) | 0x40; // Version 4
    bytes[8] = (bytes[8] & 0x3F) | 0x80; // Variant 10

    // Format as UUID string
    write!(
        &mut uuid,
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        bytes[0], bytes[1], bytes[2], bytes[3],
        bytes[4], bytes[5],
        bytes[6], bytes[7],
        bytes[8], bytes[9],
        bytes[10], bytes[11], bytes[12], bytes[13], bytes[14], bytes[15]
    ).unwrap();

    uuid
}

fn generate_uuid_v1() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Get timestamp (100-nanosecond intervals since UUID epoch: Oct 15, 1582)
    let uuid_epoch_offset = 122192928000000000u64; // Offset from Unix epoch to UUID epoch
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos() as u64;
    let timestamp = (now / 100) + uuid_epoch_offset;

    // Simple clock sequence (would be better with actual MAC address)
    let clock_seq = (now % 16384) as u16;

    // Fake MAC address (in real implementation, would use actual network interface)
    let node = [0x01, 0x23, 0x45, 0x67, 0x89, 0xAB];

    let mut uuid = String::with_capacity(36);

    let time_low = (timestamp & 0xFFFFFFFF) as u32;
    let time_mid = ((timestamp >> 32) & 0xFFFF) as u16;
    let time_hi_and_version = (((timestamp >> 48) & 0x0FFF) | 0x1000) as u16; // Version 1
    let clock_seq_hi_and_reserved = ((clock_seq >> 8) & 0x3F) | 0x80; // Variant 10
    let clock_seq_low = (clock_seq & 0xFF) as u8;

    write!(
        &mut uuid,
        "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        time_low,
        time_mid,
        time_hi_and_version,
        clock_seq_hi_and_reserved,
        clock_seq_low,
        node[0],
        node[1],
        node[2],
        node[3],
        node[4],
        node[5]
    )
    .unwrap();

    uuid
}

fn generate_uuid_v7() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    // Get current timestamp in milliseconds since Unix epoch
    let timestamp_ms = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_millis() as u64;

    // Generate random data for the rest
    let mut seed = timestamp_ms;
    let mut rng = || {
        seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
        seed
    };

    let mut uuid = String::with_capacity(36);

    // UUID v7 format:
    // - 48 bits: timestamp (milliseconds since Unix epoch)
    // - 12 bits: random data A
    // - 4 bits: version (7)
    // - 62 bits: random data B
    // - 2 bits: variant (10)

    // Split timestamp into bytes (big-endian, 48 bits = 6 bytes)
    let timestamp_bytes = [
        ((timestamp_ms >> 40) & 0xFF) as u8,
        ((timestamp_ms >> 32) & 0xFF) as u8,
        ((timestamp_ms >> 24) & 0xFF) as u8,
        ((timestamp_ms >> 16) & 0xFF) as u8,
        ((timestamp_ms >> 8) & 0xFF) as u8,
        (timestamp_ms & 0xFF) as u8,
    ];

    // Generate random data for the remaining bytes
    let rand_a = (rng() & 0xFFF) as u16; // 12 bits of random data A
    let rand_b = [
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
        (rng() & 0xFF) as u8,
    ];

    // Set version 7 in the upper 4 bits of the 7th byte
    let version_and_rand_a = 0x7000 | rand_a; // Version 7 + 12 bits random

    // Set variant bits (10) in the upper 2 bits of the 9th byte
    let variant_and_rand_b0 = 0x80 | (rand_b[0] & 0x3F); // Variant 10 + 6 bits random

    // Format as UUID string
    write!(
        &mut uuid,
        "{:02x}{:02x}{:02x}{:02x}-{:02x}{:02x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
        timestamp_bytes[0], timestamp_bytes[1], timestamp_bytes[2], timestamp_bytes[3],
        timestamp_bytes[4], timestamp_bytes[5],
        version_and_rand_a,
        variant_and_rand_b0, rand_b[1],
        rand_b[2], rand_b[3], rand_b[4], rand_b[5], rand_b[6], rand_b[7]
    ).unwrap();

    uuid
}

fn validate_uuid(uuid: &str) -> bool {
    if uuid.len() != 36 {
        return false;
    }

    let chars: Vec<char> = uuid.chars().collect();

    // Check format: xxxxxxxx-xxxx-xxxx-xxxx-xxxxxxxxxxxx
    if chars[8] != '-' || chars[13] != '-' || chars[18] != '-' || chars[23] != '-' {
        return false;
    }

    // Check all other characters are hex
    for (i, &ch) in chars.iter().enumerate() {
        if i == 8 || i == 13 || i == 18 || i == 23 {
            continue; // Skip dashes
        }
        if !ch.is_ascii_hexdigit() {
            return false;
        }
    }

    true
}

fn parse_uuid(uuid: &str) -> Result<String, String> {
    if !validate_uuid(uuid) {
        return Err("Invalid UUID format".to_string());
    }

    // Return normalized uppercase version
    Ok(uuid.to_uppercase())
}

fn get_uuid_version(uuid: &str) -> Result<u8, String> {
    if !validate_uuid(uuid) {
        return Err("Invalid UUID format".to_string());
    }

    let chars: Vec<char> = uuid.chars().collect();
    let version_char = chars[14]; // The version digit is at position 14

    match version_char.to_digit(16) {
        Some(digit) => Ok(digit as u8),
        None => Err("Invalid version digit".to_string()),
    }
}
