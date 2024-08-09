use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    assign_timestamp,
    path = "../../../../crates/sdf-macros/wit/basic-assign-timestamp",
    package = "basic-assign-timestamp",
    namespace = "examples"
)]
fn assign_timestamp_fn(_my_input: String, event_timestamp: i32) -> Result<i64> {
    Ok(event_timestamp)
}

fn main() -> Result<()> {
    Ok(())
}
