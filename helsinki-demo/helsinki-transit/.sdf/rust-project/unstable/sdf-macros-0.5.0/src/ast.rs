use proc_macro2::Span;
use syn::{
    punctuated::Punctuated, spanned::Spanned, Error, Ident, ItemFn, Meta, MetaNameValue, Result,
    Token, Type,
};
use convert_case::{Case, Casing};

pub(crate) enum SdfOperatorKind {
    Filter,
    Map,
    FilterMap,
    FlatMap,
    AssignKey,
    AssignTimestamp,
    Aggregate,
    UpdateState,
}

impl SdfOperatorKind {
    pub fn from_ast(args: &Punctuated<Meta, Token![,]>) -> Result<Self> {
        args.iter()
            .find_map(|item| match item {
                Meta::Path(p) =>  p.segments.iter().last().and_then(|p_it| {
                    match p_it.ident.to_string().as_str() {
                        "filter" => Some(Self::Filter),
                        "map" => Some(Self::Map),
                        "filter_map" => Some(Self::FilterMap),
                        "flat_map" => Some(Self::FlatMap),
                        "assign_key" => Some(Self::AssignKey),
                        "assign_timestamp" => Some(Self::AssignTimestamp),
                        "aggregate" => Some(Self::Aggregate),
                        "update_state" => Some(Self::UpdateState),
                        _ => None,
                    }
                }),
                _ => None,
            })
            .ok_or_else(|| Error::new(args[0].span(), "Missing operator type. Supported: '#[sdf(<op_type>)]' where <op_type> is one of: filter, map, filter_map, flat_map, assign_key, assign_timestamp, aggregate, update_state"))
    }

    /// Expected posible input lengths for each operator
    /// some operators can have multiple input lengths
    /// due to the key argument could be present or not
    fn expected_input_length(&self) -> Vec<usize> {
        match self {
            Self::Filter => vec![1, 2],
            Self::Map => vec![1, 2],
            Self::FilterMap => vec![1, 2],
            Self::FlatMap => vec![1, 2],
            Self::AssignKey => vec![1, 2],
            Self::AssignTimestamp => vec![2, 3],
            Self::Aggregate => vec![0],
            Self::UpdateState => vec![1, 2],
        }
    }

    fn validate_input(&self, func: &ItemFn) -> Result<()> {
        if !self
            .expected_input_length()
            .contains(&func.sig.inputs.len())
        {
            return Err(Error::new(
                func.sig.inputs.span(),
                format!(
                    "Expected {} input arguments. Found {}",
                    self.expected_input_length()
                        .iter()
                        .map(|l| l.to_string())
                        .collect::<Vec<_>>()
                        .join(" or "),
                    func.sig.inputs.len()
                ),
            ));
        }

        if let Self::AssignTimestamp = self {
            let last_arg = func.sig.inputs.iter().last().unwrap();
            if let syn::FnArg::Typed(arg) = last_arg {
                if let syn::Type::Path(path) = &*arg.ty {
                    if path.path.is_ident("i64") {
                        return Ok(());
                    }
                }
                return Err(Error::new(
                    last_arg.span(),
                    "Expected i64 argument for second argument of assign_timestamp",
                ));
            } else {
                return Err(Error::new(
                    last_arg.span(),
                    "Expected i64 argument for second argument of assign_timestamp",
                ));
            }
        }

        Ok(())
    }

    fn validate_output(&self, return_type: &Type) -> Result<()> {
        match self {
            // Check if the return type is Result<bool, _>
            Self::Filter => {
                if let syn::Type::Path(path) = return_type {
                    if path.path.is_ident("bool") {
                        return Ok(());
                    }
                }

                Err(Error::new(
                    return_type.span(),
                    "Expected bool output for filter operator",
                ))
            }
            Self::Map => {
                match return_type {
                    syn::Type::Path(path) => {
                        if path.path.segments.len() == 1 && path.path.segments[0].ident == "Option"
                        {
                            return Err(Error::new(
                            return_type.span(),
                            "Invalid option output type for map operator, maybe you meant filter_map?",
                        ));
                        }
                        return Ok(());
                    }
                    syn::Type::Tuple(tuple) => {
                        if tuple.elems.len() == 2 {
                            return Ok(());
                        }
                    }
                    _ => {}
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid output type for map operator",
                ))
            }
            Self::FilterMap => {
                if let syn::Type::Path(path) = return_type {
                    if path.path.segments.len() == 1 && path.path.segments[0].ident == "Option" {
                        return Ok(());
                    }
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid output type for filter_map operator. It must be an Option<_>",
                ))
            }
            Self::FlatMap => {
                if let syn::Type::Path(path) = return_type {
                    if path.path.segments.len() == 1 && path.path.segments[0].ident == "Vec" {
                        return Ok(());
                    }
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid output type for flat_map operator. It must be a Vec<_>",
                ))
            }
            Self::UpdateState => {
                // No output type expected
                if let syn::Type::Tuple(tuple) = return_type {
                    if tuple.elems.is_empty() {
                        return Ok(());
                    }
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid output type for update_state operator. It must be an unit type `()`",
                ))
            }
            Self::AssignKey => {
                if let syn::Type::Path(_) = return_type {
                    return Ok(());
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid output type for assign key operator",
                ))
            }
            Self::AssignTimestamp => {
                if let syn::Type::Path(path) = return_type {
                    if path.path.is_ident("i64") {
                        return Ok(());
                    }
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid output type for assign_timestamp operator. It must be an i64",
                ))
            }
            Self::Aggregate => {
                match return_type {
                    syn::Type::Path(path) => {
                        if path.path.segments.len() == 1 && path.path.segments[0].ident == "Option"
                        {
                            return Err(Error::new(
                                return_type.span(),
                                "Invalid option output type for aggregate operator",
                            ));
                        }

                        return Ok(());
                    }
                    syn::Type::Tuple(tuple) => {
                        if tuple.elems.len() == 2 {
                            return Ok(());
                        }
                    }
                    _ => {}
                }
                Err(Error::new(
                    return_type.span(),
                    "Invalid option output type for aggregate operator",
                ))
            }
        }
    }
}

pub struct SdfOperatorFn<'a> {
    pub name: &'a Ident,
    pub func: &'a ItemFn,
    pub kind: SdfOperatorKind,
    pub input_types: Vec<syn::Type>,
    pub output_type: syn::Type,
    pub has_key: bool,
}

impl<'a> SdfOperatorFn<'a> {
    pub fn from_ast(func: &'a ItemFn, kind: SdfOperatorKind) -> Result<Self> {
        if func.sig.asyncness.is_some() {
            return Err(Error::new(func.span(), "Sdf function must not be async"));
        }
        let name = &func.sig.ident;

        kind.validate_input(func)?;

        let input_types = func
            .sig
            .inputs
            .iter()
            .map(|arg| {
                if let syn::FnArg::Typed(arg) = arg {
                    Ok((*arg.ty).clone())
                } else {
                    Err(Error::new(
                        arg.span(),
                        "Expected typed argument, found self",
                    ))
                }
            })
            .collect::<Result<Vec<_>>>()?;

        let has_key = match kind {
            SdfOperatorKind::AssignTimestamp => input_types.len() == 3,
            SdfOperatorKind::Aggregate => false,
            _ => input_types.len() == 2,
        };

        let output_type = match &func.sig.output {
            syn::ReturnType::Type(_, return_type) => {
                if let syn::Type::Path(path) = &**return_type {
                    if path.path.segments.len() == 1 && path.path.segments[0].ident == "Result" {
                        if let syn::PathArguments::AngleBracketed(args) =
                            &path.path.segments[0].arguments
                        {
                            if args.args.len() <= 2 && !args.args.is_empty() {
                                if let syn::GenericArgument::Type(ty) = &args.args[0] {
                                    ty.clone()
                                } else {
                                    return Err(Error::new(
                                        return_type.span(),
                                        "Invalid output type. Must be a Result<_, _>",
                                    ));
                                }
                            } else {
                                return Err(Error::new(
                                    return_type.span(),
                                    "Invalid output type. Must be a Result<_, _>",
                                ));
                            }
                        } else {
                            return Err(Error::new(
                                return_type.span(),
                                "Invalid output type. Must be a Result<_, _>",
                            ));
                        }
                    } else {
                        return Err(Error::new(
                            return_type.span(),
                            "Invalid output type. Must be a Result<_, _>",
                        ));
                    }
                } else {
                    return Err(Error::new(
                        return_type.span(),
                        "Invalid output type. Must be a Result<_, _>",
                    ));
                }
            }
            syn::ReturnType::Default => {
                // Expected Result as output
                return Err(Error::new(
                    func.sig.output.span(),
                    "Expected return type Result<_, _>",
                ));
            }
        };

        kind.validate_output(&output_type)?;

        Ok(Self {
            input_types,
            output_type,
            name,
            func,
            kind,
            has_key,
        })
    }
}

pub struct State {
    pub name: String,
    pub ty: StateType,
    pub update_fn: Option<syn::ExprBlock>,
}

impl State {
    pub fn init_fn_name(&self) -> Ident {
        let init_fn_name = format!("init_{}", self.name);
        create_ident(&init_fn_name)
    }

    pub fn const_name(&self) -> Ident {
        let upper_case_name = self.name.to_uppercase();
        create_ident(&upper_case_name)
    }

    pub fn state_name(&self) -> Ident {
        create_ident(&self.name)
    }

    pub fn item_value_type(&self) -> Ident {
        match self.ty {
            StateType::I32 => create_ident("i32"),
            StateType::Row | StateType::Document | StateType::ListDoc => {
                let item_value = format!("{}-item-value", self.name);
                let rust_type = rust_type_case(&item_value);
                create_ident(&rust_type)
            }
            StateType::Table | StateType::ListI32 => {
                let rust_type = rust_type_case(&self.name);
                create_ident(&rust_type)
            }
        }
    }

    pub fn item_type(&self) -> Ident {
        let item = format!("{}-item", self.name);
        let rust_type = rust_type_case(&item);
        create_ident(&rust_type)
    }

    pub fn type_name(&self) -> Ident {
        let rust_type = rust_type_case(&self.name);
        create_ident(&rust_type)
    }

    pub fn wrapper_type(&self) -> Ident {
        let wrapper = format!("{}-wrapper", self.name);
        let rust_type = rust_type_case(&wrapper);
        create_ident(&rust_type)
    }
}
pub enum StateType {
    I32,
    Row,
    Table,
    Document,
    ListI32,
    ListDoc,
}

pub struct SdfBindgenConfig {
    pub path: String,
    pub package: String,
    pub world: String,
    pub interface: String,
    pub namespace: String,
    pub states: Vec<State>,
}

impl SdfBindgenConfig {
    pub fn from_ast(args: &Punctuated<Meta, Token![,]>) -> Result<Self> {
        let path = args
            .iter()
            .find_map(|item| match item {
                Meta::NameValue(nv) => Self::extract_str_config(nv, "path"),
                _ => None,
            })
            .unwrap_or(".wit".to_string());
        let package = args
            .iter()
            .find_map(|item| match item {
                Meta::NameValue(nv) => {
                    Self::extract_str_config(nv, "package")
                }
                _ => None,
            })
            .ok_or_else(|| {
                Error::new(
                    args[0].span(),
                    "Missing package. Try passing the package with: '#[sdf(package = \"<package>\")]",
                )
            })?;
        let world = args
            .iter()
            .find_map(|item| match item {
                Meta::NameValue(nv) => Self::extract_str_config(nv, "world"),
                _ => None,
            })
            .unwrap_or_else(|| format!("{}-world", package));

        let namespace = args
            .iter()
            .find_map(|item| match item {
                Meta::NameValue(nv) => {
                    Self::extract_str_config(nv, "namespace")
                }
                _ => None,
            })
            .ok_or_else(|| {
                Error::new(
                    args[0].span(),
                    "Missing namespace. Try passing the namespace with: '#[sdf(namespace = \"<namespace>\")]",
                )
            })?;

        let interface = args
            .iter()
            .find_map(|item| match item {
                Meta::NameValue(nv) => Self::extract_str_config(nv, "interface"),
                _ => None,
            })
            .unwrap_or_else(|| format!("{}-service", package));

        let states = Self::extract_states(args)?;
        Ok(Self {
            path,
            world,
            namespace,
            interface,
            package,
            states,
        })
    }

    fn extract_states(args: &Punctuated<Meta, Token![,]>) -> Result<Vec<State>> {
        args.iter()
            .filter_map(|item| match item {
                Meta::NameValue(nv) => Self::extract_state(nv).transpose(),
                _ => None,
            })
            .collect()
    }

    fn extract_state(nv: &MetaNameValue) -> Result<Option<State>> {
        if nv.path.is_ident("state") {
            match &nv.value {
                syn::Expr::Tuple(state_config) => {
                    if state_config.elems.len() >= 2 {
                        let mut name = None;
                        let mut ty = None;
                        let mut update_fn = None;

                        for elem in &state_config.elems {
                            match elem {
                                syn::Expr::Assign(assign_expr) => {
                                    if let syn::Expr::Path(path_expr) = &*assign_expr.left {
                                        if path_expr.path.is_ident("name") {
                                            if let syn::Expr::Lit(lit_expr) = &*assign_expr.right {
                                                if let syn::Lit::Str(lit_str) = &lit_expr.lit {
                                                    name = Some(lit_str.value());
                                                }
                                            }
                                        } else if path_expr.path.is_ident("ty") {
                                            if let syn::Expr::Path(path_expr) = &*assign_expr.right
                                            {
                                                if path_expr.path.is_ident("i32") {
                                                    ty = Some(StateType::I32);
                                                } else if path_expr.path.is_ident("row") {
                                                    ty = Some(StateType::Row);
                                                } else if path_expr.path.is_ident("table") {
                                                    ty = Some(StateType::Table);
                                                } else if path_expr.path.is_ident("document") {
                                                    ty = Some(StateType::Document);
                                                } else if path_expr.path.is_ident("list_i32") {
                                                    ty = Some(StateType::ListI32);
                                                } else if path_expr.path.is_ident("list_document") {
                                                    ty = Some(StateType::ListDoc);
                                                } else {
                                                    return Err(Error::new(
                                                        assign_expr.span(),
                                                        "Invalid state type. Supported types: i32, row, table. list_i32, list_document and document",
                                                    ));
                                                }
                                            } else {
                                                return Err(Error::new(
                                                    assign_expr.span(),
                                                    "Invalid state type. Supported types: i32, row, table. list_i32, list_document and document",
                                                ));
                                            }
                                        } else if path_expr.path.is_ident("update_fn") {
                                            // get code implementation of update_fn
                                            if let syn::Expr::Block(block_expr) =
                                                &*assign_expr.right
                                            {
                                                update_fn = Some(block_expr.clone());
                                            } else {
                                                return Err(Error::new(
                                                    assign_expr.right.span(),
                                                    "Invalid state configuration. Expected a block {{ .. }}",
                                                ));
                                            }
                                        } else {
                                            return Err(Error::new(
                                                assign_expr.span(),
                                                "Invalid state configuration",
                                            ));
                                        }
                                    }
                                }
                                _ => {
                                    return Err(Error::new(
                                        elem.span(),
                                        "Invalid state configuration",
                                    ))
                                }
                            }
                        }
                        match (name, ty) {
                            (Some( name), Some(ty)) =>
                            Ok(Some(State { name, ty, update_fn })),
                            (Some(_), _) => {
                                Err(Error::new(
                                    state_config.span(),
                                    "Missing state type. Try passing the state type with: '#[sdf(state = (name = \"<state_name>\", ty = <state_type>))]'",
                                ))
                            },
                            (_, _) => {
                                Err(Error::new(
                                    state_config.span(),
                                    "Missing state name. Try passing the state name with: '#[sdf(state = (name = \"<state_name>\", ty = <state_type>))]'",
                                ))
                            },
                        }
                    } else {
                        Err(Error::new(
                            state_config.span(),
                            "Invalid state configuration. Expected a tuple with two elements: name and ty",
                        ))
                    }
                }
                _ => Err(Error::new(
                    nv.value.span(),
                    "Invalid state configuration. Expected a tuple with two elements: name and ty",
                )),
            }
        } else {
            Ok(None)
        }
    }

    fn extract_str_config(nv: &MetaNameValue, config_name: &str) -> Option<String> {
        if nv.path.is_ident(config_name) {
            match &nv.value {
                syn::Expr::Lit(expr_lit) => {
                    if let syn::Lit::Str(lit_str) = &expr_lit.lit {
                        Some(lit_str.value())
                    } else {
                        None
                    }
                }
                _ => None,
            }
        } else {
            None
        }
    }

    pub fn wit_world(&self) -> &str {
        &self.world
    }

    pub fn rust_namespace(&self) -> Ident {
        create_ident(&self.namespace)
    }

    pub fn rust_package(&self) -> Ident {
        create_ident(&self.package)
    }

    pub fn rust_interface(&self) -> Ident {
        create_ident(&self.interface)
    }
}

fn create_ident(name: &str) -> Ident {
    Ident::new(&name.replace('-', "_"), Span::call_site())
}

pub fn rust_type_case(value: &str) -> String {
    if ["u8", "u16", "u32", "u64", "f32", "f64", "bool"].contains(&value) {
        return value.to_owned();
    }

    if ["s8", "s16", "s32", "s64"].contains(&value) {
        return value.replace('s', "i");
    }

    if ["float32", "float64"].contains(&value) {
        return value.replace("float", "f");
    }

    value.to_case(Case::Pascal)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rust_type_case() {
        assert_eq!(rust_type_case("string"), "String");
        assert_eq!(rust_type_case("bool"), "bool");
        assert_eq!(rust_type_case("s8"), "i8");
        assert_eq!(rust_type_case("s16"), "i16");
        assert_eq!(rust_type_case("s32"), "i32");
        assert_eq!(rust_type_case("s64"), "i64");
        assert_eq!(rust_type_case("u8"), "u8");
        assert_eq!(rust_type_case("u16"), "u16");
        assert_eq!(rust_type_case("u32"), "u32");
        assert_eq!(rust_type_case("u64"), "u64");
        assert_eq!(rust_type_case("float32"), "f32");
        assert_eq!(rust_type_case("float64"), "f64");
        assert_eq!(rust_type_case("my-type"), "MyType");
        assert_eq!(rust_type_case("my-type-2"), "MyType2");
    }
}
