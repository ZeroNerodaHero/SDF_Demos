use anyhow::Result;
use sdf_macros::sdf;
use crate::bindings::exports::my_org::subset_helsinki::subset_helsinki_service::Vp;
#[allow(unused_imports)]
use crate::bindings::my_org::helsinki_stat_types::types::*;
#[sdf(filter, package = "subset-helsinki", namespace = "my-org")]
fn subset_helsinki(pos: Vp) -> Result<bool, String> {
    Ok(pos.vehicle < 20)
}
