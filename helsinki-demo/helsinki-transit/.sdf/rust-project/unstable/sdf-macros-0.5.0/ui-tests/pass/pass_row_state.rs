use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    update_state,
    path = "../../../../crates/sdf-macros/wit/row-state",
    package = "row-state",
    namespace = "examples",
    state = (
        name = "count-per-word-row",
        ty = row,
        update_fn = {
            let _ = 2 + 2;
        },
    )
)]
fn row_state_fn(word: String) -> Result<()> {
    let mut counter = count_per_word_row();

    counter.last_match = word;
    counter.count += 1;
    let _ = counter.update();
    Ok(())
}

fn main() -> Result<()> {
    Ok(())
}
