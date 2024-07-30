use sdf_macros::sdf;
use crate::bindings::exports::examples::increment_location_count_high::increment_location_count_high_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(
    update_state,
    package = "increment-location-count-high",
    namespace = "examples",
    state = (name = "count-high", ty = i32),
)]
fn increment_location_count_high(log: StatusLog) -> Result<(), String> {
    count_high().increment(1);
    Ok(())
}
