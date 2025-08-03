use std::path::MAIN_SEPARATOR;
use std::path::Path;
use std::process::Command;
pub fn compile_resource(out_dir: &str, prefix: &str, resource: &str) -> String {
    let out_file = format!("{}{}{}.lib", out_dir, MAIN_SEPARATOR, prefix);
    // `.res`es are linkable under MSVC as well as normal libraries.
    if !Command::new::<&Path>(Path::new("rc.exe"))
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
