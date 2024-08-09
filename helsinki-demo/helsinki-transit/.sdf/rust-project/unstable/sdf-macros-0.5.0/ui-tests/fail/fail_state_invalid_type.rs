use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    update_state,
    path = "../../../../crates/sdf-macros/wit/scalar-state",
    package = "scalar-state",
    namespace = "examples",
    state = (
        name = "count-per-word-scalar",
        ty = u32,
    )
)]
fn scalar_state_fn(_my_input: String) -> Result<()> {
    let state = count_per_word_scalar();
    let _value_scalar = state.increment(1);
    Ok(())
}

fn main() -> Result<()> {
    Ok(())
}
