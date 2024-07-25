// For a more complete solution, look at https://github.com/umut-sahin/bevy-persistent

use darling::{ast::NestedMeta, FromMeta};
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, DeriveInput};

const DATA_PATH: &str = ".data";

#[derive(Debug, FromMeta)]
struct PersistentArgs {
    name: String,
}

#[proc_macro_attribute]
pub fn persistent(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let input: TokenStream = input.into();
    let DeriveInput { ident, .. } = parse2(input.clone()).unwrap();

    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
        Ok(v) => v,
        Err(e) => {
            return pm::TokenStream::from(darling::Error::from(e).write_errors());
        },
    };
    let args = match PersistentArgs::from_list(&attr_args) {
        Ok(v) => v,
        Err(e) => {
            return pm::TokenStream::from(e.write_errors());
        },
    };
    let path = format!("{}/{}.toml", DATA_PATH, args.name);
    let name = ident.to_string();

    let output = quote! {
        #[derive(Resource, Serialize, Deserialize)]
        #input

        impl Persistent for #ident {
            fn load() -> Self {
                let mut data = Self::default();
                data.reload();
                data
            }

            fn reload(&mut self) {
                *self = match std::fs::read_to_string(#path) {
                    Ok(data) => toml::from_str(&data).unwrap_or_default(),
                    Err(_) => Self::default(),
                };
            }

            fn persist(&self) {
                let data = toml::to_string(self).unwrap();
                let _ = std::fs::write(#path, data);
                debug!("{} updated, saved in {}", #name, #path);
            }

            fn update(&mut self, f: impl Fn(&mut #ident)) {
                f(self);
                self.persist()
            }

            fn reset(&mut self) {
                *self = Self::default();
                self.persist()
            }
        }
    };
    output.into()
}
