mod enums;
mod structs;

extern crate proc_macro;
use proc_macro::TokenStream;
use broccli::xprintln;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};
use crate::enums::handle_enum;
use crate::structs::handle_struct;

#[proc_macro_derive(Orbyte)]
pub fn derive_orbyte(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    #[cfg(feature = "debug")]
    xprintln!("Processing type: ", name);

    let (serialization, deserialization) = match &input.data {
        Data::Struct(data_struct) => {
            #[cfg(feature = "debug")]
            xprintln!("- Type is a struct");
            handle_struct(name, data_struct)
        }
        Data::Enum(data_enum) => {
            #[cfg(feature = "debug")]
            xprintln!("- Type is an enum");
            handle_enum(name, data_enum)
        }
        Data::Union(_data_union) => {
            #[cfg(feature = "debug")]
            xprintln!("- Type is a union (unimplemented)");
            unimplemented!()
        }
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

    #[cfg(feature = "debug")]
    xprintln!("Finished processing type: ", name, "\n");

    TokenStream::from(expanded)
}