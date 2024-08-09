use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    update_state,
    path = "../../../../crates/sdf-macros/wit/doc-state",
    package = "doc-state",
    namespace = "examples",
    state = (
        name = "count-per-word",
        ty = document,
    )
)]
fn doc_state_fn(_word: String) -> Result<()> {
    let mut counter = count_per_word();

    let _val = counter.value_mut();
    let _ = counter.update();
    Ok(())
}

fn main() -> Result<()> {
    Ok(())
}
