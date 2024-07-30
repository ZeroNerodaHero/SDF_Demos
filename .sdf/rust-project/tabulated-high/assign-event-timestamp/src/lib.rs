use sdf_macros::sdf;
use crate::bindings::exports::examples::assign_event_timestamp::assign_event_timestamp_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(assign_timestamp, package = "assign-event-timestamp", namespace = "examples")]
fn assign_event_timestamp(log: StatusLog, event_time: i64) -> Result<i64, String> {
    Ok(event_time)
}
