extern crate embed_resource;
use embed_resource::*;
fn main() {
    compile("manifest.rc", NONE).manifest_required().unwrap();
}