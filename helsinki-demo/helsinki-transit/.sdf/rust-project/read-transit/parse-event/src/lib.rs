use anyhow::Result;
use sdf_macros::sdf;
use crate::bindings::exports::my_org::parse_event::parse_event_service::Vp;
#[allow(unused_imports)]
use crate::bindings::my_org::helsinki_stat_types::types::*;
#[sdf(filter_map, package = "parse-event", namespace = "my-org")]
fn parse_event(event: String) -> Result<Option<Vp>> {
    match serde_json::from_str(&event) {
        Ok(vp) => Ok(Some(vp)),
        Err(e) => {
            println!("Failed to parse event: {}", e);
            Ok(None)
        }
    }
}
