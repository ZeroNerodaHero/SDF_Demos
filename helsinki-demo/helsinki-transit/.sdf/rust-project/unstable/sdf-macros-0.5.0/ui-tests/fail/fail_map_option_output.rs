use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    map,
    path = "../../../../crates/sdf-macros/wit/basic-map",
    package = "basic-map",
    namespace = "examples"
)]
fn map_fn(my_input: String) -> Result<Option<String>> {
    Ok(Some(my_input))
}

fn main() -> Result<()> {
    Ok(())
}
