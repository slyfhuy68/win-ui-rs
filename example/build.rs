use std::env;
extern crate embed_resource;
extern crate capdows_resource;
use std::collections::HashMap;
use capdows_resource::*;
use embed_resource::*;
use std::path::Path;
use std::fs::File;
use std::io::Write;
fn main() -> Result<(), Box<dyn std::error::Error>>{
    //print!("cargo::warning={} {}\n", is_debug, env::var("DEBUG").unwrap()=="true");
    compile("manifest.rc", NONE).manifest_required().unwrap();
    let vstr = Version{
        product_internal_version: (0u16, 0u16, 0u16, 1u16), 
        file_internal_version: None,
        debug: None, 
        pre_release: false, 
        pached: false, 
        variant: ProductVariant::default(), 
        strings: HashMap::from([
            (LangID::new("0804")?, StringInfo::default()),
            (LangID::new("0809")?, StringInfo::default()),
        ]), 
        os: Default::default(), 
        ftype: Default::default()
    }.pre_compile()?.get();
    let out_dir = env::var("OUT_DIR").unwrap();
    let dest_path = Path::new(&out_dir).join("resource.rc");
    let mut f = File::create(&dest_path).expect("无法创建文件");
    f.write_all(vstr.as_bytes()).expect("无法写入文件");
    compile(dest_path.to_str().unwrap(), NONE).manifest_required().unwrap();
    print!("cargo::warning={}nnnn{}\n", dest_path.to_str().unwrap(), LangID::new("0804")?.to_hex_string());
    Ok(())
}
