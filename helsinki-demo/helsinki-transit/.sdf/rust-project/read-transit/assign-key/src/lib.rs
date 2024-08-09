use anyhow::Result;
use sdf_macros::sdf;
use crate::bindings::exports::my_org::assign_key::assign_key_service::Vp;
#[allow(unused_imports)]
use crate::bindings::my_org::helsinki_stat_types::types::*;
#[sdf(
    assign_key,
    package = "assign-key",
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
fn assign_key(event: Vp) -> Result<String> {
    Ok(event.vehicle.to_string())
}
