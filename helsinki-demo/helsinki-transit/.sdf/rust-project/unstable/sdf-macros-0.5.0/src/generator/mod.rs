use proc_macro2::TokenStream;
use quote::quote;

use crate::ast::{SdfBindgenConfig, SdfOperatorFn, SdfOperatorKind, State, StateType};

pub(crate) fn generate_operator(
    func: &SdfOperatorFn,
    bindgen_config: &SdfBindgenConfig,
) -> TokenStream {
    let bindings = common_bindings_generate(bindgen_config, &func.kind);
    match func.kind {
        SdfOperatorKind::Filter
        | SdfOperatorKind::Map
        | SdfOperatorKind::FilterMap
        | SdfOperatorKind::FlatMap
        | SdfOperatorKind::UpdateState
        | SdfOperatorKind::AssignKey => generate_trait_impl(func, bindgen_config, bindings),
        SdfOperatorKind::AssignTimestamp => {
            generate_assign_timestamp(func, bindgen_config, bindings)
        }
        SdfOperatorKind::Aggregate => generate_aggregate(func, bindgen_config, bindings),
    }
}

fn generate_trait_impl(
    func: &SdfOperatorFn,
    bindgen_config: &SdfBindgenConfig,
    bindings: TokenStream,
) -> TokenStream {
    let state_trait_impl = generate_states_trait_impl(bindgen_config, &func.kind);

    let f_name = &func.name;

    let key_param_sig = if func.has_key {
        let input_ty = func.input_types.first().unwrap();
        Some(quote! { key: #input_ty, })
    } else {
        None
    };
    let input_ty = if func.has_key {
        func.input_types.get(1).unwrap()
    } else {
        func.input_types.first().unwrap()
    };

    let key_param = if func.has_key {
        Some(quote! { key, })
    } else {
        None
    };

    let output_type = &func.output_type;
    let func = &func.func;

    quote! {
        pub mod _sdf_gen_ {
          #bindings

          impl _GuestSdfInterface for Component {
              fn #f_name(#key_param_sig my_input: #input_ty) -> ::std::result::Result<#output_type, String> {
                  super::#f_name(#key_param my_input).map_err(|err| err.to_string())
              }

              #state_trait_impl
          }
        }

        pub use _sdf_gen_::*;

        #func
    }
}

fn generate_assign_timestamp(
    func: &SdfOperatorFn,
    bindgen_config: &SdfBindgenConfig,
    bindings: TokenStream,
) -> TokenStream {
    let state_trait_impl = generate_states_trait_impl(bindgen_config, &func.kind);

    let f_name = &func.name;
    let key_param_sig = if func.has_key {
        let input_ty = func.input_types.first().unwrap();
        Some(quote! { key: #input_ty, })
    } else {
        None
    };
    let input_ty = if func.has_key {
        func.input_types.get(1).unwrap()
    } else {
        func.input_types.first().unwrap()
    };

    let key_param = if func.has_key {
        Some(quote! { key, })
    } else {
        None
    };
    let output_type = &func.output_type;
    let func = &func.func;

    quote! {
        pub mod _sdf_gen_ {
          #bindings

          impl _GuestSdfInterface for Component {
              fn #f_name(#key_param_sig my_input: #input_ty, event_timestamp: i64) -> ::std::result::Result<#output_type, String> {
                  super::#f_name(#key_param my_input, event_timestamp).map_err(|err| err.to_string())
              }

              #state_trait_impl
          }
        }

        pub use _sdf_gen_::*;
        #func
    }
}

fn generate_aggregate(
    func: &SdfOperatorFn,
    bindgen_config: &SdfBindgenConfig,
    bindings: TokenStream,
) -> TokenStream {
    let state_trait_impl = generate_states_trait_impl(bindgen_config, &func.kind);
    let f_name = &func.name;
    let output_type = &func.output_type;
    let func = &func.func;

    quote! {
        pub mod _sdf_gen_ {
          #bindings

          impl _GuestSdfInterface for Component {
              fn #f_name() -> ::std::result::Result<#output_type, String> {
                  super::#f_name().map_err(|err| err.to_string())
              }
              #state_trait_impl
          }
        }

        pub use _sdf_gen_::*;
        #func
    }
}

fn generate_states_trait_impl(
    bindgen_config: &SdfBindgenConfig,
    op_type: &SdfOperatorKind,
) -> TokenStream {
    let v: Vec<_> = bindgen_config
        .states
        .iter()
        .map(|s| generate_state_trait_impl(s, op_type))
        .collect();

    quote! {
        #(#v)*
    }
}

fn generate_state_trait_impl(state_config: &State, op_type: &SdfOperatorKind) -> TokenStream {
    let init_fn_name = &state_config.init_fn_name();
    let state_const_name = &state_config.const_name();

    match &state_config.ty {
        StateType::I32 => match op_type {
            SdfOperatorKind::Aggregate => {
                let rust_type = &state_config.type_name();
                quote! {
                    fn #init_fn_name(initial: #rust_type) {
                        #state_const_name.set(initial).expect("already initialized");
                    }
                }
            }
            _ => {
                quote! {
                    fn #init_fn_name(initial: self::bindings::sdf::value_state::values::Value32) {
                        #state_const_name.set(initial).expect("already initialized");
                    }
                }
            }
        },
        StateType::Table => {
            quote! {
                fn #init_fn_name(initial: ::sdfg::DfValue) {
                    #state_const_name.set(initial.into()).expect("already initialized");
                }
            }
        }
        StateType::ListI32 => {
            quote! {
                fn #init_fn_name(initial: List32) {
                    #state_const_name.set(initial).expect("already initialized");
                }
            }
        }
        StateType::ListDoc => {
            quote! {
                fn #init_fn_name(initial: ListDocument) {
                    #state_const_name.set(initial).expect("already initialized");
                }
            }
        }
        StateType::Document => match op_type {
            SdfOperatorKind::Aggregate => {
                let state_type = &state_config.type_name();
                let item_type = &state_config.item_type();
                quote! {
                    fn #init_fn_name(initial: #state_type) {
                        #state_const_name.set(initial.iter().filter_map(|kv| ::bson::from_reader(kv.value.as_slice()).ok().map(|value|crate::#item_type{ key: kv.key.clone(), value})).collect()).expect("already initialized");
                    }
                }
            }
            _ => quote! {
                fn  #init_fn_name(initial: DocumentValue) {
                    #state_const_name.set(initial).expect("already initialized");
                }
            },
        },
        StateType::Row => {
            quote! {
                fn  #init_fn_name(initial: ::sdrg::bindings::sdf::row_state::row::RowValue) {
                    #state_const_name.set(initial).expect("already initialized");
                }
            }
        }
    }
}

fn generate_state_consts(
    bindgen_config: &SdfBindgenConfig,
    op_type: &SdfOperatorKind,
) -> TokenStream {
    let v: Vec<_> = bindgen_config
        .states
        .iter()
        .map(|s| generate_state_const(s, op_type))
        .collect();

    quote! {
        #(#v)*
    }
}

fn generate_state_const(state_config: &State, op_type: &SdfOperatorKind) -> TokenStream {
    let state_const_name = state_config.const_name();
    let state_name = state_config.state_name();

    match &state_config.ty {
        StateType::I32 => match op_type {
            SdfOperatorKind::Aggregate => {
                let rust_type = &state_config.type_name();
                quote! {
                    static #state_const_name: std::sync::OnceLock<#rust_type> = std::sync::OnceLock::new();
                    pub(crate) fn #state_name() -> &'static #rust_type {
                        #state_const_name.get().expect("not initialized")
                    }
                }
            }
            _ => {
                quote! {
                static #state_const_name: std::sync::OnceLock<self::bindings::sdf::value_state::values::Value32> = std::sync::OnceLock::new();
                pub(crate) fn #state_name() -> &'static self::bindings::sdf::value_state::values::Value32 {
                    #state_const_name.get().expect("not initialized")
                  }
                }
            }
        },
        StateType::Row => {
            let state_config_wrapper_ty = state_config.wrapper_type();
            let item_value_type = state_config.item_value_type();

            let update_fn = if let Some(update_fn) = &state_config.update_fn {
                quote! {
                    pub fn update(&self) -> ::anyhow::Result<()> {
                        #[warn(unused_braces)]
                        #update_fn;

                        Ok(())
                    }
                }
            } else {
                quote! {}
            };
            quote! {
                static #state_const_name: std::sync::OnceLock<::sdrg::bindings::sdf::row_state::row::RowValue> = std::sync::OnceLock::new();
                pub(crate) fn #state_name() -> #state_config_wrapper_ty<'static> {
                    let resource = #state_const_name.get().expect("not initialized");
                    #state_config_wrapper_ty::deserialize_from(resource).expect("deserialize")
                }

                pub struct #state_config_wrapper_ty<'a> {
                    _inner_value: crate::#item_value_type,
                    resource: &'a ::sdrg::bindings::sdf::row_state::row::RowValue,
                }

                impl std::fmt::Debug for #state_config_wrapper_ty<'_> {
                    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        f.debug_struct(stringify!(#state_config_wrapper_ty))
                            .field("value", &self._inner_value)
                            .finish()
                    }
                }

                impl std::ops::Deref for #state_config_wrapper_ty<'_> {
                    type Target = crate::#item_value_type;
                    fn deref(&self) -> &Self::Target {
                        &self._inner_value
                    }
                }

                impl std::ops::DerefMut for #state_config_wrapper_ty<'_> {
                    fn deref_mut(&mut self) -> &mut Self::Target {
                        &mut self._inner_value
                    }
                }

                impl<'a> #state_config_wrapper_ty<'a> {
                    fn deserialize_from(row: &'a ::sdrg::bindings::sdf::row_state::row::RowValue) -> Result<Self, String> {
                        let _inner_value = ::sdrg::RowSerde::deserialize_from(row)
                            .map_err(|err| err.to_string())?;
                        Ok(Self { _inner_value, resource: row })
                    }

                   #update_fn
                }
            }
        }
        StateType::Document => {
            let state_config_wrapper_ty = state_config.wrapper_type();
            let item_value_type = state_config.item_value_type();

            match op_type {
                SdfOperatorKind::Aggregate => {
                    let item_type = &state_config.item_type();
                    quote! {
                        static #state_const_name: std::sync::OnceLock<Vec<crate::#item_type>> = std::sync::OnceLock::new();
                        pub(crate) fn #state_name() -> &'static Vec<crate::#item_type> {
                            #state_const_name.get().expect("not initialized")
                        }
                    }
                }
                _ => quote! {
                    static #state_const_name: std::sync::OnceLock<DocumentValue> = std::sync::OnceLock::new();
                    pub(crate) fn #state_name() ->  #state_config_wrapper_ty<'static> {
                        let resource = #state_const_name.get().expect("not initialized");
                        #state_config_wrapper_ty::deserialize_from(resource).expect("deserialize")
                    }

                    pub struct #state_config_wrapper_ty<'a> {
                        _inner_value: Option<crate::#item_value_type>,
                        resource: &'a DocumentValue,
                    }

                    impl std::fmt::Debug for #state_config_wrapper_ty<'_> {
                        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                            f.debug_struct(stringify!(#state_config_wrapper_ty))
                                .field("value", &self._inner_value)
                                .finish()
                        }
                    }

                    impl<'a> #state_config_wrapper_ty<'a> {
                        fn deserialize_from(resource: &'a DocumentValue) -> Result<Self, String> {
                            let doc_bytes = resource.get();

                            let _inner_value = if doc_bytes.len() == 0 {
                                None
                            } else {
                                Some(::bson::from_reader(doc_bytes.as_slice())
                                .map_err(|err| err.to_string())?)
                            };

                            Ok(Self { _inner_value, resource })
                        }

                       pub fn value_mut(&mut self) -> &mut Option<crate::#item_value_type> {
                            &mut self._inner_value
                        }

                       pub fn update(&self) -> ::anyhow::Result<()> {
                          if let Some(value) = &self._inner_value {
                              let bytes = ::bson::to_vec(value)?;
                              self.resource.set(&bytes);
                          }
                          Ok(())
                        }
                    }
                },
            }
        }
        StateType::ListDoc => {
            let item_value = state_config.item_value_type();
            quote! {
                static #state_const_name: std::sync::OnceLock<ListDocument> = std::sync::OnceLock::new();
                pub(crate) fn #state_name() -> Vec<(String, crate::#item_value)> {
                    #state_const_name.get().expect("not initialized").get().iter().filter_map(|(key, value)|  ::bson::from_reader(value.as_slice()).map(|val| (key.clone(), val)).ok()).collect()
                }

            }
        }
        StateType::ListI32 => {
            quote! {
                static #state_const_name: std::sync::OnceLock<List32> = std::sync::OnceLock::new();
                pub(crate) fn #state_name() -> Vec<(String, i32)> {
                    #state_const_name.get().expect("not initialized").get()
                }

            }
        }
        StateType::Table => {
            quote! {
                static #state_const_name: std::sync::OnceLock<::sdfg::LazyDf> = std::sync::OnceLock::new();
                pub(crate) fn #state_name() -> &'static ::sdfg::LazyDf {
                    #state_const_name.get().expect("not initialized")
                }
            }
        }
    }
}
// TODO: add capability to update `with` configuration
fn common_bindings_generate(
    bindgen_config: &SdfBindgenConfig,
    op_type: &SdfOperatorKind,
) -> TokenStream {
    let world_name = bindgen_config.wit_world();
    let rust_namespace = bindgen_config.rust_namespace();
    let rust_package = bindgen_config.rust_package();
    let rust_interface = bindgen_config.rust_interface();
    let wit_path = &bindgen_config.path;

    let state_consts = generate_state_consts(bindgen_config, op_type);

    let row_binding = if bindgen_config
        .states
        .iter()
        .any(|state| matches!(state.ty, StateType::Row))
        & !matches!(op_type, SdfOperatorKind::Aggregate)
    {
        Some(quote! {
            "sdf:row-state/row": ::sdrg::bindings::sdf::row_state::row,
        })
    } else {
        None
    };

    let df_binding = bindgen_config
        .states
        .iter()
        .any(|state| matches!(state.ty, StateType::Table))
        || (matches!(op_type, SdfOperatorKind::Aggregate)
            && bindgen_config
                .states
                .iter()
                .any(|state| matches!(state.ty, StateType::Row)));
    let table_binding = if df_binding {
        Some(quote! {
            "sdf:df/lazy": ::sdfg::wit::lazy
        })
    } else {
        None
    };

    quote! {
        #[allow(dead_code)]
        #[allow(clippy::all)]
        pub(crate) mod bindings {
            use wit_bindgen::generate;
                generate!({
                    world : #world_name,
                    path : #wit_path,
                    additional_derives:[serde::Serialize, serde::Deserialize],
                    generate_unused_types: true,
                    with : {
                        #row_binding
                        #table_binding
                    }
                });
        }

        #state_consts

        struct Component;

        self::bindings::export!(Component with_types_in bindings);

        pub(crate) use self::bindings::exports::#rust_namespace::#rust_package::#rust_interface::*;

        use self::bindings::exports::#rust_namespace::#rust_package::#rust_interface::Guest as _GuestSdfInterface;
    }
}
