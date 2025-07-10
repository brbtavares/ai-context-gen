//! Rust AST parsing module for the AI Context Generator.
//!
//! This module provides functionality to parse Rust source code and extract
//! structural information such as modules, functions, structs, enums, and
//! implementations using the `syn` crate.

use anyhow::Result;
use quote::ToTokens;
use serde::{Deserialize, Serialize};
use syn::{parse_file, Item, ItemEnum, ItemFn, ItemImpl, ItemMod, ItemStruct, Signature};

/// Complete analysis result for a single Rust source file.
///
/// Contains all structural information extracted from parsing the file's AST,
/// including modules, functions, structs, enums, and implementations.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RustAnalysis {
    /// Path to the analyzed file.
    pub file_path: String,

    /// List of modules defined in the file.
    pub modules: Vec<ModuleInfo>,

    /// List of functions defined in the file.
    pub functions: Vec<FunctionInfo>,

    /// List of structs defined in the file.
    pub structs: Vec<StructInfo>,

    /// List of enums defined in the file.
    pub enums: Vec<EnumInfo>,

    /// List of impl blocks defined in the file.
    pub implementations: Vec<ImplInfo>,

    /// Summary of the AST structure.
    pub ast_summary: String,
}

/// Information about a module definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    /// Name of the module.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// Number of items contained in the module.
    pub items_count: usize,
}

/// Information about a function definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionInfo {
    /// Name of the function.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// Whether the function is async.
    pub is_async: bool,

    /// List of parameter types as strings.
    pub parameters: Vec<String>,

    /// Return type as a string, if any.
    pub return_type: Option<String>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about a struct definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StructInfo {
    /// Name of the struct.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// List of fields in the struct.
    pub fields: Vec<FieldInfo>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about a struct field.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FieldInfo {
    /// Name of the field.
    pub name: String,

    /// Type of the field as a string.
    pub field_type: String,

    /// Visibility modifier for the field.
    pub visibility: String,
}

/// Information about an enum definition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EnumInfo {
    /// Name of the enum.
    pub name: String,

    /// Visibility modifier (pub, pub(crate), private, etc.).
    pub visibility: String,

    /// List of variant names.
    pub variants: Vec<String>,

    /// Documentation comment, if present.
    pub documentation: Option<String>,
}

/// Information about an implementation block.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImplInfo {
    /// The type being implemented (e.g., "MyStruct", "Vec<T>").
    pub target: String,

    /// Name of the trait being implemented, if this is a trait impl.
    pub trait_name: Option<String>,

    /// List of methods defined in the implementation.
    pub methods: Vec<FunctionInfo>,
}

/// Rust source code parser using syn for AST analysis.
///
/// Provides static methods for parsing Rust source files and extracting
/// structural information about the code.
pub struct RustParser;

impl RustParser {
    /// Parses a Rust source file and extracts structural information.
    ///
    /// This method uses the `syn` crate to parse Rust source code into an AST
    /// and then extracts information about modules, functions, structs, enums,
    /// and implementations.
    ///
    /// # Arguments
    ///
    /// * `file_path` - Path to the file being parsed (used for error reporting)
    /// * `content` - Source code content as a string
    ///
    /// # Returns
    ///
    /// A `RustAnalysis` containing all extracted structural information.
    ///
    /// # Errors
    ///
    /// Returns an error if the source code cannot be parsed as valid Rust syntax.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use ai_context_gen::parser::RustParser;
    ///
    /// let source = r#"
    /// pub struct MyStruct {
    ///     pub field: String,
    /// }
    ///
    /// impl MyStruct {
    ///     pub fn new() -> Self {
    ///         Self { field: String::new() }
    ///     }
    /// }
    /// "#;
    ///
    /// let analysis = RustParser::parse_rust_file("example.rs", source).unwrap();
    /// assert_eq!(analysis.structs.len(), 1);
    /// assert_eq!(analysis.implementations.len(), 1);
    /// ```
    pub fn parse_rust_file(file_path: &str, content: &str) -> Result<RustAnalysis> {
        let syntax_tree = parse_file(content)?;

        let mut analysis = RustAnalysis {
            file_path: file_path.to_string(),
            modules: Vec::new(),
            functions: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            implementations: Vec::new(),
            ast_summary: String::new(),
        };

        // Analyze each item in the file
        for item in &syntax_tree.items {
            match item {
                Item::Mod(item_mod) => {
                    analysis.modules.push(Self::parse_module(item_mod));
                }
                Item::Fn(item_fn) => {
                    analysis.functions.push(Self::parse_function(item_fn));
                }
                Item::Struct(item_struct) => {
                    analysis.structs.push(Self::parse_struct(item_struct));
                }
                Item::Enum(item_enum) => {
                    analysis.enums.push(Self::parse_enum(item_enum));
                }
                Item::Impl(item_impl) => {
                    analysis.implementations.push(Self::parse_impl(item_impl));
                }
                _ => {}
            }
        }

        analysis.ast_summary = Self::generate_ast_summary(&analysis);

        Ok(analysis)
    }

    fn parse_module(item: &ItemMod) -> ModuleInfo {
        let items_count = item
            .content
            .as_ref()
            .map(|(_, items)| items.len())
            .unwrap_or(0);

        ModuleInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            items_count,
        }
    }

    fn parse_function(item: &ItemFn) -> FunctionInfo {
        let sig = &item.sig;

        FunctionInfo {
            name: sig.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            is_async: sig.asyncness.is_some(),
            parameters: Self::parse_parameters(sig),
            return_type: Self::parse_return_type(sig),
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_struct(item: &ItemStruct) -> StructInfo {
        let fields = match &item.fields {
            syn::Fields::Named(fields) => fields
                .named
                .iter()
                .map(|f| FieldInfo {
                    name: f.ident.as_ref().unwrap().to_string(),
                    field_type: f.ty.to_token_stream().to_string(),
                    visibility: Self::parse_visibility(&f.vis),
                })
                .collect(),
            syn::Fields::Unnamed(fields) => fields
                .unnamed
                .iter()
                .enumerate()
                .map(|(i, f)| FieldInfo {
                    name: format!("field_{i}"),
                    field_type: f.ty.to_token_stream().to_string(),
                    visibility: Self::parse_visibility(&f.vis),
                })
                .collect(),
            syn::Fields::Unit => Vec::new(),
        };

        StructInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            fields,
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_enum(item: &ItemEnum) -> EnumInfo {
        let variants = item.variants.iter().map(|v| v.ident.to_string()).collect();

        EnumInfo {
            name: item.ident.to_string(),
            visibility: Self::parse_visibility(&item.vis),
            variants,
            documentation: Self::extract_doc_comments(&item.attrs),
        }
    }

    fn parse_impl(item: &ItemImpl) -> ImplInfo {
        let target = item.self_ty.to_token_stream().to_string();
        let trait_name = item
            .trait_
            .as_ref()
            .map(|(_, path, _)| path.to_token_stream().to_string());

        let methods = item
            .items
            .iter()
            .filter_map(|item| {
                if let syn::ImplItem::Fn(method) = item {
                    Some(FunctionInfo {
                        name: method.sig.ident.to_string(),
                        visibility: Self::parse_visibility(&method.vis),
                        is_async: method.sig.asyncness.is_some(),
                        parameters: Self::parse_parameters(&method.sig),
                        return_type: Self::parse_return_type(&method.sig),
                        documentation: Self::extract_doc_comments(&method.attrs),
                    })
                } else {
                    None
                }
            })
            .collect();

        ImplInfo {
            target,
            trait_name,
            methods,
        }
    }

    fn parse_visibility(vis: &syn::Visibility) -> String {
        match vis {
            syn::Visibility::Public(_) => "pub".to_string(),
            syn::Visibility::Restricted(restricted) => {
                format!("pub({})", restricted.path.to_token_stream())
            }
            syn::Visibility::Inherited => "private".to_string(),
        }
    }

    fn parse_parameters(sig: &Signature) -> Vec<String> {
        sig.inputs
            .iter()
            .map(|input| match input {
                syn::FnArg::Receiver(receiver) => {
                    if receiver.mutability.is_some() {
                        "&mut self".to_string()
                    } else {
                        "&self".to_string()
                    }
                }
                syn::FnArg::Typed(typed) => {
                    format!(
                        "{}: {}",
                        typed.pat.to_token_stream(),
                        typed.ty.to_token_stream()
                    )
                }
            })
            .collect()
    }

    fn parse_return_type(sig: &Signature) -> Option<String> {
        match &sig.output {
            syn::ReturnType::Default => None,
            syn::ReturnType::Type(_, ty) => Some(ty.to_token_stream().to_string()),
        }
    }

    fn extract_doc_comments(attrs: &[syn::Attribute]) -> Option<String> {
        let mut doc_comments = Vec::new();

        for attr in attrs {
            if attr.path().is_ident("doc") {
                if let Ok(syn::Lit::Str(lit_str)) = attr.parse_args() {
                    doc_comments.push(lit_str.value());
                }
            }
        }

        if doc_comments.is_empty() {
            None
        } else {
            Some(doc_comments.join("\n"))
        }
    }

    fn generate_ast_summary(analysis: &RustAnalysis) -> String {
        let mut summary = String::new();

        summary.push_str(&format!("# AST Summary for {}\n\n", analysis.file_path));

        if !analysis.modules.is_empty() {
            summary.push_str("## Modules\n");
            for module in &analysis.modules {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} items\n",
                    module.name, module.visibility, module.items_count
                ));
            }
            summary.push('\n');
        }

        if !analysis.structs.is_empty() {
            summary.push_str("## Structs\n");
            for struct_info in &analysis.structs {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} fields\n",
                    struct_info.name,
                    struct_info.visibility,
                    struct_info.fields.len()
                ));
            }
            summary.push('\n');
        }

        if !analysis.enums.is_empty() {
            summary.push_str("## Enums\n");
            for enum_info in &analysis.enums {
                summary.push_str(&format!(
                    "- `{}` ({}) - {} variants\n",
                    enum_info.name,
                    enum_info.visibility,
                    enum_info.variants.len()
                ));
            }
            summary.push('\n');
        }

        if !analysis.functions.is_empty() {
            summary.push_str("## Functions\n");
            for func in &analysis.functions {
                let async_marker = if func.is_async { "async " } else { "" };
                summary.push_str(&format!(
                    "- `{}{}{}` ({})\n",
                    async_marker,
                    func.name,
                    if func.parameters.is_empty() {
                        "()"
                    } else {
                        "(...)"
                    },
                    func.visibility
                ));
            }
            summary.push('\n');
        }

        if !analysis.implementations.is_empty() {
            summary.push_str("## Implementations\n");
            for impl_info in &analysis.implementations {
                let trait_part = impl_info
                    .trait_name
                    .as_ref()
                    .map(|t| format!("{t} for "))
                    .unwrap_or_default();
                summary.push_str(&format!("- `impl {}{}`\n", trait_part, impl_info.target));
            }
        }

        summary
    }
}
