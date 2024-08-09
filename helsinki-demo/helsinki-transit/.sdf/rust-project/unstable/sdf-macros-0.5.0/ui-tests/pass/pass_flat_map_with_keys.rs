use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    flat_map,
    path = "../../../../crates/sdf-macros/wit/flat-map-with-keys",
    package = "flat-map-with-keys",
    namespace = "examples"
)]
fn flat_map_fn(key: Option<String>, my_input: String) -> Result<Vec<(Option<String>, String)>> {
    Ok(vec![(key.clone(), my_input.to_uppercase())])
}

fn main() -> Result<()> {
    Ok(())
}
