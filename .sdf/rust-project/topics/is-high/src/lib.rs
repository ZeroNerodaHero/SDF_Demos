use sdf_macros::sdf;
use crate::bindings::exports::examples::is_high::is_high_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(filter, package = "is-high", namespace = "examples")]
fn is_high(log: StatusLog) -> Result<bool, String> {
    Ok(log.error == 2)
}
