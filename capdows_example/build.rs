extern crate capdows_resource;
use capdows_resource::*;

fn main() {
    if cfg!(target_os = "windows") {
        let mut res = WindowsResource::new();
        res.set_manifest_file("app.manifest");
        res.compile().unwrap();
    }
}