use sdf_macros::sdf;
use crate::bindings::exports::examples::randencode::randencode_service::Car;
use crate::bindings::exports::examples::randencode::randencode_service::StatusLog;
#[allow(unused_imports)]
use crate::bindings::examples::car_processing_types::types::*;
#[sdf(map, package = "randencode", namespace = "examples")]
fn randencode(car: Car) -> Result<StatusLog, String> {
    use rand::Rng;
    const MAX_LOCATION: u32 = 10;
    let mut rng = rand::thread_rng();
    let errorCode: u32 = rng.gen::<u32>() % 3;
    let mut errorMsg: String = "No Error".to_string();
    if errorCode == 1 {
        errorMsg = "Low Error".to_string();
    } else {
        errorMsg = "High Error".to_string();
    }
    let log: StatusLog = StatusLog {
        location: rng.gen::<u32>() % MAX_LOCATION,
        error: errorCode,
        error_id: rng.gen(),
        reason: errorMsg,
    };
    Ok(log)
}
