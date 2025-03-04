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

#[allow(unused_variables)]
fn handle_struct(name: &syn::Ident, data_struct: &DataStruct) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let fields = match &data_struct.fields {
        Fields::Named(fields_named) => {
            let field_serializations = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                quote! { bytes.extend(self.#field_name.serialize()); }
            });
            let field_deserialization = fields_named.named.iter().map(|field| {
                let field_name = &field.ident;
                let field_type = &field.ty;
                quote! {
                    let #field_name: #field_type = Deserialize::deserialize(&bytes[offset..])?;
                    offset += #field_name.serialize().len();
                }
            });
            let field_names = fields_named.named.iter().map(|field| &field.ident);

            (
                quote! {
                    let mut bytes = Vec::new();
                    #(#field_serializations)*
                    bytes
                },
                quote! {
                    let mut offset = 0;
                    #(#field_deserialization)*
                    Some(Self { #(#field_names),* })
                },
            )
        }
        _ => unimplemented!(),
    };
    fields
}

fn handle_enum(name: &syn::Ident, data_enum: &DataEnum) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    let variant_serializations = data_enum.variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields_unnamed) => {
                let field_names = (0..fields_unnamed.unnamed.len()).map(|i| syn::Ident::new(&format!("field_{}", i), name.span()));
                let field_serializations = field_names.clone().map(|f| quote! { bytes.extend(#f.serialize()); });
                quote! {
                    Self::#variant_name(#(#field_names),*) => {
                        let mut bytes = vec![#index as u8 as u8];

                        #(#field_serializations)*
                        bytes
                    }
                }
            }
            Fields::Unit => quote! {
                Self::#variant_name => vec![#index as u8]
            },
            _ => unimplemented!(),
        }
    });

    let variant_deserialization = data_enum.variants.iter().enumerate().map(|(index, variant)| {
        let variant_name = &variant.ident;
        match &variant.fields {
            Fields::Unnamed(fields_unnamed) => {
                let field_names = (0..fields_unnamed.unnamed.len()).map(|i| syn::Ident::new(&format!("field_{}", i), name.span()));
                let field_deserialization = fields_unnamed.unnamed.iter().enumerate().map(|(i, field)| {
                    let field_name = syn::Ident::new(&format!("field_{}", i), name.span());
                    let field_type = &field.ty;
                    quote! {
                        let #field_name: #field_type = Deserialize::deserialize(&bytes[offset..])?;
                        offset += #field_name.serialize().len();
                    }
                });
                quote! {
                    #index => {
                        let mut offset = 1;
                        #(#field_deserialization)*
                        Some(Self::#variant_name(#(#field_names),*))
                    }
                }
            }
            Fields::Unit => quote! {
                #index => Some(Self::#variant_name)
            },
            _ => unimplemented!(),
        }
    });

    (
        quote! {
            match self {
                #(#variant_serializations),*
            }
        },
        quote! {
            match bytes.get(0).copied()? as usize {
                #(#variant_deserialization),*
                _ => None,
            }
        },
    )
}
