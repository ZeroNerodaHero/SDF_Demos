use anyhow::Result;
use sdf_macros::sdf;

use crate::bindings::exports::examples::custom_types::custom_types_service::{MyInput, MyOutput};

#[sdf(
    map,
    path = "../../../../crates/sdf-macros/wit/custom-types",
    package = "custom-types",
    namespace = "examples"
)]
fn custom_types_fn(my_input: MyInput) -> Result<MyOutput> {
    Ok(MyOutput {
        body: my_input.body,
    })
}

fn main() -> Result<()> {
    Ok(())
}
