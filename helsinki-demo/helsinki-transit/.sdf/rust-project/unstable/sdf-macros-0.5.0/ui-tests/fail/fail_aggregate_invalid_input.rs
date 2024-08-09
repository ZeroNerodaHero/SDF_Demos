use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    aggregate,
    path = "../../../../crates/sdf-macros/wit/basic-aggregate",
    package = "basic-aggregate",
    namespace = "examples"
)]
fn aggregate_fn(input: String) -> Result<Vec<String>> {
    Ok(vec!["a".to_string(), "b".to_string()])
}

fn main() -> Result<()> {
    Ok(())
}
