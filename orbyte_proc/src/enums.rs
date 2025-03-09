use broccli::xprintln;
use quote::quote;
use syn::{DataEnum, Fields};

pub fn handle_enum(
    name: &syn::Ident,
    data_enum: &DataEnum,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    #[cfg(feature = "debug")]
    xprintln!("-- Handling enum: ", name);

    let mut serialize_variants = Vec::new();
    let mut deserialize_variants = Vec::new();

    for (index, variant) in data_enum.variants.iter().enumerate() {
        let variant_name = &variant.ident;
        #[cfg(feature = "debug")]
        xprintln!("--- Processing variant: ", variant_name);
        let index_literal = index as u8;

        match &variant.fields {
            Fields::Named(fields_named) => {
                #[cfg(feature = "debug")]
                xprintln!("---- Variant has named fields");

                let mut field_names = Vec::new();
                let mut field_serializations = Vec::new();
                let mut field_deserializations = Vec::new();

                for field in fields_named.named.iter() {
                    let field_name = field
                        .ident
                        .clone()
                        .expect("Named field must have an identifier");

                    #[cfg(feature = "debug")]
                    xprintln!("----- Processing named field: ", field_name);

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
                #[cfg(feature = "debug")]
                xprintln!("---- Variant has unnamed fields");

                let mut field_names = Vec::new();
                let mut field_serializations = Vec::new();
                let mut field_deserializations = Vec::new();

                for (i, field) in fields_unnamed.unnamed.iter().enumerate() {
                    let field_name = syn::Ident::new(&format!("field_{}", i), name.span());
                    let field_type = &field.ty;

                    #[cfg(feature = "debug")]
                    xprintln!("----- Processing unnamed field: ", field_name);

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
                #[cfg(feature = "debug")]
                xprintln!("---- Variant is a unit variant");

                serialize_variants.push(quote! {
                    Self::#variant_name => {
                        vec![#index_literal]
                    }
                });

                deserialize_variants.push(quote! {
                    #index_literal => {
                        Some(Self::#variant_name)
                    }
                });
            }
        }
    }

    #[cfg(feature = "debug")]
    xprintln!("--- Finished processing enum: ", name);

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