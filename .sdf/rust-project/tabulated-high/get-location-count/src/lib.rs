use sdf_macros::sdf;
use crate::bindings::exports::examples::get_location_count::get_location_count_service::StatusTotal;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(
    aggregate,
    package = "get-location-count",
    namespace = "examples",
    state = (name = "count-by-location", ty = i32),
)]
fn get_location_count() -> Result<StatusTotal, String> {
    let mut cc = count_by_location().clone();
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
