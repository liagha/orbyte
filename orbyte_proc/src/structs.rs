use broccli::xprintln;
use quote::quote;
use syn::{DataStruct, Fields};

pub fn handle_struct(
    name: &syn::Ident,
    data_struct: &DataStruct,
) -> (proc_macro2::TokenStream, proc_macro2::TokenStream) {
    #[cfg(feature = "debug")]
    xprintln!("-- Handling struct: ", name);

    match &data_struct.fields {
        Fields::Named(fields) => {
            #[cfg(feature = "debug")]
            xprintln!("--- Struct has named fields");

            let mut serialize_statements = Vec::new();
            let mut deserialize_statements = Vec::new();
            let mut field_names = Vec::new();

            for field in &fields.named {
                let field_name = &field.ident;
                let field_type = &field.ty;

                #[cfg(feature = "debug")]
                if let Some(name) = field_name {
                    xprintln!("---- Processing named field: ", name);
                }

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
            #[cfg(feature = "debug")]
            xprintln!("--- Struct has unnamed fields");

            let mut serialize_statements = Vec::new();
            let mut deserialize_statements = Vec::new();
            let mut field_names = Vec::new();

            for (i, field) in fields.unnamed.iter().enumerate() {
                let field_type = &field.ty;
                let field_name = syn::Ident::new(&format!("field_{}", i), name.span());
                let index = syn::Index::from(i);

                #[cfg(feature = "debug")]
                xprintln!("---- Processing unnamed field: ", field_name);

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
        Fields::Unit => {
            #[cfg(feature = "debug")]
            xprintln!("--- Struct is a unit struct");

            (quote! { Vec::new() }, quote! { Some(Self) })
        }
    }
}