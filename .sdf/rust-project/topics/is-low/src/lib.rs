use sdf_macros::sdf;
use crate::bindings::exports::examples::is_low::is_low_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(filter, package = "is-low", namespace = "examples")]
fn is_low(log: StatusLog) -> Result<bool, String> {
    Ok(log.error == 1)
}
