use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    filter,
    path = "../../../../crates/sdf-macros/wit/basic-filter",
    package = "basic-filter",
    namespace = "examples"
)]
fn filter_fn(my_input: String) -> Result<String> {
    Ok(my_input)
}

fn main() -> Result<()> {
    Ok(())
}
