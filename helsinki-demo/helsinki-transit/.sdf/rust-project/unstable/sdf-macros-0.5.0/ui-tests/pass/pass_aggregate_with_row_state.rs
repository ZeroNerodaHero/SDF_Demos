use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    aggregate,
    path = "../../../../crates/sdf-macros/wit/aggregate-with-state",
    package = "aggregate-with-state",
    namespace = "examples",
    state = (
        name = "my-state",
        ty = table
    )
)]
fn aggregate_fn() -> Result<Vec<String>> {
    let my_state = my_state();
    let _shape = my_state.shape();
    Ok(vec!["a".to_string(), "b".to_string()])
}

fn main() -> Result<()> {
    Ok(())
}
