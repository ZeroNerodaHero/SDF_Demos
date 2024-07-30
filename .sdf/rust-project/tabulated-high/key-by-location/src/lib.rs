use sdf_macros::sdf;
use crate::bindings::exports::examples::key_by_location::key_by_location_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(
    assign_key,
    package = "key-by-location",
    namespace = "examples",
    state = (name = "count-by-location", ty = i32),
)]
fn key_by_location(log: StatusLog) -> Result<String, String> {
    Ok(log.location.to_string())
}
