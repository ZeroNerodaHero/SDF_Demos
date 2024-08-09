use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    filter_map,
    path = "../../../../crates/sdf-macros/wit/basic-filter-map",
    package = "basic-filter-map",
    namespace = "examples"
)]
fn filter_map_fn(my_input: String) -> Result<Option<String>> {
    Ok(Some(my_input.to_uppercase()))
}

fn main() -> Result<()> {
    Ok(())
}
