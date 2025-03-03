extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Data, Fields, Lit};

#[proc_macro_derive(ToFormattedString, attributes(to_formatted))]
pub fn derive_to_formatted_string(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let fields = match &input.data {
        Data::Struct(data_struct) => match &data_struct.fields {
            Fields::Named(fields_named) => {
                let field_formats = fields_named.named.iter().map(|field| {
                    let field_name = &field.ident;
                    let field_string = field_name.as_ref().map(|f| f.to_string()).unwrap_or_default();

                    let mut custom_name = field_string.clone();
                    for attr in &field.attrs {
                        if attr.path().is_ident("to_formatted") {
                            attr.parse_nested_meta(|meta| {
                                if meta.path.is_ident("tag") {
                                    if let Ok(lit_str) = meta.value()?.parse::<Lit>() {
                                        if let Lit::Str(lit_str) = lit_str {
                                            custom_name = lit_str.value();
                                        }
                                    }
                                }
                                Ok(())
                            }).ok();
                        }
                    }

                    quote! {
                        format!("{}: {}", #custom_name, self.#field_name.to_formatted_string())
                    }
                });

                quote! {
                    {
                        let mut result = String::from("{ ");
                        let fields: Vec<String> = vec![#(#field_formats),*];
                        result.push_str(&fields.join(", "));
                        result.push_str(" }");
                        result
                    }
                }
            }
            Fields::Unnamed(fields_unnamed) => {
                let field_formats = fields_unnamed.unnamed.iter().enumerate().map(|(i, _)| {
                    let index = syn::Index::from(i);
                    quote! {
                        format!("{}", self.#index.to_formatted_string())
                    }
                });

                quote! {
                    {
                        let mut result = String::from("( ");
                        let fields: Vec<String> = vec![#(#field_formats),*];
                        result.push_str(&fields.join(", "));
                        result.push_str(" )");
                        result
                    }
                }
            }
            Fields::Unit => {
                quote! {
                    String::from("{}")
                }
            }
        },
        Data::Enum(enum_data) => {
            let variants = enum_data.variants.iter().map(|variant| {
                let variant_name = &variant.ident;

                match &variant.fields {
                    Fields::Named(fields_named) => {
                        let field_names = fields_named.named.iter().map(|field| &field.ident);
                        let field_formats = fields_named.named.iter().map(|field| {
                            let field_name = &field.ident;
                            let field_string = field_name.as_ref().map(|f| f.to_string()).unwrap_or_default();

                            let mut custom_name = field_string.clone();
                            for attr in &field.attrs {
                                if attr.path().is_ident("to_formatted") {
                                    attr.parse_nested_meta(|meta| {
                                        if meta.path.is_ident("tag") {
                                            if let Ok(lit_str) = meta.value()?.parse::<Lit>() {
                                                if let Lit::Str(lit_str) = lit_str {
                                                    custom_name = lit_str.value();
                                                }
                                            }
                                        }
                                        Ok(())
                                    }).ok();
                                }
                            }

                            quote! {
                                format!("{}: {}", #custom_name, #field_name.to_formatted_string())
                            }
                        });

                        quote! {
                            Self::#variant_name { #(#field_names),* } => {
                                let mut result = format!("{}: {{ ", stringify!(#variant_name));
                                let fields: Vec<String> = vec![#(#field_formats),*];
                                result.push_str(&fields.join(", "));
                                result.push_str(" }");
                                result
                            }
                        }
                    }
                    Fields::Unnamed(fields_unnamed) => {
                        let field_names = (0..fields_unnamed.unnamed.len()).map(|i| {
                            syn::Ident::new(&format!("field{}", i), proc_macro2::Span::call_site())
                        });
                        let field_formats = field_names.clone().map(|field| {
                            quote! {
                                format!("{}", #field.to_formatted_string())
                            }
                        });

                        quote! {
                            Self::#variant_name ( #(#field_names),* ) => {
                                let mut result = format!("{}(", stringify!(#variant_name));
                                let fields: Vec<String> = vec![#(#field_formats),*];
                                result.push_str(&fields.join(", "));
                                result.push(')');
                                result
                            }
                        }
                    }
                    Fields::Unit => {
                        quote! {
                            Self::#variant_name => format!("{}", stringify!(#variant_name))
                        }
                    }
                }
            });

            quote! {
                match self {
                    #(#variants),*
                }
            }
        }
        Data::Union(_) => {
            panic!("Only structs and enums are supported!")
        }
    };

    let expanded = quote! {
        impl ToFormattedString for #name {
            fn to_formatted_string(&self) -> String {
                #fields
            }
        }
    };

    TokenStream::from(expanded)
}
