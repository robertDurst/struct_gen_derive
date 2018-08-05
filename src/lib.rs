#![warn(missing_docs)]
//! # struct_gen_derive
//! struct_gen_derive is a dependency of struct_gen that implements the `Zero` trait for fixed-length arrays over an enum of types
//! Out of context of struct_gen, it is pretty useless.

extern crate proc_macro;
extern crate syn;
#[macro_use]
extern crate quote;

use proc_macro::TokenStream;
use syn::Ty::Path;
use syn::{Body, VariantData};

/// struct_iterator is a procedural macro for taking a tuple struct of types
/// implementing`Zero` on each type for fixed-size arrays of lengths [0,10].
///
/// # struct_iterator
/// Comments and boilerplate from [dtolnay's presentation at Mozilla][dtolnay],
/// basic struct iteration based off of [Christopher Breeden's blog post][blog].
///
/// [dtolnay]: https://air.mozilla.org/rust-meetup-december-2016-12-15/
/// [blog]: https://cbreeden.github.io/Macros11/
/// 
/// *Note:* for the internal `impl_struct_iter` function we use default for now instead
/// of zoor. This will likely change by 0.2.0 release of struct_gen, however for now, where
/// we are only interested in the zero in zero-or-override, this is fine.
#[proc_macro_derive(StructIterator)]
pub fn struct_iterator(input: TokenStream) -> TokenStream {
    // Construct a string representation of the type definition
    let s = input.to_string();

    // Parse the string representation
    let ast = syn::parse_macro_input(&s).unwrap();

    // Build the impl
    let gen = match ast.body {
        Body::Enum(_) => panic!("Enum unsporrted."),
        Body::Struct(ref fields) => impl_struct_iter(fields),
    };

    // Return the generated impl
    gen.parse().unwrap()
}

/// impl_struct_iter is the meat of the struct_iterator method. It takes a fragment of rust code
/// specifically a tuple of types, and outputs code implementing `Zero` (from struct_gen) on these
/// types for arrays of fixed-length between [0,10].
fn impl_struct_iter(fields: &VariantData) -> quote::Tokens {
    // capture all the types to impl Zero on
    let mut idents = Vec::new();

    // This is some ugly nested matching.
    // TODO: find a cleaner way to do this.
    match fields {
        VariantData::Tuple(ref fields) => {
            for (_, field) in fields.iter().enumerate() {
                match &field.ty {
                    Path(_, ref f) => {
                        let ident = &f.segments[0].ident;
                        idents.push(ident);
                    }
                    _ => panic!("Unexpected tuple field."),
                }
            }
        }

        _ => panic!("Unsupported variant data."),
    }

    // the resolved code
    let mut res = Vec::new();

    // If greater than 10, use a std::vec::Vec.
    for i in 0..=10 {
        for x in idents.iter() {
            let size = i as usize;
            res.push(quote! {
                impl_zero!(
                    [#x; #size], [<#x>::default(); #size]
                );
            });
        }
    }

    // return the rust code fragment
    quote! {
        #(#res)*
    }
}
