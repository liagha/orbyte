extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DataEnum, DataStruct, DeriveInput, Fields};

#[proc_macro_derive(Orbyte)]
pub fn derive_orbyte(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    let (serialization, deserialization) = match &input.data {
        Data::Struct(data_struct) => handle_struct(name, data_struct),
        Data::Enum(data_enum) => handle_enum(name, data_enum),
        _ => unimplemented!(),
    };

    let expanded = quote! {
        impl Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                #serialization
            }
        }

        impl Deserialize for #name {
            fn deserialize(bytes: &[u8]) -> Option<Self> {
                #deserialization
            }
        }
    };

    TokenStream::from(expanded)
}

fn handle_struct(
    name: &syn::Ident,
    data_struct: &DataStruct,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    match &data_struct.fields {
        Fields::Named(fields) => {
            let mut serialize_statements = Vec::new();
            let mut deserialize_statements = Vec::new();
            let mut field_names = Vec::new();

            for field in &fields.named {
                let field_name = &field.ident;
                let field_type = &field.ty;
                serialize_statements.push(quote! { bytes.extend(self.#field_name.serialize()); });
                deserialize_statements.push(quote! {
                    let #field_name: #field_type = Deserialize::deserialize(&bytes[offset..])?;
                    offset += #field_name.serialize().len();
                });
                field_names.push(field_name);
            }

            (
                quote! {
                    let mut bytes = Vec::new();
                    #(#serialize_statements)*
                    bytes
                },
                quote! {
                    let mut offset = 0;
                    #(#deserialize_statements)*
                    Some(Self { #(#field_names),* })
                },
            )
        }
        Fields::Unnamed(fields) => {
            let mut serialize_statements = Vec::new();
            let mut deserialize_statements = Vec::new();
            let mut field_names = Vec::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_type = &field.ty;
                let field_name = syn::Ident::new(&format!("field_{}", i), name.span());
                let index = syn::Index::from(i);

                serialize_statements.push(quote! { bytes.extend(self.#index.serialize()); });
                deserialize_statements.push(quote! {
                    let #field_name: #field_type = Deserialize::deserialize(&bytes[offset..])?;
                    offset += #field_name.serialize().len();
                });
                field_names.push(field_name);
            }

            (
                quote! {
                    let mut bytes = Vec::new();
                    #(#serialize_statements)*
                    bytes
                },
                quote! {
                    let mut offset = 0;
                    #(#deserialize_statements)*
                    Some(Self(#(#field_names),*))
                },
            )
        }
        Fields::Unit => (
            quote! { Vec::new() },
            quote! { Some(Self) },
        ),
    }
}

fn handle_enum(
    name: &syn::Ident,
    data_enum: &DataEnum,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let mut serialize_variants = Vec::new();
    let mut deserialize_variants = Vec::new();

    for (index, variant) in data_enum.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        let index_literal = index as u8;

        match &variant.fields {
            Fields::Named(fields_named) => {
                let mut field_names = Vec::new();
                let mut field_serializations = Vec::new();
                let mut field_deserializations = Vec::new();

                for field in fields_named.named.iter() {
                    let field_name = field.ident.clone().expect("Named field must have an identifier");
                    let field_type = &field.ty;

                    field_names.push(field_name.clone());
                    field_serializations.push(quote! { bytes.extend(#field_name.serialize()); });
                    field_deserializations.push(quote! {
                        let #field_name: #field_type = Deserialize::deserialize(&bytes[offset..])?;
                        offset += #field_name.serialize().len();
                    });
                }

                serialize_variants.push(quote! {
                    Self::#variant_name { #(#field_names),* } => {
                        let mut bytes = vec![#index_literal];
                        #(#field_serializations)*
                        bytes
                    }
                });

                deserialize_variants.push(quote! {
                    #index_literal => {
                        let mut offset = 1;
                        #(#field_deserializations)*
                        Some(Self::#variant_name { #(#field_names),* })
                    }
                });
            }
            Fields::Unnamed(fields_unnamed) => {
                let mut field_names = Vec::new();
                let mut field_serializations = Vec::new();
                let mut field_deserializations = Vec::new();

                for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                    let field_name = syn::Ident::new(&format!("field_{}", i), name.span());
                    let field_type = &field.ty;

                    field_names.push(field_name.clone());
                    field_serializations.push(quote! { bytes.extend(#field_name.serialize()); });
                    field_deserializations.push(quote! {
                        let #field_name: #field_type = Deserialize::deserialize(&bytes[offset..])?;
                        offset += #field_name.serialize().len();
                    });
                }

                serialize_variants.push(quote! {
                    Self::#variant_name(#(#field_names),*) => {
                        let mut bytes = vec![#index_literal];
                        #(#field_serializations)*
                        bytes
                    }
                });

                deserialize_variants.push(quote! {
                    #index_literal => {
                        let mut offset = 1;
                        #(#field_deserializations)*
                        Some(Self::#variant_name(#(#field_names),*))
                    }
                });
            }
            Fields::Unit => {
                serialize_variants.push(quote! {
                    Self::#variant_name => vec![#index_literal]
                });

                deserialize_variants.push(quote! {
                    #index_literal => Some(Self::#variant_name)
                });
            }
        }
    }

    (
        quote! {
            match self {
                #(#serialize_variants),*
            }
        },
        quote! {
            match bytes.get(0).copied()? {
                #(#deserialize_variants),*
                _ => None,
            }
        },
    )
}
