use rusty_v8 as v8;
use std::path::{Path, PathBuf};

pub fn setup_path(scope: &mut v8::HandleScope) {
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

    // Create path object
    let path_obj = v8::Object::new(scope);
    let path_key = v8::String::new(scope, "path").unwrap();
    rode_obj.set(scope, path_key.into(), path_obj.into());

    // path.join(...paths)
    let join_key = v8::String::new(scope, "join").unwrap();
    let join_func = v8::Function::new(scope, path_join).unwrap();
    path_obj.set(scope, join_key.into(), join_func.into());

    // path.resolve(...paths)
    let resolve_key = v8::String::new(scope, "resolve").unwrap();
    let resolve_func = v8::Function::new(scope, path_resolve).unwrap();
    path_obj.set(scope, resolve_key.into(), resolve_func.into());

    // path.dirname(path)
    let dirname_key = v8::String::new(scope, "dirname").unwrap();
    let dirname_func = v8::Function::new(scope, path_dirname).unwrap();
    path_obj.set(scope, dirname_key.into(), dirname_func.into());

    // path.basename(path, ext?)
    let basename_key = v8::String::new(scope, "basename").unwrap();
    let basename_func = v8::Function::new(scope, path_basename).unwrap();
    path_obj.set(scope, basename_key.into(), basename_func.into());

    // path.extname(path)
    let extname_key = v8::String::new(scope, "extname").unwrap();
    let extname_func = v8::Function::new(scope, path_extname).unwrap();
    path_obj.set(scope, extname_key.into(), extname_func.into());

    // path.isAbsolute(path)
    let is_absolute_key = v8::String::new(scope, "isAbsolute").unwrap();
    let is_absolute_func = v8::Function::new(scope, path_is_absolute).unwrap();
    path_obj.set(scope, is_absolute_key.into(), is_absolute_func.into());

    // path.normalize(path)
    let normalize_key = v8::String::new(scope, "normalize").unwrap();
    let normalize_func = v8::Function::new(scope, path_normalize).unwrap();
    path_obj.set(scope, normalize_key.into(), normalize_func.into());

    // path.relative(from, to)
    let relative_key = v8::String::new(scope, "relative").unwrap();
    let relative_func = v8::Function::new(scope, path_relative).unwrap();
    path_obj.set(scope, relative_key.into(), relative_func.into());

    // path.sep (path separator)
    let sep_key = v8::String::new(scope, "sep").unwrap();
    let sep_value = if cfg!(windows) { "\\" } else { "/" };
    let sep_str = v8::String::new(scope, sep_value).unwrap();
    path_obj.set(scope, sep_key.into(), sep_str.into());

    // path.delimiter (PATH delimiter)
    let delimiter_key = v8::String::new(scope, "delimiter").unwrap();
    let delimiter_value = if cfg!(windows) { ";" } else { ":" };
    let delimiter_str = v8::String::new(scope, delimiter_value).unwrap();
    path_obj.set(scope, delimiter_key.into(), delimiter_str.into());
}

fn path_join(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let mut path = PathBuf::new();

    for i in 0..args.length() {
        if let Some(arg_str) = args.get(i).to_string(scope) {
            let segment = arg_str.to_rust_string_lossy(scope);
            if !segment.is_empty() {
                path.push(segment);
            }
        }
    }

    let result = path.to_string_lossy().to_string();
    let result_str = v8::String::new(scope, &result).unwrap();
    rv.set(result_str.into());
}

fn path_resolve(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    let mut path = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));

    for i in 0..args.length() {
        if let Some(arg_str) = args.get(i).to_string(scope) {
            let segment = arg_str.to_rust_string_lossy(scope);
            if !segment.is_empty() {
                let segment_path = PathBuf::from(segment);
                if segment_path.is_absolute() {
                    path = segment_path;
                } else {
                    path.push(segment_path);
                }
            }
        }
    }

    let result = path
        .canonicalize()
        .unwrap_or(path)
        .to_string_lossy()
        .to_string();
    let result_str = v8::String::new(scope, &result).unwrap();
    rv.set(result_str.into());
}

fn path_dirname(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result_str = v8::String::new(scope, ".").unwrap();
        rv.set(result_str.into());
        return;
    }

    if let Some(path_str) = args.get(0).to_string(scope) {
        let path_string = path_str.to_rust_string_lossy(scope);
        let path = Path::new(&path_string);
        let dirname = path.parent().unwrap_or(Path::new("."));
        let result = dirname.to_string_lossy().to_string();
        let result_str = v8::String::new(scope, &result).unwrap();
        rv.set(result_str.into());
    }
}

fn path_basename(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result_str = v8::String::new(scope, "").unwrap();
        rv.set(result_str.into());
        return;
    }

    if let Some(path_str) = args.get(0).to_string(scope) {
        let path_string = path_str.to_rust_string_lossy(scope);
        let path = Path::new(&path_string);
        let mut basename = path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        // If second argument is provided (extension), remove it
        if args.length() >= 2 {
            if let Some(ext_str) = args.get(1).to_string(scope) {
                let ext = ext_str.to_rust_string_lossy(scope);
                if basename.ends_with(&ext) {
                    basename = basename[..basename.len() - ext.len()].to_string();
                }
            }
        }

        let result_str = v8::String::new(scope, &basename).unwrap();
        rv.set(result_str.into());
    }
}

fn path_extname(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result_str = v8::String::new(scope, "").unwrap();
        rv.set(result_str.into());
        return;
    }

    if let Some(path_str) = args.get(0).to_string(scope) {
        let path_string = path_str.to_rust_string_lossy(scope);
        let path = Path::new(&path_string);
        let ext = path
            .extension()
            .map(|e| format!(".{}", e.to_string_lossy()))
            .unwrap_or_default();
        let result_str = v8::String::new(scope, &ext).unwrap();
        rv.set(result_str.into());
    }
}

fn path_is_absolute(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result = v8::Boolean::new(scope, false);
        rv.set(result.into());
        return;
    }

    if let Some(path_str) = args.get(0).to_string(scope) {
        let path_string = path_str.to_rust_string_lossy(scope);
        let path = Path::new(&path_string);
        let is_abs = path.is_absolute();
        let result = v8::Boolean::new(scope, is_abs);
        rv.set(result.into());
    }
}

fn path_normalize(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 1 {
        let result_str = v8::String::new(scope, ".").unwrap();
        rv.set(result_str.into());
        return;
    }

    if let Some(path_str) = args.get(0).to_string(scope) {
        let path_string = path_str.to_rust_string_lossy(scope);
        let path = PathBuf::from(path_string);
        let normalized = path.clean();
        let result = normalized.to_string_lossy().to_string();
        let result_str = v8::String::new(scope, &result).unwrap();
        rv.set(result_str.into());
    }
}

fn path_relative(
    scope: &mut v8::HandleScope,
    args: v8::FunctionCallbackArguments,
    mut rv: v8::ReturnValue,
) {
    if args.length() < 2 {
        let result_str = v8::String::new(scope, ".").unwrap();
        rv.set(result_str.into());
        return;
    }

    if let (Some(from_str), Some(to_str)) =
        (args.get(0).to_string(scope), args.get(1).to_string(scope))
    {
        let from_string = from_str.to_rust_string_lossy(scope);
        let to_string = to_str.to_rust_string_lossy(scope);

        let from_path = PathBuf::from(from_string);
        let to_path = PathBuf::from(to_string.clone());

        if let Ok(relative) = to_path.strip_prefix(&from_path) {
            let result = relative.to_string_lossy().to_string();
            let result_str = v8::String::new(scope, &result).unwrap();
            rv.set(result_str.into());
        } else {
            // Fallback to returning the 'to' path if strip_prefix fails
            let result_str = v8::String::new(scope, &to_string).unwrap();
            rv.set(result_str.into());
        }
    }
}

// Extension trait for path normalization
trait PathClean {
    fn clean(&self) -> PathBuf;
}

impl PathClean for PathBuf {
    fn clean(&self) -> PathBuf {
        let mut components = Vec::new();

        for component in self.components() {
            match component {
                std::path::Component::CurDir => {
                    // Skip "." components
                }
                std::path::Component::ParentDir => {
                    // ".." component - remove last component if possible
                    if !components.is_empty()
                        && components.last() != Some(&std::path::Component::ParentDir)
                    {
                        components.pop();
                    } else {
                        components.push(component);
                    }
                }
                _ => {
                    components.push(component);
                }
            }
        }

        components.iter().collect()
    }
}
