use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Ident, Visibility,
};

#[proc_macro_derive(TypedBuilder)]
pub fn derive_typed_builder(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let vis = input.vis;

    let (fn_build, builder) = match input.data {
        Data::Struct(ref data) => match data.fields {
            Fields::Named(ref fields) => {
                (build_fn(&name, fields), builder_impl(&name, vis, fields))
            }
            Fields::Unnamed(_) => todo!(),
            Fields::Unit => todo!(),
        },
        Data::Enum(_) => todo!(),
        Data::Union(_) => todo!(),
    };

    let expanded = quote! {
        impl #name {
            #fn_build
        }

        #builder
    };

    proc_macro::TokenStream::from(expanded)
}

fn build_fn(name: &Ident, fields: &FieldsNamed) -> TokenStream {
    if fields.named.is_empty() {
        quote! {
            pub fn build() -> Self { Self{} }
        }
    } else {
        let builder_name = builder_name(&name);
        quote! {
            pub fn build() -> #builder_name {
                #builder_name::__init()
            }
        }
    }
}

fn builder_name(name: &Ident) -> Ident {
    format_ident!("{}TypedBuilder", name)
}

fn builder_impl(name: &Ident, vis: Visibility, fields: &FieldsNamed) -> TokenStream {
    if fields.named.is_empty() {
        return quote! {};
    }

    let builder_name = builder_name(name);

    let defaults = fields.named.iter().map(|f| {
        let field = &f.ident;
        quote_spanned! {f.span()=> #field: Default::default()}
    });

    quote! {
        #[doc(hidden)]
        #vis struct #builder_name {}

        impl #builder_name {
            // Can be called by #name only.
            #[doc(hidden)]
            fn __init() -> Self { Self {} }

            #vis fn complete(self) -> #name {
                #name { #(#defaults),* }
            }
        }
    }
}
