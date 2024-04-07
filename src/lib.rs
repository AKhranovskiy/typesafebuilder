#![allow(incomplete_features)]
#![feature(generic_const_exprs)]

use proc_macro2::TokenStream;
use quote::{format_ident, quote, quote_spanned};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Field, Fields, FieldsNamed, Ident,
    Type, Visibility,
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
            Fields::Unnamed(_) => unimplemented!("Structs with unnamed fields are not supported"),
            Fields::Unit => (
                quote! {
                    pub fn build() -> Self { Self{} }
                },
                quote!(),
            ),
        },
        Data::Enum(_) => unimplemented!("Enums are not supported"),
        Data::Union(_) => unimplemented!("Unions are not supported"),
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
        let builder_name = builder_name(name);
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

trait FieldExt {
    fn flag(&self) -> Ident;
}
impl FieldExt for Field {
    fn flag(&self) -> Ident {
        format_ident!(
            "{}",
            self.ident.as_ref().unwrap().to_string().to_uppercase()
        )
    }
}

fn builder_impl(name: &Ident, vis: Visibility, fields: &FieldsNamed) -> TokenStream {
    if fields.named.is_empty() {
        return quote! {};
    }

    if fields.named.iter().any(|f| is_option(&f.ty)) {
        unimplemented!("Optional fields are not supported");
    }

    let builder_name = builder_name(name);

    let builder_decl = {
        let decl_generics = fields.named.iter().map(|f| {
            let ident = f.flag();
            quote! { const #ident: bool = false }
        });

        quote! { #builder_name<#(#decl_generics),*> }
    };

    let (inner_name, inner_decl, inner_init) = inner(&builder_name, fields);

    let setters = match fields.named.len() {
        0 => {
            unreachable!("No builder for empty structs")
        }
        1 => {
            let field = fields.named.first().unwrap();
            let ident = &field.ident;
            let ty = &field.ty;

            quote! {
                impl #builder_name {
                    #vis fn #ident(self, value: #ty) -> #name {
                        #name {
                            #ident: value,
                        }
                    }
                }
            }
        }
        _ => {
            let setters = fields.named.iter().map(|current_field| {
                // Flags declarations except the current field.
                // `impl<const B: bool, const C: bool>`
                let flag_names = {
                    let flag_names = fields
                        .named
                        .iter()
                        .filter(|field| field.ident != current_field.ident)
                        .map(|field| {
                            let ident = field.flag();
                            quote! { const #ident: bool }
                        });

                    quote! { #(#flag_names),* }
                };

                // impl specialization
                // `Foo<false, B, C>`
                let input_flags = {
                    let input_flags = fields.named.iter().map(|field| {
                        if field.ident == current_field.ident {
                            quote! { false }
                        } else {
                            let ident = field.flag();
                            quote! { #ident }
                        }
                    });

                    quote! { #(#input_flags),* }
                };

                // Output builder flags
                // `FooBuilder<true, B, C>`
                let output_flags = {
                    let output_flags = fields.named.iter().map(|field| {
                        if field.ident == current_field.ident {
                            quote! { true }
                        } else {
                            let ident = field.flag();
                            quote! { #ident }
                        }
                    });

                    quote! { #(#output_flags),* }
                };

                // Flag condition for impl specialization
                // `B & C`
                let flag_condition = {
                    let flags = fields
                        .named
                        .iter()
                        .filter(|field| field.ident != current_field.ident)
                        .map(|field| {
                            let ident = field.flag();
                            quote! { #ident }
                        });

                    quote! { #(#flags)&* }
                };

                // All flags set to true except the current.
                // Final specialization.
                let complete_flags = {
                    let complete_flags = fields.named.iter().map(|field| {
                        if field.ident == current_field.ident {
                            quote! { false }
                        } else {
                            quote! { true }
                        }
                    });

                    quote! { #(#complete_flags),* }
                };

                let boolean_name = format_ident!("{name}Boolean");
                let false_trait_name = format_ident!("{name}False");

                let ident = current_field.ident.as_ref();
                let ty = &current_field.ty;

                let move_fields = {
                    let move_fields = fields
                        .named
                        .iter()
                        .filter(|field| field.ident != current_field.ident)
                        .map(|f| {
                            let ident = f.ident.as_ref().unwrap();
                            quote! { #ident: self.inner.#ident.unwrap() }
                        });

                    quote! { #(#move_fields),* }
                };

                quote! {
                    // Conditional implementation where some flags are not set.
                    impl<#flag_names> #builder_name<#input_flags>
                    where #boolean_name<{#flag_condition}>: #false_trait_name
                    {
                        pub fn #ident(self, value: #ty) -> #builder_name<#output_flags> {
                            #builder_name::<#output_flags> {
                                inner: #inner_name {
                                    #ident: Some(value),
                                    ..self.inner
                                }
                            }
                        }
                    }
                    // Final implementation where all flags except the current are set.
                    impl #builder_name<#complete_flags>{
                        pub fn #ident(self, value: #ty) -> #name {
                            #name {
                                #ident: value,
                                #move_fields
                            }
                        }
                    }
                }
            });

            quote! {
                #(#setters)*
            }
        }
    };

    let builder = match fields.named.len() {
        0 => unreachable!(),
        1 => {
            quote! {
                #[doc(hidden)]
                #[derive(Debug)]
                struct #builder_name;
            }
        }
        _ => {
            quote! {
                #[doc(hidden)]
                #[derive(Debug)]
                struct #builder_decl {
                    inner: #inner_name
                }
            }
        }
    };

    let boolean = boolean(vis, name);

    quote! {
        #inner_decl

        #builder

        impl #builder_name {
            // Can be called by #name only.
            #[doc(hidden)]
            fn __init() -> Self {
                Self {
                    #inner_init
                }
            }
        }

        #boolean

        #setters
    }
}

fn inner(builder_name: &Ident, fields: &FieldsNamed) -> (Ident, TokenStream, TokenStream) {
    let inner_name = format_ident!("_{builder_name}Inner");

    let inner_fields = fields.named.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote_spanned! {f.span()=> #ident: Option<#ty> }
    });

    match fields.named.len() {
        0 => unreachable!(),
        1 => (inner_name, quote! {}, quote! {}),
        _ => (
            inner_name.clone(),
            quote! {
                #[doc(hidden)]
                #[derive(Debug, Clone, Default)]
                struct #inner_name {
                    #(#inner_fields),*
                }
            },
            quote! { inner: Default::default() },
        ),
    }
}

fn boolean(vis: Visibility, name: &Ident) -> TokenStream {
    let boolean_name = format_ident!("{name}Boolean");
    let true_name = format_ident!("{name}True");
    let false_name = format_ident!("{name}False");
    quote! {
        #[doc(hidden)]
        #vis struct #boolean_name<const B: bool>;
        #[doc(hidden)]
        #vis trait #true_name {}
        #[doc(hidden)]
        #vis trait #false_name {}
        impl #true_name for #boolean_name<true> {}
        impl #false_name for #boolean_name<false> {}
    }
}

fn is_option(ty: &Type) -> bool {
    if let Type::Path(ref tp) = ty {
        tp.qself.is_none()
            && tp
                .path
                .segments
                .last()
                .map_or(false, |s| s.ident == "Option")
    } else {
        false
    }
}
