use sdf_macros::sdf;
use crate::bindings::exports::examples::get_location_count_high::get_location_count_high_service::StatusTotal;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(
    aggregate,
    package = "get-location-count-high",
    namespace = "examples",
    state = (name = "count-high", ty = i32),
)]
fn get_location_count_high() -> Result<StatusTotal, String> {
    let mut cc = count_high().clone();
    Ok(
        cc
            .iter()
            .map(|entry| StatusLocation {
                location: entry.key.clone().to_string(),
                error_count: entry.value,
            })
            .collect(),
    )
}
