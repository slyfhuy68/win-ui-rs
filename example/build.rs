extern crate capdows_resource;
// use either::Either;
// use either::Either::*;
use capdows_resource::{version::*, image::*};
use capdows_resource::*;
use std::collections::HashMap;
fn main() -> Result<(), Box<dyn std::error::Error>> {
    //print!("cargo::warning={} {}\n", is_debug, env::var("DEBUG").unwrap()=="true");
    // compile("manifest.rc", NONE).manifest_required().unwrap();
    let vstr = Version {
        product_internal_version: (0u16, 0u16, 0u16, 1u16),
        file_internal_version: None,
        debug: None,
        pre_release: false,
        pached: false,
        variant: ProductVariant::default(),
        strings: HashMap::from([
            (LangID::new("0804")?, StringInfo::default()),
            // (LangID::new("0809")?, StringInfo::default()),
        ]),
        os: Default::default(),
        ftype: Default::default(),
    }.pre_compile()?;
    let icon1 = Icon(NumberId(1), "./res/ICON1.ico".into()).pre_compile()?;
    let icon2 = Icon(NumberId(2), "./res/ICON2.ico".into()).pre_compile()?;
    let icon3 = Icon(NumberId(3), "./res/ICON3.ico".into()).pre_compile()?;
    let cursor1 = Cursor(NumberId(4), "./res/CURSOR1.cur".into()).pre_compile()?;
    compile_all!(vstr, icon1, icon2, icon3, cursor1)?;
    // print!(
    //     "cargo::warning={}nnnn{}\n",
    //     dest_path.to_str().unwrap(),
    //     LangID::new("0804")?.to_hex_string()
    // );
    Ok(())
}
