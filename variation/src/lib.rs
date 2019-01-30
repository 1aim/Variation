//! # Variation
//! A procedural macro to generate enum to variant conversion methods.
//!
//! ## Methods generated
//!
//! #### `is_*` methods
//! An `is_variant` method is generated for each for variant in an enum.
//!
//! ```rust
//! use variation::Variation;
//!
//! #[derive(Variation)]
//! enum Type {
//!     Unit,
//!     Integer(i32),
//! }
//!
//! fn main() {
//!     let return_type = Type::Unit;
//!
//!     assert!(return_type.is_unit());
//!     assert!(!return_type.is_integer());
//! }
//! ```
//!
//! #### `as_*` & `as_*_mut` methods
//! Variants that have one or more inner types have `as` and `as_mut` allowing you
//! to get a immutable or mutable reference to the inner types. Variants with a
//! single inner type will return `&{mut} T`. Variants that have more than one inner
//! type will return a tuple with a reference to each type.
//!
//! ```rust
//! use variation::Variation;
//!
//! #[derive(Variation)]
//! enum Type {
//!     Unit,
//!     Integer(i32),
//!     Real(i32, u32),
//! }
//!
//! fn main() {
//!     let mut return_type = Type::Integer(5);
//!     let real_value = Type::Real(3, 14);
//!
//!     assert_eq!(Some(&mut 5), return_type.as_integer_mut());
//!     assert_eq!(Some((&3, &14)), real_value.as_real());
//!     assert_eq!(None, real_value.as_integer());
//! }
//! ```
//!
//! #### `into_*` methods
//! Variants that have one or more inner types have an `into` method, allowing you
//! to attempt to convert a enum into its inner values. This method will panic when
//! called on a variant that does not match the method.
//!
//! ```rust
//! use variation::Variation;
//!
//! #[derive(Variation)]
//! enum Type {
//!     Unit,
//!     Integer(i32),
//!     Real(i32, u32),
//! }
//!
//! fn main() {
//!     let mut return_type = Type::Integer(5);
//!     let real_value = Type::Real(3, 14);
//!     let unit = Type::Unit;
//!
//!     assert_eq!(5, return_type.into_integer());
//!     assert_eq!((3, 14), real_value.into_real());
//!     // Panics
//!     unit.into_integer();
//!
//! }
//! ```

extern crate proc_macro;

use heck::SnakeCase;
use proc_macro2::{Ident, Span, TokenStream};
use quote::{quote, ToTokens};
use syn::*;

#[proc_macro_derive(Variation)]
pub fn variation_derive(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_variation(&ast)
}

fn impl_variation(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let mut implementation = TokenStream::new();

    let data = match ast.data {
        Data::Enum(ref s) => s,
        _ => {
            // name.span()
            //     .unstable()
            //     .error("`#[derive(Variation)]` is only available for structs")
            //     .emit();

            // return TokenStream::new()
            panic!("`#[derive(Variation)]` is only available for enums")
        }
    };

    for variant in &data.variants {
        let variant_name = &variant.ident;
        let snake_case = variant_name.to_string().to_snake_case();
        let is_fn = Ident::new(&format!("is_{}", snake_case), Span::call_site());
        let as_fn = Ident::new(&format!("as_{}", snake_case), Span::call_site());
        let as_mut_fn = Ident::new(&format!("as_{}_mut", snake_case), Span::call_site());
        let into_fn = Ident::new(&format!("into_{}", snake_case), Span::call_site());
        let field_count = variant.fields.iter().count();
        let ignoring_fields = if field_count > 0 {
            let fields = vec![(); field_count].into_iter().fold(TokenStream::new(), |mut acc, _| {
                acc.extend(quote!(_,));
                acc
            });

            quote![(#fields)]
        } else {
            quote!()
        };

        let value_fields = generate_ident_list_pattern(field_count, false, false);
        let ref_fields = generate_ident_list_pattern(field_count, true, false);
        let ref_mut_fields = generate_ident_list_pattern(field_count, true, true);

        let return_by_value = match field_count {
            0 => quote!(),
            1 => variant.fields.iter().next().unwrap().into_token_stream(),
            _ => {
                let type_list = variant.fields.iter().fold(TokenStream::new(), |mut acc, f| {
                    let ty = &f.ty;
                    acc.extend(quote!(#ty,));
                    acc
                });

                quote![(#type_list)]
            }
        };

        let return_by_ref = match field_count {
            0 => quote!(),
            1 => {
                let ty = variant.fields.iter().next().unwrap();
                quote!(&#ty)
            },
            _ => {
                let type_list = variant.fields.iter().fold(TokenStream::new(), |mut acc, f| {
                    let ty = &f.ty;
                    acc.extend(quote!(&#ty,));
                    acc
                });

                quote![(#type_list)]
            }
        };

        let return_by_ref_mut = match field_count {
            0 => quote!(),
            1 => {
                let ty = variant.fields.iter().next().unwrap();
                quote!(&mut #ty)
            },
            _ => {
                let type_list = variant.fields.iter().fold(TokenStream::new(), |mut acc, f| {
                    let ty = &f.ty;
                    acc.extend(quote!(&mut #ty,));
                    acc
                });

                quote![(#type_list)]
            }
        };

        let return_value = match field_count {
            0 => quote!(),
            1 => Ident::new("v0", Span::call_site()).into_token_stream(),
            _ => value_fields.clone()
        };

        implementation.extend(quote! {
            pub fn #is_fn(&self) -> bool {
                match self {
                    #name::#variant_name#ignoring_fields => true,
                    _ => false,
                }
            }
        });

        if field_count > 0 {
            implementation.extend(quote! {
                pub fn #as_fn(&self) -> Option<#return_by_ref> {
                    match self {
                        #name::#variant_name#ref_fields => Some(#return_value),
                        _ => None,
                    }
                }

                pub fn #as_mut_fn(&mut self) -> Option<#return_by_ref_mut> {
                    match self {
                        #name::#variant_name#ref_mut_fields => Some(#return_value),
                        _ => None,
                    }
                }

                /// Consumes the enum and returns the inner type.
                /// # Panics
                /// When this method is called on the wrong enum variant.
                pub fn #into_fn(self) -> #return_by_value {
                    match self {
                        #name::#variant_name#value_fields => #return_value,
                        _ => panic!("")
                    }
                }
            })
        }
    }

    let gen = quote! {
        impl #name {
            #implementation
        }
    };

    gen.into()
}

fn generate_ident_list_pattern(count: usize, refed: bool, mutable: bool) -> TokenStream {
    if count > 0 {
        let fields = (0..).take(count).fold(TokenStream::new(), |mut acc, i| {
            let mut pattern = TokenStream::new();
            let ident = Ident::new(&format!("v{}", i), Span::call_site());

            if refed {
                pattern.extend(quote!(ref));
            }

            if mutable {
                pattern.extend(quote!(mut));
            }

            acc.extend(quote!(#pattern #ident,));
            acc
        });

        quote![(#fields)]
    } else {
        quote!()
    }
}
