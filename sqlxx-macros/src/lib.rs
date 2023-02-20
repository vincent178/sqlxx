extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("MyMacro only works on structs"),
    };

    // let column_names = fields
    //     .iter()
    //     .map(|f| f.ident.as_ref().unwrap().to_string())
    //     .filter(|f| f != "id")
    //     .collect::<Vec<_>>();

    let update_sql = String::from("UPDATE users ");
    let mut set_columns: Vec<String> = vec![];

    let dynamic_bind = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .filter(|f| f.to_string() != "id")
        .enumerate()
        .map(|(i, f)| {
            let column = f.to_owned().to_string();

            if i == 0 {
                set_columns.push(format!("SET {} = ${}", column, i + 2));
            } else {
                set_columns.push(format!("{} = ${}", column, i + 2));
            }

            quote! { .bind(self.#f.to_owned()) }
        })
        .collect::<Vec<_>>();

    let update_sql = update_sql + set_columns.join(", ").as_str() + " WHERE id = $1 RETURNING *";

    let output = quote! {
        impl #name {
            // fn fields() -> Vec<&'static str> {
            //     vec![#(#column_names),*]
            // }

            async fn save(&mut self, db: &sqlx::PgPool) {
                let instance: #name = sqlx::query_as(#update_sql).bind(self.id)#(#dynamic_bind)*.fetch_one(db).await.unwrap();

                self.id = instance.id;
            }
        }
    };

    output.into()
}
