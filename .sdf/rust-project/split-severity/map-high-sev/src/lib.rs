use sdf_macros::sdf;
use crate::bindings::exports::examples::map_high_sev::map_high_sev_service::StatusTotal;
use crate::bindings::exports::examples::map_high_sev::map_high_sev_service::StatusLocation;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(flat_map, package = "map-high-sev", namespace = "examples")]
fn map_high_sev(logList: StatusTotal) -> Result<Vec<StatusLocation>, String> {
    Ok(logList)
}
