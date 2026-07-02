use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Expr, Fields, Path, Type};

struct MemoryAttr {
    block: Path,
    index_expr: Option<Expr>,
}

pub(crate) fn derive_memory_access(input: DeriveInput) -> TokenStream {
    let struct_name = &input.ident;

    let Data::Struct(data_struct) = &input.data else {
        panic!("MemoryAccess can only be derived for structs");
    };

    let Fields::Named(fields) = &data_struct.fields else {
        panic!("MemoryAccess requires named fields");
    };

    let mut read_arms = quote!();
    let mut write_arms = quote!();

    for field in &fields.named {
        let field_ident = field.ident.as_ref().unwrap();
        let ty = &field.ty;

        let (default_block, is_refcell) =
            extract_type_info(ty).expect("Could not determine the inner type of the field");

        let mut parsed_attrs = Vec::new();
        for attr in &field.attrs {
            if attr.path().is_ident("memory") {
                let mut explicit_block = None;
                let mut index_expr = None;

                if matches!(attr.meta, syn::Meta::List(_)) {
                    attr.parse_nested_meta(|meta| {
                        if meta.path.is_ident("block") {
                            explicit_block = Some(meta.value()?.parse()?);
                            Ok(())
                        } else if meta.path.is_ident("index") {
                            let expr_str: syn::LitStr = meta.value()?.parse()?;
                            index_expr = Some(syn::parse_str(&expr_str.value())?);
                            Ok(())
                        } else {
                            Err(meta.error("unrecognized memory attribute"))
                        }
                    })
                    .unwrap();
                }

                let final_block = explicit_block.unwrap_or_else(|| default_block.clone());

                parsed_attrs.push(MemoryAttr {
                    block: final_block,
                    index_expr,
                });
            }
        }

        for mem_attr in parsed_attrs {
            let block_path = mem_attr.block;

            let index = mem_attr
                .index_expr
                .unwrap_or_else(|| syn::parse_quote!(index));

            let read_access = if is_refcell {
                quote! { self.#field_ident.borrow().read(#index) }
            } else {
                quote! { self.#field_ident.read(#index) }
            };

            read_arms.extend(quote! {
                #block_path::BLOCK_ID => { #read_access }
            });

            if is_refcell {
                write_arms.extend(quote! {
                    #block_path::BLOCK_ID => {
                        self.#field_ident
                            .borrow_mut()
                            .write(#index, value)
                            .then_some(value)
                    }
                });
            }
        }
    }

    let expanded = quote! {
        impl<'a> MemoryAccess for #struct_name<'a> {
            fn read(
                &self,
                ctx: &RuntimeContext,
                block_id: u64,
                index: usize,
            ) -> Option<sonorust_ir::IRValue> {
                match block_id {
                    #read_arms
                    other => {
                        tracing::warn!(
                            "Attempted to read from unknown block ID {}, index {}",
                            other,
                            index
                        );
                        None
                    }
                }
            }

            fn write(
                &self,
                ctx: &RuntimeContext,
                block_id: u64,
                index: usize,
                value: sonorust_ir::IRValue,
            ) -> Option<sonorust_ir::IRValue> {
                match block_id {
                    #write_arms
                    other => {
                        tracing::warn!(
                            "Attempted to write to unknown block ID {}, index {}",
                            other,
                            index
                        );
                        None
                    }
                }
            }
        }
    };

    TokenStream::from(expanded)
}

fn extract_type_info(ty: &Type) -> Option<(Path, bool)> {
    let inner_ty = if let Type::Reference(type_ref) = ty {
        &*type_ref.elem
    } else {
        ty
    };

    if let Type::Path(type_path) = inner_ty {
        let last_segment = type_path.path.segments.last()?;

        if last_segment.ident == "RefCell" {
            if let syn::PathArguments::AngleBracketed(args) = &last_segment.arguments {
                if let Some(syn::GenericArgument::Type(Type::Path(inner_path))) = args.args.first()
                {
                    return Some((inner_path.path.clone(), true));
                }
            }
        } else {
            return Some((type_path.path.clone(), false));
        }
    }
    None
}
