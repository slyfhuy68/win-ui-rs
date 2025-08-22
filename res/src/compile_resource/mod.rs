//!本模块及所有子模块的代码修改自[embed_resource 3.0.5](https://crates.io/crates/embed-resource/3.0.5)
use std::env;
use std::path::Path;
#[cfg(all(target_os = "windows", target_env = "msvc"))]
mod windows_msvc;
#[cfg(all(target_os = "windows", not(target_env = "msvc")))]
mod windows_not_msvc;

#[cfg(all(target_os = "windows", target_env = "msvc"))]
use windows_msvc::*;
#[cfg(all(target_os = "windows", not(target_env = "msvc")))]
use windows_not_msvc::*;

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum LinkFor<'r> {
    #[default]
    AllBinaries,
    CustomBins(&'r [&'r str]),
    Tests,
    Benchmarks,
    Examples,
    Everything,
}
pub unsafe fn compile_win32_res<P: AsRef<Path>>(raw_resource_file: P, link_for: LinkFor<'_>) {
    let out_file = raw_resource_file
        .as_ref()
        .to_str()
        .expect("raw_resource_file's path not UTF-8");
    match link_for {
        LinkFor::AllBinaries => {
            println!("cargo:rustc-link-arg-bins={out_file}");
        }
        LinkFor::CustomBins(for_bins) => {
            for bin in for_bins {
                println!("cargo:rustc-link-arg-bin={bin}={out_file}");
            }
        }
        LinkFor::Tests => {
            println!("cargo:rustc-link-arg-tests={out_file}");
        }
        LinkFor::Benchmarks => {
            println!("cargo:rustc-link-arg-benches={out_file}");
        }
        LinkFor::Examples => {
            println!("cargo:rustc-link-arg-examples={out_file}");
        }
        LinkFor::Everything => {
            println!("cargo:rustc-link-arg={out_file}");
        }
    }
}
pub unsafe fn compile_win32_rc<P: AsRef<Path>>(resource_file: P, link_for: LinkFor<'_>) {
    let out_file = get_out_file_name(
        &env::var("OUT_DIR").expect("No OUT_DIR env var"),
        resource_file
            .as_ref()
            .file_stem()
            .expect("resource_file has no stem")
            .to_str()
            .expect("resource_file's stem not UTF-8"),
    );
    compile_resource(
        &out_file,
        resource_file
            .as_ref()
            .to_str()
            .expect("resource_file not UTF-8"),
    );
    unsafe {
        compile_win32_res(out_file, link_for);
    }
}
