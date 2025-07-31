use capdows_resource::{LinkFor, PreCompilePruduct};
fn main() {
    PreCompilePruduct::from(
        "
#define RT_MANIFEST 24
1 RT_MANIFEST \"app.manifest\""
            .to_string(),
    )
    .compile_for(LinkFor::Everything)
}
