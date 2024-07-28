// For a more complete solution, look at https://github.com/umut-sahin/bevy-persistent

use darling::{ast::NestedMeta, FromMeta};
use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Data, DeriveInput, Meta};

const DATA_PATH: &str = ".data";

#[derive(FromMeta)]
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

    // TODO: Wasm support
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

            fn persist(&self) -> Result<()> {
                let data = toml::to_string(self).with_context(|| format!("Failed to serialize data for {}", #name))?;
                std::fs::write(#path, data).with_context(|| format!("Failed to save serialized data to {} for {}", #path, #name))?;
                debug!("{} updated, saved in {}", #name, #path);
                Ok(())
            }

            fn update(&mut self, f: impl Fn(&mut #ident)) -> Result<()> {
                f(self);
                self.persist()
            }

            fn reset(&mut self) -> Result<()> {
                *self = Self::default();
                self.persist()
            }
        }
    };
    output.into()
}

#[proc_macro_derive(AssetAttr, attributes(asset))]
pub fn derive_asset_attr(_item: pm::TokenStream) -> pm::TokenStream {
    pm::TokenStream::new()
}

#[proc_macro_attribute]
pub fn asset_key(args: pm::TokenStream, input: pm::TokenStream) -> pm::TokenStream {
    let input: TokenStream = input.into();
    let DeriveInput { ident, data, .. } = parse2(input.clone()).unwrap();

    let Data::Enum(data) = data else {
        panic!("An asset key must be an Enum");
    };

    let names: Vec<_> = data
        .variants
        .iter()
        .map(|v| {
            // TODO: Check that the value path is "asset" and allow multiple attrs
            let name = v.ident.clone();
            let attr = v
                .attrs
                .get(0)
                .expect("Each asset must provide a path attribute");
            let Meta::NameValue(value) = attr.meta.clone() else {
                panic!("The asset attribute must be in the form #[asset = \"path\"]");
            };
            let asset_path = value.value;
            quote!((#ident::#name, asset_server.load(#asset_path)))
        })
        .collect();

    let args: TokenStream = args.into();

    let output = quote! {
        #[derive(Copy, Clone, Eq, PartialEq, Hash, Reflect, macros::AssetAttr)]
        #input

        impl AssetKey for #ident {
            type Asset = #args;
        }

        impl FromWorld for AssetMap<#ident> {
            fn from_world(world: &mut World) -> Self {
                let asset_server = world.resource::<AssetServer>();
                [#(#names),*].into()
            }
        }
    };
    output.into()
}
