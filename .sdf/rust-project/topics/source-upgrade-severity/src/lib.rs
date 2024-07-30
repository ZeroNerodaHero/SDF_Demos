#[allow(dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
mod bindings {
    use wit_bindgen::generate;
    generate!(
        { world : "source-upgrade-severity-world", path : ".wit", additional_derives :
        [serde::Serialize, serde::Deserialize] }
    );
}
mod serialize {
    use crate::bindings;
    struct Component;
    bindings::export!(Component with_types_in bindings);
    use crate::bindings::exports::examples::source_upgrade_severity::deserialize::StatusLocation;
    use crate::bindings::exports::examples::source_upgrade_severity::deserialize::Guest as DeserializeInputInterface;
    impl DeserializeInputInterface for Component {
        fn deserialize_key(
            input_str: Option<String>,
        ) -> Result<Option<Vec<u8>>, String> {
            let Some(input_str) = input_str else {
                return Ok(None);
            };
            let input = input_str.as_bytes();
            Ok(Some(input.to_vec()))
        }
        fn deserialize_input(input_str: String) -> Result<StatusLocation, String> {
            let mut input_str = input_str;
            let bytes = unsafe { input_str.as_mut_vec() };
            simd_json::serde::from_slice(bytes).map_err(|err| err.to_string())
        }
    }
}
