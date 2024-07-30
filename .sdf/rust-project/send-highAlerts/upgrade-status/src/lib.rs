use sdf_macros::sdf;
use crate::bindings::exports::examples::upgrade_status::upgrade_status_service::StatusLog;
use crate::bindings::exports::examples::upgrade_status::upgrade_status_service::StatusLocation;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(filter_map, package = "upgrade-status", namespace = "examples")]
fn upgrade_status(status: StatusLocation) -> Result<Option<StatusLog>, String> {
    if status.error_count < 2 {
        return Ok(None);
    }
    let num: u32 = status.location.parse().unwrap();
    Ok(
        Some(StatusLog {
            location: num,
            error: 2,
            error_id: 1000,
            reason: format!(
                "Upgraded Error. Total Errors {} in window", status.error_count
            )
                .to_string(),
        }),
    )
}
