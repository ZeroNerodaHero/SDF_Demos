use anyhow::Result;
use sdf_macros::sdf;
use crate::bindings::exports::my_org::update_speed::update_speed_service::Vp;
#[allow(unused_imports)]
use crate::bindings::my_org::helsinki_stat_types::types::*;
#[sdf(
    update_state,
    package = "update-speed",
    namespace = "my-org",
    state = (
        name = "vehicle-stat",
        ty = row,
        update_fn = {use
        sdrg::bindings::sdf::row_state::types::Dvalue;self.resource.set(
            &[("events".to_string(), Dvalue::I32(self.events.clone())),
            ("route".to_string(), Dvalue::String(self.route.clone())),
            ("speed".to_string(), Dvalue::Float64(self.speed.clone())),
            ]
        ).map_err(|e|anyhow::anyhow!("Failed to update row: {}", e))?;},
    ),
)]
fn update_speed(event: Vp) -> Result<()> {
    let mut veh = vehicle_stat();
    veh.events += 1;
    veh.route = event.route.clone();
    veh.speed = (veh.speed + event.speed) / 2.0f64;
    veh.update()?;
    Ok(())
}
