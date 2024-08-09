use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    assign_key,
    path = "../../../../crates/sdf-macros/wit/basic-assign-key",
    package = "basic-assign-key",
    namespace = "examples"
)]
fn assign_key_fn(my_input: String) -> Result<String> {
    my_input
        .chars()
        .next()
        .map(|c| c.to_uppercase().to_string())
        .ok_or_else(|| anyhow::anyhow!("empty string"))
}

fn main() -> Result<()> {
    Ok(())
}
