use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    map,
    path = "../../../../crates/sdf-macros/wit/basic-map",
    package = "basic-map",
    namespace = "examples"
)]
fn map_fn(my_input: String) -> Result<String> {
    Ok(my_input.to_uppercase())
}

fn main() -> Result<()> {
    Ok(())
}
