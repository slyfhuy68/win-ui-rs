use std::env;
use std::path::MAIN_SEPARATOR;
use std::path::Path;
use std::path::PathBuf;
use std::process::Command;
pub fn compile_resource(out_dir: &str, prefix: &str, resource: &str) -> String {
    let out_file = format!("{}{}{}.lib", out_dir, MAIN_SEPARATOR, prefix);
    // `.res`es are linkable under MSVC as well as normal libraries.
    if !Command::new::<&Path>(find_windows_sdk_rc().as_ref())
        .args(&["/fo", &out_file, "/I", out_dir])
        .arg(resource)
        .status()
        .expect("Failed to execute RC.EXE (is it in PATH?)")
        .success()
    {
        panic!("RC.EXE failed to compile specified resource file");
    }
    out_file
}
pub fn find_windows_sdk_rc() -> PathBuf {
    let arch = match env::var("HOST").expect("No HOST env var").as_bytes() {
        [b'x', b'8', b'6', b'_', b'6', b'4', ..] => "x64",
        [b'a', b'a', b'r', b'c', b'h', b'6', b'4', ..] => "arm64",
        _ => "x86",
    };

    let output = Command::new("where").arg("rc.exe").output().unwrap();

    if !output.status.success() {
        return Path::new("rc.exe").to_owned();
    }

    let stdout = String::from_utf8(output.stdout).expect("`where.exe` outputed non utf-8 string");
    let first_path = if let Some(x) = stdout.lines().next().and_then(|line| {
        let trimmed = line.trim();
        if !trimmed.is_empty() {
            Some(trimmed.to_string())
        } else {
            None
        }
    }) {
        x
    } else {
        return Path::new("rc.exe").to_owned();
    };

    let mut path = PathBuf::from(first_path);
    if !path.pop() {
        return Path::new("rc.exe").to_owned();
    };
    if !path.pop() {
        return Path::new("rc.exe").to_owned();
    };
    path.push(arch);
    path.push("rc.exe");
    if path.exists() && path.is_file() {
        path
    } else {
        Path::new("rc.exe").to_owned()
    }
}
