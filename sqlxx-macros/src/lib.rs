extern crate proc_macro;

use async_trait::async_trait;
use proc_macro::{Ident, TokenStream};
use quote::quote;
use syn::parse::{Parse, ParseBuffer};
use syn::{parse_macro_input, Data, DeriveInput, Field, Fields, FieldsNamed, ItemStruct};

#[proc_macro_attribute]
pub fn baz(attr: TokenStream, item: TokenStream) -> TokenStream {
    println!("attr {}", attr);
    item
}

struct ModelStruct {
    pub struct_name: Ident,
    pub fields: Vec<String>,
}

impl Parse for ModelStruct {
    fn parse(input: &ParseBuffer) -> syn::Result<Self> {
        todo!()
    }
}

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let input_ts = input.clone();
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("MyMacro only works on structs"),
    };

    let field_names = fields
        .iter()
        .map(|field| field.ident.as_ref().unwrap().to_string());

    let tracker_name = syn::Ident::new(
        format!("{}ChangeTracker", name.to_string()).as_str(),
        proc_macro2::Span::call_site(),
    );

    let output = quote! {
        struct #tracker_name {
        }

        impl #name {
            fn fields() -> Vec<&'static str> {
                vec![#(#field_names),*]
            }
        }

        impl Save for #name {
            fn save(&mut self) {
                self.id = 3;
            }
        }
    };

    output.into()
}

#[proc_macro_attribute]
pub fn model(_attr: TokenStream, input: TokenStream) -> TokenStream {
    let item = parse_macro_input!(input as ItemStruct);

    let struct_name = &item.ident;

    let fields = &item.fields;

    let field_names = fields.iter().map(|field| field.ident.as_ref().unwrap());

    let field_types = fields.iter().map(|field| {
        // println!("{}", &field.ty);
        &field.ty
    });

    let field_names_str = field_names.clone().map(|f| f.to_string());

    let output = quote! {
        struct #struct_name {
            #(#field_names: #field_types),*
            dirty: #struct_name,
        }

        impl #struct_name {
            fn fields() -> Vec<&'static str> {
                vec![#(#field_names_str),*]
            }

        }

        impl Save for #struct_name {
            fn save(&mut self) {
                self.id = 3;
            }
        }
    };

    output.into()
}
