use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    filter_map,
    path = "../../../../crates/sdf-macros/wit/basic-filter-map",
    package = "basic-filter-map",
    namespace = "examples"
)]
fn filter_map_fn(my_input: String) -> Result<u64> {
    Ok(my_input.len() as u64)
}

fn main() -> Result<()> {
    Ok(())
}
