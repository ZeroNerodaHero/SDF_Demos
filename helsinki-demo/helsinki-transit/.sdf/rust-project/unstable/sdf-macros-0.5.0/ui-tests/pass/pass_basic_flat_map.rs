use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    flat_map,
    path = "../../../../crates/sdf-macros/wit/basic-flat-map",
    package = "basic-flat-map",
    namespace = "examples"
)]
fn flat_map_fn(my_input: String) -> Result<Vec<String>> {
    Ok(my_input.chars().map(|c| c.to_string()).collect())
}

fn main() -> Result<()> {
    Ok(())
}
