#[allow(dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
mod bindings {
    use wit_bindgen::generate;
    generate!(
        { world : "sink-upgrade-severity-world", path : ".wit", additional_derives :
        [serde::Serialize, serde::Deserialize] }
    );
}
mod serialize {
    use crate::bindings;
    struct Component;
    bindings::export!(Component with_types_in bindings);
    use crate::bindings::exports::examples::sink_upgrade_severity::serialize::StatusLocation;
    use crate::bindings::exports::examples::sink_upgrade_severity::serialize::Guest as SerializeOutputInterface;
    impl SerializeOutputInterface for Component {
        fn serialize_key(output: Option<Vec<u8>>) -> Result<Option<Vec<u8>>, String> {
            let Some(output) = output else {
                return Ok(None);
            };
            Ok(Some(output))
        }
        fn serialize_output(output: StatusLocation) -> Result<Vec<u8>, String> {
            serde_json::to_vec(&output).map_err(|err| err.to_string())
        }
    }
}
