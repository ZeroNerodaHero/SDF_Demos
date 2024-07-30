use sdf_macros::sdf;
use crate::bindings::exports::examples::increment_location_count::increment_location_count_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(
    update_state,
    package = "increment-location-count",
    namespace = "examples",
    state = (name = "count-by-location", ty = i32),
)]
fn increment_location_count(log: StatusLog) -> Result<(), String> {
    count_by_location().increment(1);
    Ok(())
}
