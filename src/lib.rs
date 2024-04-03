use std::collections::HashMap;

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Ident, Type,
    Visibility,
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
            pub fn build() -> #builder_name<true> {
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

    if fields.named.iter().any(|f| is_option(&f.ty)) {
        unimplemented!("Optional fields not implemented");
    }

    let flags = fields
        .named
        .iter()
        .map(|f| f.ident.as_ref())
        .filter(Option::is_some)
        .map(Option::unwrap)
        .map(|field| field.to_string().to_uppercase())
        .map(|field| (field, false))
        .collect::<HashMap<String, bool>>();

    dbg!(&flags);

    let decl_generics = flags.iter().map(|(k,v)| {
        let ident = format_ident!("{k}");
        quote! { const #ident: bool = #v }
    });

    let all_false = flags.iter().map(|_| quote! { false });
    let all_true = flags.iter().map(|_| quote! { true });
    let all_true2 = flags.iter().map(|_| quote! { true });

    let defaults = fields.named.iter().map(|f| {
        let field = &f.ident;
        quote_spanned! {f.span()=> #field: Default::default()}
    });

    let false_flags = quote! { #(#all_false),* };
    let true_flags = quote! { #(#all_true),* };

    quote! {
        #[doc(hidden)]
        #vis struct #builder_name<#(#decl_generics),*> {}

        impl #builder_name<#false_flags> {
            // Can be called by #name only.
            #[doc(hidden)]
            fn __init() -> #builder_name<#true_flags> { #builder_name::<#true_flags> {} }
        }

        impl #builder_name<#true_flags> {
            #vis fn complete(self) -> #name {
                #name { #(#defaults),* }
            }
        }
    }
}

fn is_option(ty: &Type) -> bool {
    if let Type::Path(ref tp) = ty {
        tp.qself.is_none()
            && tp
                .path
                .segments
                .last()
                .map_or(false, |s| s.ident.to_string() == "Option")
    } else {
        false
    }
}
