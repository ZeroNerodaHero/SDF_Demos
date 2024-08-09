#[allow(dead_code)]
#[rustfmt::skip]
#[allow(clippy::all)]
pub mod bindings {
    use wit_bindgen::generate;

    generate!({
        world: "arrow-world",
        path: "wit",
        additional_derives: [PartialEq, Clone],
    });
}

mod expr;
pub use expr::*;

pub mod wit {
    pub use crate::bindings::sdf::df::lazy;
}
