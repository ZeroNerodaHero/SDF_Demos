use sdf_macros::sdf;
use crate::bindings::exports::examples::key_by_location_high::key_by_location_high_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(
    assign_key,
    package = "key-by-location-high",
    namespace = "examples",
    state = (name = "count-high", ty = i32),
)]
fn key_by_location_high(log: StatusLog) -> Result<String, String> {
    Ok(log.location.to_string())
}
