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

    pub fn execute(&mut self, code: &str) -> Result<(), String> {
        let scope = &mut v8::HandleScope::new(&mut self.isolate);
        let context = v8::Context::new(scope);
        let scope = &mut v8::ContextScope::new(scope, context);

        crate::utils::setup_console(scope);
        crate::utils::setup_fs(scope);
        crate::utils::setup_http(scope);

        let code = v8::String::new(scope, code).unwrap();
        let script = match v8::Script::compile(scope, code, None) {
            Some(script) => script,
            None => return Err("Failed to compile script".to_string()),
        };

        match script.run(scope) {
            Some(_) => Ok(()),
            None => Err("Script execution failed".to_string()),
        }
    }
}
