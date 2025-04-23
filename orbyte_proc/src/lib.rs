extern crate proc_macro;
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[cfg(feature = "debug")]
use broccli::xprintln;

mod structs;
mod enums;

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
            structs::handle_struct(name, data_struct)
        }
        Data::Enum(data_enum) => {
            #[cfg(feature = "debug")]
            xprintln!("- Type is an enum");
            enums::handle_enum(name, data_enum)
        }
        Data::Union(_data_union) => {
            #[cfg(feature = "debug")]
            xprintln!("- Type is a union (unimplemented)");
            unimplemented!()
        }
    };

    let expanded = quote! {
        impl orbyte::Serialize for #name {
            fn serialize(&self) -> Vec<u8> {
                #serialization
            }
        }

        impl orbyte::Deserialize for #name {
            fn deserialize(bytes: &[u8]) -> Result<Self, orbyte::OrbyteError> {
                #deserialization
            }
        }
    };

    #[cfg(feature = "debug")]
    xprintln!("Finished processing type: ", name, "\n");

    TokenStream::from(expanded)
}