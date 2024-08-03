//! Procedural macro helpers for some game systems.

#![warn(missing_docs)]

use proc_macro as pm;
use proc_macro2::TokenStream;
use quote::quote;
use syn::{parse2, Data, DeriveInput, Meta};

/// Helper macro to allow deriving `asset` attributes for struct fields.
/// There may be better ways to do this.
#[proc_macro_derive(AssetAttr, attributes(asset))]
pub fn derive_asset_attr(_item: pm::TokenStream) -> pm::TokenStream {
    pm::TokenStream::new()
}

/// Defines an `AssetKey`, a collection of assets of the same type that are
/// loaded together.
///
/// # Examples
///
/// ```ignore
/// use game::prelude::*;
///
/// pub fn plugin(app: &mut App) {
///     app.load_asset::<SomeAssetKey>();
/// }
///
/// #[asset_key(Image)]
/// pub enum SomeAssetKey {
///     #[asset = "some/asset.png"]
///     SomeVariant,
/// }
/// ```
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
            let name = v.ident.clone();
            let asset_path = v
                .attrs
                .iter()
                .filter_map(|attr| {
                    let Meta::NameValue(value) = attr.meta.clone() else { return None };
                    if value.path.get_ident()? != "asset" {
                        return None;
                    };
                    Some(value.value)
                })
                .next()
                .expect("Each asset must provide a path attribute");
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
