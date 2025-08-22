use std::path::MAIN_SEPARATOR;
use std::path::Path;
use std::process::Command;
#[inline]
pub fn get_out_file_name(out_dir: &str, prefix: &str) -> String {
    format!("{out_dir}{MAIN_SEPARATOR}{prefix}.lib")
}
pub fn compile_resource(out_file: &str, resource: &str) {
    // `.res`es are linkable under MSVC as well as normal libraries.
    if !Command::new::<&Path>(Path::new("rc.exe"))
        .args(["/fo", out_file])
        .arg(resource)
        .status()
        .expect("Failed to execute RC.EXE (is it in PATH?)")
        .success()
    {
        panic!("RC.EXE failed to compile specified resource file");
    }
}
