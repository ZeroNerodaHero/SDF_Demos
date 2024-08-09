use anyhow::Result;
use sdf_macros::sdf;
use crate::bindings::exports::my_org::compute_top_vehicle::compute_top_vehicle_service::TopVehicle;
#[allow(unused_imports)]
use crate::bindings::my_org::helsinki_stat_types::types::*;
#[sdf(
    aggregate,
    package = "compute-top-vehicle",
    namespace = "my-org",
    state = (
        name = "vehicle-stat",
        ty = table,
        update_fn = {use
        sdrg::bindings::sdf::row_state::types::Dvalue;self.resource.set(
            &[("events".to_string(), Dvalue::I32(self.events.clone())),
            ("route".to_string(), Dvalue::String(self.route.clone())),
            ("speed".to_string(), Dvalue::Float64(self.speed.clone())),
            ]
        ).map_err(|e|anyhow::anyhow!("Failed to update row: {}", e))?;},
    ),
)]
fn compute_top_vehicle() -> Result<TopVehicle> {
    let mut stat = vehicle_stat();
    let top5 = stat.sql("select * from vehicle_stat order by speed desc limit 5")?;
    let rows = top5.rows()?;
    let mut top_vehicles = vec![];
    let key = top5.key()?;
    let speed_value = top5.col("speed")?;
    while rows.next() {
        let vehicle = rows.str(&key)?;
        let speed = rows.f64(&speed_value)?;
        top_vehicles.push(VpStat { vehicle, speed });
    }
    Ok(top_vehicles)
}
