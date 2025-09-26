use rusty_v8 as v8;
use std::fs;
use std::path::Path;

pub fn rode_read_file(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "readFile requires a filename argument").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let filename = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid filename").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    match fs::read_to_string(&filename) {
        Ok(content) => {
            let result = v8::String::new(scope, &content).unwrap();
            rv.set(result.into());
        }
        Err(err) => {
            let error_msg = format!("Failed to read file '{}': {}", filename, err);
            let error = v8::String::new(scope, &error_msg).unwrap();
            scope.throw_exception(error.into());
        }
    }
}

pub fn rode_write_file(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        let error =
            v8::String::new(scope, "writeFile requires filename and content arguments").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let filename = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid filename").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    let content = match args.get(1).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid content").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    match fs::write(&filename, &content) {
        Ok(_) => {}
        Err(err) => {
            let error_msg = format!("Failed to write file '{}': {}", filename, err);
            let error = v8::String::new(scope, &error_msg).unwrap();
            scope.throw_exception(error.into());
        }
    }
}

pub fn rode_exists(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "exists requires a path argument").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let path = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid path").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    let exists = Path::new(&path).exists();
    let result = v8::Boolean::new(scope, exists);
    rv.set(result.into());
}

pub fn rode_mkdir(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "mkdir requires a path argument").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let path = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid path").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    let recursive = if args.length() > 1 {
        args.get(1).to_boolean(scope).boolean_value(scope)
    } else {
        false
    };

    let result = if recursive {
        fs::create_dir_all(&path)
    } else {
        fs::create_dir(&path)
    };

    if let Err(err) = result {
        let error_msg = format!("Failed to create directory '{}': {}", path, err);
        let error = v8::String::new(scope, &error_msg).unwrap();
        scope.throw_exception(error.into());
    }
}

pub fn rode_remove(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    _rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "remove requires a path argument").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let path = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid path").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    let recursive = if args.length() > 1 {
        args.get(1).to_boolean(scope).boolean_value(scope)
    } else {
        false
    };

    let path_obj = Path::new(&path);
    let result = if path_obj.is_dir() {
        if recursive {
            fs::remove_dir_all(&path)
        } else {
            fs::remove_dir(&path)
        }
    } else {
        fs::remove_file(&path)
    };

    if let Err(err) = result {
        let error_msg = format!("Failed to remove '{}': {}", path, err);
        let error = v8::String::new(scope, &error_msg).unwrap();
        scope.throw_exception(error.into());
    }
}

pub fn rode_read_dir(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let error = v8::String::new(scope, "readDir requires a path argument").unwrap();
        scope.throw_exception(error.into());
        return;
    }

    let path = match args.get(0).to_string(scope) {
        Some(s) => s.to_rust_string_lossy(scope),
        None => {
            let error = v8::String::new(scope, "Invalid path").unwrap();
            scope.throw_exception(error.into());
            return;
        }
    };

    match fs::read_dir(&path) {
        Ok(entries) => {
            let array = v8::Array::new(scope, 0);
            let mut index = 0;

            for entry in entries {
                if let Ok(entry) = entry {
                    let file_name = entry.file_name();
                    let file_name_str = file_name.to_string_lossy();

                    let entry_obj = v8::Object::new(scope);

                    let name_key = v8::String::new(scope, "name").unwrap();
                    let name_val = v8::String::new(scope, &file_name_str).unwrap();
                    entry_obj.set(scope, name_key.into(), name_val.into());

                    let is_dir = entry.path().is_dir();
                    let is_dir_key = v8::String::new(scope, "isDirectory").unwrap();
                    let is_dir_val = v8::Boolean::new(scope, is_dir);
                    entry_obj.set(scope, is_dir_key.into(), is_dir_val.into());

                    let is_file_key = v8::String::new(scope, "isFile").unwrap();
                    let is_file_val = v8::Boolean::new(scope, !is_dir);
                    entry_obj.set(scope, is_file_key.into(), is_file_val.into());

                    array.set_index(scope, index, entry_obj.into());
                    index += 1;
                }
            }

            rv.set(array.into());
        }
        Err(err) => {
            let error_msg = format!("Failed to read directory '{}': {}", path, err);
            let error = v8::String::new(scope, &error_msg).unwrap();
            scope.throw_exception(error.into());
        }
    }
}

pub fn setup_fs(scope: &mut v8::HandleScope) {
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

    let fs_obj = v8::Object::new(scope);
    let fs_key = v8::String::new(scope, "fs").unwrap();

    let read_file_key = v8::String::new(scope, "readFile").unwrap();
    let read_file_func = v8::Function::new(scope, rode_read_file).unwrap();
    fs_obj.set(scope, read_file_key.into(), read_file_func.into());

    let write_file_key = v8::String::new(scope, "writeFile").unwrap();
    let write_file_func = v8::Function::new(scope, rode_write_file).unwrap();
    fs_obj.set(scope, write_file_key.into(), write_file_func.into());

    let exists_key = v8::String::new(scope, "exists").unwrap();
    let exists_func = v8::Function::new(scope, rode_exists).unwrap();
    fs_obj.set(scope, exists_key.into(), exists_func.into());

    let mkdir_key = v8::String::new(scope, "mkdir").unwrap();
    let mkdir_func = v8::Function::new(scope, rode_mkdir).unwrap();
    fs_obj.set(scope, mkdir_key.into(), mkdir_func.into());

    let remove_key = v8::String::new(scope, "remove").unwrap();
    let remove_func = v8::Function::new(scope, rode_remove).unwrap();
    fs_obj.set(scope, remove_key.into(), remove_func.into());

    let read_dir_key = v8::String::new(scope, "readDir").unwrap();
    let read_dir_func = v8::Function::new(scope, rode_read_dir).unwrap();
    fs_obj.set(scope, read_dir_key.into(), read_dir_func.into());

    rode_obj.set(scope, fs_key.into(), fs_obj.into());
}
