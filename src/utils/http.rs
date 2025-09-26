use rusty_v8 as v8;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

struct HttpHandler {
    callback_js: String,
}

fn handle_client(mut stream: TcpStream, handler: &HttpHandler) {
    let mut buffer = [0; 1024];
    if let Ok(_) = stream.read(&mut buffer) {
        let request = String::from_utf8_lossy(&buffer);
        let lines: Vec<&str> = request.lines().collect();

        if lines.is_empty() {
            return;
        }

        let first_line_parts: Vec<&str> = lines[0].split_whitespace().collect();
        if first_line_parts.len() < 3 {
            return;
        }

        let method = first_line_parts[0];
        let path = first_line_parts[1];

        let response_body = format!(
            r#"
            (function() {{
                const JSON = {{
                    stringify: function(obj) {{
                        if (typeof obj === 'string') return '"' + obj + '"';
                        if (typeof obj === 'number' || typeof obj === 'boolean') return obj.toString();
                        if (obj === null) return 'null';
                        if (Array.isArray(obj)) {{
                            return '[' + obj.map(item => JSON.stringify(item)).join(',') + ']';
                        }}
                        if (typeof obj === 'object') {{
                            const pairs = [];
                            for (const key in obj) {{
                                pairs.push('"' + key + '":' + JSON.stringify(obj[key]));
                            }}
                            return '{{' + pairs.join(',') + '}}';
                        }}
                        return '""';
                    }}
                }};
                const Date = {{
                    now: function() {{
                        return Math.floor(Math.random() * 1000000000);
                    }}
                }};
                const request = {{
                    method: "{}",
                    url: "{}"
                }};
                const handler = {};
                try {{
                    const response = handler(request);
                    return response;
                }} catch (e) {{
                    return {{ status: 500, body: "Handler error: " + e.toString() }};
                }}
            }})()
            "#,
            method, path, handler.callback_js
        );

        let mut isolate = v8::Isolate::new(Default::default());
        let scope = &mut v8::HandleScope::new(&mut isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        let code = v8::String::new(scope, &response_body).unwrap();

        let (status, body) = if let Some(script) = v8::Script::compile(scope, code, None) {
            if let Some(result) = script.run(scope) {
                if let Some(obj) = result.to_object(scope) {
                    let status_key = v8::String::new(scope, "status").unwrap();
                    let body_key = v8::String::new(scope, "body").unwrap();

                    let status = obj
                        .get(scope, status_key.into())
                        .and_then(|v| v.to_uint32(scope))
                        .map(|v| v.value() as u16)
                        .unwrap_or(200);

                    let body = obj
                        .get(scope, body_key.into())
                        .and_then(|v| v.to_string(scope))
                        .map(|v| v.to_rust_string_lossy(scope))
                        .unwrap_or_else(|| "".to_string());

                    (status, body)
                } else {
                    let body = result
                        .to_string(scope)
                        .map(|v| v.to_rust_string_lossy(scope))
                        .unwrap_or_else(|| "".to_string());
                    (200, body)
                }
            } else {
                (500, "Script execution failed".to_string())
            }
        } else {
            (500, "Script compilation failed".to_string())
        };

        let response = format!(
            "HTTP/1.1 {} OK\r\nContent-Length: {}\r\n\r\n{}",
            status,
            body.len(),
            body
        );

        let _ = stream.write(response.as_bytes());
        let _ = stream.flush();
    }
}

pub fn rode_serve(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "serve requires a handler function").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let handler_func = args.get(0);
    if !handler_func.is_function() {
        let error = v8::String::new(scope, "serve requires a function as first argument").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let port = if args.length() > 1 {
        args.get(1)
            .to_uint32(scope)
            .map(|v| v.value() as u16)
            .unwrap_or(8000)
    } else {
        8000
    };

    let callback_js = handler_func
        .to_string(scope)
        .map(|s| s.to_rust_string_lossy(scope))
        .unwrap_or_else(|| "function() { return { status: 500, body: 'Error' }; }".to_string());

    let handler = HttpHandler { callback_js };

    let server_handle = thread::spawn(move || {
        let listener = match TcpListener::bind(format!("127.0.0.1:{}", port)) {
            Ok(listener) => listener,
            Err(e) => {
                eprintln!("Failed to bind to port {}: {}", port, e);
                return;
            }
        };

        println!("Server running on http://127.0.0.1:{}", port);

        for stream in listener.incoming() {
            match stream {
                Ok(stream) => {
                    let handler_clone = HttpHandler {
                        callback_js: handler.callback_js.clone(),
                    };
                    thread::spawn(move || {
                        handle_client(stream, &handler_clone);
                    });
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    });

    loop {
        thread::sleep(Duration::from_millis(100));
        if server_handle.is_finished() {
            break;
        }
    }
}

pub fn setup_http(scope: &mut v8::HandleScope) {
    let global = scope.get_current_context().global(scope);
    let rode_key = v8::String::new(scope, "Rode").unwrap();

    let rode_obj = if let Some(existing) = global
        .get(scope, rode_key.into())
        .and_then(|v| v.to_object(scope))
    {
        existing
    } else {
        let new_obj = v8::Object::new(scope);
        global.set(scope, rode_key.into(), new_obj.into());
        new_obj
    };

    let http_obj = v8::Object::new(scope);
    let http_key = v8::String::new(scope, "http").unwrap();

    let serve_key = v8::String::new(scope, "serve").unwrap();
    let serve_func = v8::Function::new(scope, rode_serve).unwrap();
    http_obj.set(scope, serve_key.into(), serve_func.into());

    rode_obj.set(scope, http_key.into(), http_obj.into());
}
