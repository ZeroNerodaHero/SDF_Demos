use anyhow::Result;
use sdf_macros::sdf;
use crate::bindings::exports::my_org::assign_timestamp::assign_timestamp_service::Vp;
#[allow(unused_imports)]
use crate::bindings::my_org::helsinki_stat_types::types::*;
#[sdf(assign_timestamp, package = "assign-timestamp", namespace = "my-org")]
fn assign_timestamp(event: Vp, _event_time: i64) -> Result<i64> {
    use chrono::{DateTime, FixedOffset};
    fn parse_utc(timestamp: &str) -> anyhow::Result<i64> {
        let time = DateTime::<FixedOffset>::parse_from_str(timestamp, "%+")
            .map_err(|err| anyhow::anyhow!("time parse error: {}", err))?;
        Ok(time.timestamp_millis())
    }
    parse_utc(&event.tst)
}
