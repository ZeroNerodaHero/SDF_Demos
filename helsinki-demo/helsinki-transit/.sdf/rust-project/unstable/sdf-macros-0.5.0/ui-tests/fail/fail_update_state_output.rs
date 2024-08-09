use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    update_state,
    path = "../../../../crates/sdf-macros/wit/basic-update-state",
    package = "basic-update-state",
    namespace = "examples"
)]
fn update_state_fn(my_input: String) -> Result<String> {
    Ok(my_input)
}

fn main() -> Result<()> {
    Ok(())
}
