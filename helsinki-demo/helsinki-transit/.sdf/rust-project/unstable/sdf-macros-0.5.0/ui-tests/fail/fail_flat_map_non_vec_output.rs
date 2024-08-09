use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    flat_map,
    path = "../../../../crates/sdf-macros/wit/basic-flat-map",
    package = "basic-flat-map",
    namespace = "examples"
)]
fn flat_map_fn(my_input: String) -> Result<u64> {
    Ok(my_input.len() as u64)
}

fn main() -> Result<()> {
    Ok(())
}
