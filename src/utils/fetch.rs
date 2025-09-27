use reqwest;
use rusty_v8 as v8;
use std::collections::HashMap;
use std::time::Duration;
use tokio::runtime::Runtime;

pub fn setup_fetch(scope: &mut v8::HandleScope) {
    let fetch_fn = v8::Function::new(scope, fetch_function).unwrap();
    let global = scope.get_current_context().global(scope);
    let key = v8::String::new(scope, "fetch").unwrap();
    global.set(scope, key.into(), fetch_fn.into());
}

fn fetch_function(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    let url_arg = args.get(0);
    let options_arg = args.get(1);

    // Extract URL
    let url = if let Some(url_str) = url_arg.to_string(scope) {
        url_str.to_rust_string_lossy(scope)
    } else {
        let error = v8::String::new(scope, "fetch: URL is required").unwrap();
        let exception = v8::Exception::type_error(scope, error);
        scope.throw_exception(exception);
        return;
    };

    // Parse options
    let mut method = "GET".to_string();
    let mut headers = HashMap::new();
    let mut body: Option<String> = None;
    let mut timeout_ms = 30000u64; // 30 seconds default

    if !options_arg.is_undefined() && !options_arg.is_null() {
        if let Some(options_obj) = options_arg.to_object(scope) {
            // Method
            let method_key = v8::String::new(scope, "method").unwrap();
            if let Some(method_val) = options_obj.get(scope, method_key.into()) {
                if let Some(method_str) = method_val.to_string(scope) {
                    method = method_str.to_rust_string_lossy(scope).to_uppercase();
                }
            }

            // Headers
            let headers_key = v8::String::new(scope, "headers").unwrap();
            if let Some(headers_val) = options_obj.get(scope, headers_key.into()) {
                if let Some(headers_obj) = headers_val.to_object(scope) {
                    let property_names = headers_obj.get_own_property_names(scope);
                    if let Some(names) = property_names {
                        let length = names.length();
                        for i in 0..length {
                            let index = v8::Integer::new(scope, i as i32);
                            if let Some(key_val) = names.get(scope, index.into()) {
                                if let Some(key_str) = key_val.to_string(scope) {
                                    let key_rust = key_str.to_rust_string_lossy(scope);
                                    if let Some(value_val) = headers_obj.get(scope, key_val) {
                                        if let Some(value_str) = value_val.to_string(scope) {
                                            let value_rust = value_str.to_rust_string_lossy(scope);
                                            headers.insert(key_rust, value_rust);
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
            }

            // Body
            let body_key = v8::String::new(scope, "body").unwrap();
            if let Some(body_val) = options_obj.get(scope, body_key.into()) {
                if !body_val.is_undefined() && !body_val.is_null() {
                    if let Some(body_str) = body_val.to_string(scope) {
                        body = Some(body_str.to_rust_string_lossy(scope));
                    }
                }
            }

            // Timeout
            let timeout_key = v8::String::new(scope, "timeout").unwrap();
            if let Some(timeout_val) = options_obj.get(scope, timeout_key.into()) {
                if let Some(timeout_num) = timeout_val.to_number(scope) {
                    timeout_ms = timeout_num.value() as u64;
                }
            }
        }
    }

    // Create tokio runtime for async operation
    let rt = Runtime::new().unwrap();

    // Perform the HTTP request
    let result = rt.block_on(async {
        let client = reqwest::Client::builder()
            .timeout(Duration::from_millis(timeout_ms))
            .build()
            .map_err(|e| format!("Failed to create HTTP client: {}", e))?;

        let mut request = match method.as_str() {
            "GET" => client.get(&url),
            "POST" => client.post(&url),
            "PUT" => client.put(&url),
            "DELETE" => client.delete(&url),
            "PATCH" => client.patch(&url),
            "HEAD" => client.head(&url),
            _ => return Err(format!("Unsupported HTTP method: {}", method)),
        };

        // Add headers
        for (key, value) in headers {
            request = request.header(&key, &value);
        }

        // Add body if present
        if let Some(body_content) = body {
            request = request.body(body_content);
        }

        // Send request
        let response = request
            .send()
            .await
            .map_err(|e| format!("Request failed: {}", e))?;

        let status = response.status().as_u16();
        let status_text = response
            .status()
            .canonical_reason()
            .unwrap_or("Unknown")
            .to_string();

        // Get response headers
        let mut response_headers = HashMap::new();
        for (name, value) in response.headers() {
            if let Ok(value_str) = value.to_str() {
                response_headers.insert(name.to_string(), value_str.to_string());
            }
        }

        // Get response body
        let body_text = response
            .text()
            .await
            .map_err(|e| format!("Failed to read response body: {}", e))?;

        Ok((status, status_text, response_headers, body_text))
    });

    match result {
        Ok((status, status_text, response_headers, body_text)) => {
            // Create response object
            let response_obj = v8::Object::new(scope);

            // Set status
            let status_key = v8::String::new(scope, "status").unwrap();
            let status_val = v8::Integer::new(scope, status as i32);
            response_obj.set(scope, status_key.into(), status_val.into());

            // Set statusText
            let status_text_key = v8::String::new(scope, "statusText").unwrap();
            let status_text_val = v8::String::new(scope, &status_text).unwrap();
            response_obj.set(scope, status_text_key.into(), status_text_val.into());

            // Set ok
            let ok_key = v8::String::new(scope, "ok").unwrap();
            let ok_val = v8::Boolean::new(scope, status >= 200 && status < 300);
            response_obj.set(scope, ok_key.into(), ok_val.into());

            // Set headers
            let headers_key = v8::String::new(scope, "headers").unwrap();
            let headers_obj = v8::Object::new(scope);
            for (name, value) in response_headers {
                let header_key = v8::String::new(scope, &name).unwrap();
                let header_val = v8::String::new(scope, &value).unwrap();
                headers_obj.set(scope, header_key.into(), header_val.into());
            }
            response_obj.set(scope, headers_key.into(), headers_obj.into());

            // Add body as a property instead of methods (simpler approach for this V8 version)
            let body_key = v8::String::new(scope, "body").unwrap();
            let body_val = v8::String::new(scope, &body_text).unwrap();
            response_obj.set(scope, body_key.into(), body_val.into());

            // Add text() method that returns the body
            let text_fn = v8::Function::new(scope, text_method).unwrap();
            let text_key = v8::String::new(scope, "text").unwrap();
            response_obj.set(scope, text_key.into(), text_fn.into());

            // Add json() method that parses the body
            let json_fn = v8::Function::new(scope, json_method).unwrap();
            let json_key = v8::String::new(scope, "json").unwrap();
            response_obj.set(scope, json_key.into(), json_fn.into());

            retval.set(response_obj.into());
        }
        Err(error_msg) => {
            println!("Fetch error: {}", error_msg); // Debug output
            let error = v8::String::new(scope, &error_msg).unwrap();
            let exception = v8::Exception::error(scope, error);
            scope.throw_exception(exception);
        }
    }
}

fn text_method(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    // Get 'this' object (the response)
    let this = args.this();
    let body_key = v8::String::new(scope, "body").unwrap();

    if let Some(body_val) = this.get(scope, body_key.into()) {
        retval.set(body_val);
    } else {
        let empty = v8::String::new(scope, "").unwrap();
        retval.set(empty.into());
    }
}

fn json_method(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut retval: v8::ReturnValue,
) {
    // Get 'this' object (the response)
    let this = args.this();
    let body_key = v8::String::new(scope, "body").unwrap();

    if let Some(body_val) = this.get(scope, body_key.into()) {
        if let Some(body_str) = body_val.to_string(scope) {
            let body_text = body_str.to_rust_string_lossy(scope);

            match serde_json::from_str::<serde_json::Value>(&body_text) {
                Ok(json_val) => {
                    let json_str = json_val.to_string();
                    let json_v8_str = v8::String::new(scope, &json_str).unwrap();
                    if let Some(parsed) = v8::json::parse(scope, json_v8_str) {
                        retval.set(parsed);
                    } else {
                        let error =
                            v8::String::new(scope, "Failed to parse JSON response").unwrap();
                        let exception = v8::Exception::syntax_error(scope, error);
                        scope.throw_exception(exception);
                    }
                }
                Err(e) => {
                    let error_msg = format!("Invalid JSON: {}", e);
                    let error = v8::String::new(scope, &error_msg).unwrap();
                    let exception = v8::Exception::syntax_error(scope, error);
                    scope.throw_exception(exception);
                }
            }
        }
    } else {
        let error = v8::String::new(scope, "No response body").unwrap();
        let exception = v8::Exception::error(scope, error);
        scope.throw_exception(exception);
    }
}
