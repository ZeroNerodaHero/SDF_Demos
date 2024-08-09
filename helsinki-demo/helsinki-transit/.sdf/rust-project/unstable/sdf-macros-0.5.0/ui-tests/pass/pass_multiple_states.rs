use anyhow::Result;
use sdf_macros::sdf;

#[sdf(
    map,
    path = "../../../../crates/sdf-macros/wit/multi-state",
    package = "multi-state",
    namespace = "examples",
    state = (
        name = "count-per-word-row",
        ty = row,
        update_fn = {
            let _ = 2 + 2;
        },
    ),
    state = (
        name = "count-per-word-scalar",
        ty = i32,
    )
)]
fn multi_state_fn(word: String) -> Result<String> {
    let mut counter = count_per_word_row();
    counter.last_match = word.clone();
    counter.count += 1;
    let _ = counter.update();

    let counter_scalar = count_per_word_scalar();
    let _ = counter_scalar.increment(1);

    Ok(word)
}

fn main() -> Result<()> {
    Ok(())
}
