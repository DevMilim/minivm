use proc_macro::TokenStream;
use quote::quote;
use syn::Data;

#[proc_macro_derive(FromRepr)]
pub fn parse_enum(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::DeriveInput);

    let name = &input.ident;

    let variants = if let Data::Enum(data_enum) = &input.data {
        &data_enum.variants
    } else {
        return syn::Error::new_spanned(name, "So pode ser usado em enum!")
            .to_compile_error()
            .into();
    };

    let arms = variants.iter().enumerate().map(|(i, v)| {
        let v_name = &v.ident;
        let index = i as u8;
        quote! {
            #index => Ok(#name::#v_name),
        }
    });

    let expanded = quote! {
        impl std::convert::TryFrom<u8> for #name{
            type Error = ();

            fn try_from(value: u8) -> Result<Self, Self::Error>{
                match value {
                    #(#arms)*
                    _ => Err(())
                }
            }
        }
    };
    expanded.into()
}
