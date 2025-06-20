// Future literal-building helpers will live here.

// ---------------------------------------------------------------------------
// Extracted literal-construction helpers shared across generator crates.
//
// This module centralises the sizable collection of enums, traits and helpers
// that are responsible for converting `serde_json::Value` instances into Rust
// source-code literals using type information from a `syn` AST.
//
// By housing everything in one dedicated module we:
//   * keep domain logic in one place (high cohesion),
//   * dramatically reduce the size of `test.rs` (entropy ↓), and
//   * allow other generator modules to benefit from the same helpers without
//     duplicating implementations.
// ---------------------------------------------------------------------------

use serde_json::Value;
use std::collections::HashMap;
use syn::{File, Type};

// ---------------------------------------------------------------------------
// Primitive helpers
// ---------------------------------------------------------------------------

#[derive(Debug, Clone, PartialEq)]
pub enum PrimitiveType {
    Integral(IntegralType),
    Real(RealType),
    Other(OtherType),
}

#[derive(Debug, Clone, PartialEq)]
pub enum IntegralType {
    // Signed integers
    I8,
    I16,
    I32,
    I64,
    I128,
    ISize,
    // Unsigned integers
    U8,
    U16,
    U32,
    U64,
    U128,
    USize,
}

#[derive(Debug, Clone, PartialEq)]
pub enum RealType {
    F32,
    F64,
}

#[derive(Debug, Clone, PartialEq)]
pub enum OtherType {
    Bool,
    String,
    Str,
}

impl PrimitiveType {
    fn from_type_name(name: &str) -> Option<Self> {
        match name {
            // Signed integers
            "i8" => Some(Self::Integral(IntegralType::I8)),
            "i16" => Some(Self::Integral(IntegralType::I16)),
            "i32" => Some(Self::Integral(IntegralType::I32)),
            "i64" => Some(Self::Integral(IntegralType::I64)),
            "i128" => Some(Self::Integral(IntegralType::I128)),
            "isize" => Some(Self::Integral(IntegralType::ISize)),
            // Unsigned integers
            "u8" => Some(Self::Integral(IntegralType::U8)),
            "u16" => Some(Self::Integral(IntegralType::U16)),
            "u32" => Some(Self::Integral(IntegralType::U32)),
            "u64" => Some(Self::Integral(IntegralType::U64)),
            "u128" => Some(Self::Integral(IntegralType::U128)),
            "usize" => Some(Self::Integral(IntegralType::USize)),
            // Floating point
            "f32" => Some(Self::Real(RealType::F32)),
            "f64" => Some(Self::Real(RealType::F64)),
            // Other primitives
            "bool" => Some(Self::Other(OtherType::Bool)),
            "String" => Some(Self::Other(OtherType::String)),
            "str" => Some(Self::Other(OtherType::Str)),
            _ => None,
        }
    }

    fn to_literal(&self, value: &Value) -> String {
        match self {
            Self::Integral(_) => value.as_i64().expect("Expected integer").to_string(),
            Self::Real(_) => {
                let num = value.as_f64().expect("Expected float");
                if num.fract() == 0.0 {
                    format!("{}.0", num as i64)
                } else {
                    num.to_string()
                }
            }
            Self::Other(other_type) => other_type.to_literal(value),
        }
    }
}

impl OtherType {
    fn to_literal(&self, value: &Value) -> String {
        match self {
            Self::Bool => value.as_bool().expect("Expected boolean").to_string(),
            Self::String => format!(
                "\"{}\".to_string()",
                value.as_str().expect("Expected string")
            ),
            Self::Str => format!("\"{}\"", value.as_str().expect("Expected string")),
        }
    }
}

// ---------------------------------------------------------------------------
// Main conversion traits
// ---------------------------------------------------------------------------

pub trait AsLiteral {
    fn as_literal(&self, value: &Value, ast: &File) -> String;
}

impl AsLiteral for syn::ReturnType {
    fn as_literal(&self, value: &Value, ast: &File) -> String {
        match self {
            syn::ReturnType::Type(_, ty) => ty.as_literal(value, ast),
            syn::ReturnType::Default => panic!("Unit return type not supported"),
        }
    }
}

impl AsLiteral for Type {
    fn as_literal(&self, value: &Value, ast: &File) -> String {
        match self {
            Type::Reference(type_ref) => type_ref.as_literal(value, ast),
            Type::Path(type_path) => type_path.as_literal(value, ast),
            Type::Tuple(type_tuple) => type_tuple.as_literal(value, ast),
            _ => panic!("Unsupported type variant: {:?}", self),
        }
    }
}

impl AsLiteral for syn::TypeReference {
    fn as_literal(&self, value: &Value, ast: &File) -> String {
        match &*self.elem {
            Type::Path(path) if path.type_name() == "str" => {
                PrimitiveType::Other(OtherType::Str).to_literal(value)
            }
            _ => self.elem.as_literal(value, ast),
        }
    }
}

impl AsLiteral for syn::TypePath {
    fn as_literal(&self, value: &Value, ast: &File) -> String {
        let type_name = self.type_name();

        // Try primitive types first
        if let Some(primitive) = PrimitiveType::from_type_name(&type_name) {
            return primitive.to_literal(value);
        }

        // Check for user-defined types in AST
        if let Some(type_def) = ast.find_type(&type_name) {
            return type_def.as_literal(self, value, ast);
        }

        // Generic type parameter like `T`, `K`, `V`…
        if self.path.segments.len() == 1
            && type_name.len() <= 2
            && type_name.chars().all(|c| c.is_ascii_uppercase())
        {
            return infer_literal_from_value(value);
        }

        // Fallback to std-types (Option, Vec, HashMap, …)
        self.as_standard_type(value, ast)
    }
}

// ---------------------------------------------------------------------------
// Fallback literal inference for unknown/generic types
// ---------------------------------------------------------------------------

pub fn infer_literal_from_value(value: &Value) -> String {
    match value {
        Value::Null => "None".to_string(),
        Value::Bool(b) => b.to_string(),
        Value::Number(n) => {
            if let Some(i) = n.as_i64() {
                i.to_string()
            } else if let Some(f) = n.as_f64() {
                if f.fract() == 0.0 {
                    format!("{}.0", f as i64)
                } else {
                    f.to_string()
                }
            } else {
                panic!("Invalid number: {:?}", n)
            }
        }
        Value::String(s) => format!("\"{}\".to_string()", s),
        Value::Array(arr) => {
            let elements: Vec<String> = arr.iter().map(infer_literal_from_value).collect();
            format!("vec![{}]", elements.join(", "))
        }
        Value::Object(obj) => {
            let pairs: Vec<String> = obj
                .iter()
                .map(|(k, v)| format!("(\"{}\".to_string(), {})", k, infer_literal_from_value(v)))
                .collect();
            format!("HashMap::from([{}])", pairs.join(", "))
        }
    }
}

// ---------------------------------------------------------------------------
// Helpers for analysing `syn` types
// ---------------------------------------------------------------------------

pub trait TypeOperations {
    fn type_name(&self) -> String;
    fn generic_args(&self) -> Vec<&Type>;
    fn as_standard_type(&self, value: &Value, ast: &File) -> String;
}

impl TypeOperations for syn::TypePath {
    fn type_name(&self) -> String {
        self.path.segments.last().unwrap().ident.to_string()
    }

    fn generic_args(&self) -> Vec<&Type> {
        match &self.path.segments.last().unwrap().arguments {
            syn::PathArguments::AngleBracketed(args) => args
                .args
                .iter()
                .filter_map(|arg| match arg {
                    syn::GenericArgument::Type(ty) => Some(ty),
                    _ => None,
                })
                .collect(),
            _ => Vec::new(),
        }
    }

    fn as_standard_type(&self, value: &Value, ast: &File) -> String {
        let type_args = self.generic_args();
        match self.type_name().as_str() {
            "Option" => value.as_option(&type_args, ast),
            "Vec" => value.as_vec(&type_args, ast),
            "HashMap" | "BTreeMap" => value.as_map(&self.type_name(), &type_args, ast),
            _ => panic!("Unsupported type: {}", self.type_name()),
        }
    }
}

pub trait ValueOperations {
    fn as_option(&self, type_args: &[&Type], ast: &File) -> String;
    fn as_vec(&self, type_args: &[&Type], ast: &File) -> String;
    fn as_map(&self, type_name: &str, type_args: &[&Type], ast: &File) -> String;
}

impl ValueOperations for Value {
    fn as_option(&self, type_args: &[&Type], ast: &File) -> String {
        if self.is_null() {
            let inner = type_args.first().expect("Option requires type argument");
            match inner {
                Type::Path(path) if path.type_name() == "Vec" => {
                    format!("None::<{}>", type_to_string(inner))
                }
                _ => "None".to_string(),
            }
        } else {
            let inner = type_args.first().expect("Option requires type argument");
            format!("Some({})", inner.as_literal(self, ast))
        }
    }

    fn as_vec(&self, type_args: &[&Type], ast: &File) -> String {
        let elements = if let Some(arr) = self.as_array() {
            arr.clone()
        } else {
            vec![self.clone()]
        };
        let element_type = type_args.first().expect("Vec requires type argument");
        let literals = elements
            .iter()
            .map(|elem| element_type.as_literal(elem, ast))
            .collect::<Vec<_>>();
        format!("vec![{}]", literals.join(", "))
    }

    fn as_map(&self, type_name: &str, type_args: &[&Type], ast: &File) -> String {
        let obj = self.as_object().expect("Expected object for map");
        let key_type = type_args.first().expect("Map requires key type");
        let value_type = type_args.get(1).expect("Map requires value type");

        let pairs = obj
            .iter()
            .map(|(k, v)| {
                let key_value = k.as_typed_key(key_type);
                let key_literal = key_type.as_literal(&key_value, ast);
                let value_literal = value_type.as_literal(v, ast);
                format!("({}, {})", key_literal, value_literal)
            })
            .collect::<Vec<_>>();

        format!("{}::from([{}])", type_name, pairs.join(", "))
    }
}

pub fn type_to_string(ty: &Type) -> String {
    match ty {
        Type::Path(path) => {
            let type_name = path.type_name();
            let args = path.generic_args();
            if args.is_empty() {
                type_name
            } else {
                let arg_strings: Vec<String> = args.iter().map(|arg| type_to_string(arg)).collect();
                format!("{}<{}>", type_name, arg_strings.join(", "))
            }
        }
        Type::Reference(type_ref) => {
            format!("&{}", type_to_string(&type_ref.elem))
        }
        Type::Tuple(type_tuple) => {
            let elements: Vec<String> = type_tuple.elems.iter().map(type_to_string).collect();
            format!("({})", elements.join(", "))
        }
        _ => "Unknown".to_string(),
    }
}

// ---------------------------------------------------------------------------
// Linking back into the syn AST for user-defined types
// ---------------------------------------------------------------------------

pub trait TypeDefinition {
    fn as_literal(&self, type_path: &syn::TypePath, value: &Value, ast: &File) -> String;
}

impl TypeDefinition for syn::Item {
    fn as_literal(&self, type_path: &syn::TypePath, value: &Value, ast: &File) -> String {
        match self {
            syn::Item::Struct(s) => s.as_literal(type_path, value, ast),
            syn::Item::Enum(e) => e.as_literal(type_path, value, ast),
            syn::Item::Type(t) => t.ty.as_literal(value, ast),
            _ => panic!("Unsupported type definition"),
        }
    }
}

impl TypeDefinition for syn::ItemStruct {
    fn as_literal(&self, type_path: &syn::TypePath, value: &Value, ast: &File) -> String {
        let obj = value.as_object().expect("Expected object for struct");
        let generics_map = self.generics.resolve_with(&type_path.generic_args());

        let fields = self
            .fields
            .iter()
            .map(|field| {
                let name = field.field_name();
                let value = obj
                    .get(&name)
                    .unwrap_or_else(|| panic!("Missing field: {}", name));
                let concrete_type = field.ty.resolve_generics(&generics_map);
                let literal = concrete_type.as_literal(value, ast);
                format!("{}: {}", name, literal)
            })
            .collect::<Vec<_>>();

        format!("{} {{ {} }}", type_path.type_name(), fields.join(", "))
    }
}

impl TypeDefinition for syn::ItemEnum {
    fn as_literal(&self, type_path: &syn::TypePath, value: &Value, ast: &File) -> String {
        let obj = value.as_object().expect("Expected object for enum");
        let type_name = type_path.type_name();

        self.variants
            .iter()
            .find_map(|variant| {
                let variant_name = variant.ident.to_string();
                obj.get(&variant_name)
                    .map(|variant_value| match &variant.fields {
                        syn::Fields::Unit => format!("{}::{}", type_name, variant_name),
                        syn::Fields::Unnamed(fields) => match fields.unnamed.len() {
                            0 => format!("{}::{}", type_name, variant_name),
                            1 => {
                                let field_type = &fields.unnamed.first().unwrap().ty;
                                let literal = field_type.as_literal(variant_value, ast);
                                format!("{}::{}({})", type_name, variant_name, literal)
                            }
                            _ => {
                                let arr = variant_value
                                    .as_array()
                                    .expect("Expected array for multi-field tuple variant");
                                if arr.len() != fields.unnamed.len() {
                                    panic!(
                                        "Tuple variant field count mismatch: expected {}, got {}",
                                        fields.unnamed.len(),
                                        arr.len()
                                    );
                                }
                                let literals: Vec<String> = fields
                                    .unnamed
                                    .iter()
                                    .zip(arr.iter())
                                    .map(|(field, value)| field.ty.as_literal(value, ast))
                                    .collect();
                                format!("{}::{}({})", type_name, variant_name, literals.join(", "))
                            }
                        },
                        syn::Fields::Named(fields) => {
                            let variant_obj = variant_value
                                .as_object()
                                .expect("Expected object for named fields");
                            let field_literals = fields
                                .named
                                .iter()
                                .map(|field| {
                                    let name = field.field_name();
                                    let value = variant_obj
                                        .get(&name)
                                        .unwrap_or_else(|| panic!("Missing field: {}", name));
                                    let literal = field.ty.as_literal(value, ast);
                                    format!("{}: {}", name, literal)
                                })
                                .collect::<Vec<_>>();
                            format!(
                                "{}::{} {{ {} }}",
                                type_name,
                                variant_name,
                                field_literals.join(", ")
                            )
                        }
                    })
            })
            .unwrap_or_else(|| panic!("No matching variant found"))
    }
}

// ---------------------------------------------------------------------------
// Misc helper traits used by the above implementations
// ---------------------------------------------------------------------------

pub trait FieldOperations {
    fn field_name(&self) -> String;
}

impl FieldOperations for syn::Field {
    fn field_name(&self) -> String {
        self.ident
            .as_ref()
            .expect("Expected named field")
            .to_string()
    }
}

pub trait GenericOperations {
    fn resolve_with<'a>(&self, type_args: &[&'a Type]) -> HashMap<String, &'a Type>;
}

impl GenericOperations for syn::Generics {
    fn resolve_with<'a>(&self, type_args: &[&'a Type]) -> HashMap<String, &'a Type> {
        self.params
            .iter()
            .enumerate()
            .filter_map(|(i, param)| {
                if let syn::GenericParam::Type(type_param) = param {
                    type_args
                        .get(i)
                        .map(|&ty| (type_param.ident.to_string(), ty))
                } else {
                    None
                }
            })
            .collect()
    }
}

pub trait TypeResolution {
    fn resolve_generics<'a>(&'a self, generics_map: &HashMap<String, &'a Type>) -> &'a Type;
}

impl TypeResolution for Type {
    fn resolve_generics<'a>(&'a self, generics_map: &HashMap<String, &'a Type>) -> &'a Type {
        if let Type::Path(path) = self {
            if path.path.segments.len() == 1 {
                let ident = path.path.segments.first().unwrap().ident.to_string();
                if let Some(&resolved) = generics_map.get(&ident) {
                    return resolved;
                }
            }
        }
        self
    }
}

pub trait AstOperations {
    fn find_type(&self, name: &str) -> Option<&syn::Item>;
}

impl AstOperations for File {
    fn find_type(&self, name: &str) -> Option<&syn::Item> {
        self.items.iter().find(|item| match item {
            syn::Item::Struct(s) => s.ident == name,
            syn::Item::Enum(e) => e.ident == name,
            syn::Item::Type(t) => t.ident == name,
            _ => false,
        })
    }
}

pub trait KeyOperations {
    fn as_typed_key(&self, key_type: &Type) -> Value;
}

impl KeyOperations for str {
    fn as_typed_key(&self, key_type: &Type) -> Value {
        if let Type::Path(path) = key_type {
            if let Some(primitive) = PrimitiveType::from_type_name(&path.type_name()) {
                if matches!(primitive, PrimitiveType::Integral(_)) {
                    if let Ok(num) = self.parse::<i64>() {
                        return Value::Number(serde_json::Number::from(num));
                    }
                }
            }
        }
        Value::String(self.to_string())
    }
}

impl AsLiteral for syn::TypeTuple {
    fn as_literal(&self, value: &Value, ast: &File) -> String {
        let array = value.as_array().expect("Expected array for tuple");

        if self.elems.len() != array.len() {
            panic!(
                "Tuple length mismatch: expected {}, got {}",
                self.elems.len(),
                array.len()
            );
        }

        let elements: Vec<String> = self
            .elems
            .iter()
            .zip(array.iter())
            .map(|(ty, val)| ty.as_literal(val, ast))
            .collect();

        format!("({})", elements.join(", "))
    }
}
