use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    filter,
    path = "../../../../crates/sdf-macros/wit/basic-filter",
    namespace = "examples"
)]
fn filter_fn(my_input: String) -> Result<bool> {
    Ok(my_input.len() > 5)
}

fn main() -> Result<()> {
    Ok(())
}
