extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, Data, DeriveInput};

/*
 * (a, b, c, d) and ($1, $2, $3, $4) for insert
 * SET a = $2, b = $3, c = $4 for update
 */

#[proc_macro_derive(Model)]
pub fn derive_model(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let name = &ast.ident;
    let fields = match ast.data {
        Data::Struct(ref data) => &data.fields,
        _ => panic!("Model derive macro only works on structs"),
    };

    // to build column list like (a, b, c, d)
    let mut insert_columns: Vec<String> = vec![];
    // to build bind value list ($1, $2, $3, $4)
    let mut insert_values: Vec<String> = vec![];
    // to build set column list a = $2, b = $3, c = $4
    let mut set_columns: Vec<String> = vec![];

    let fields_idents = fields.iter().map(|f| f.ident.as_ref().unwrap());

    let colums_idents = fields_idents.filter(|f| f.to_string() != "id");

    let dynamic_bind = colums_idents
        .enumerate()
        .map(|(i, f)| {
            let column = f.to_owned().to_string();

            // $1 reserved for 'id', see update_sql below
            set_columns.push(format!("{} = ${}", column, i + 2));

            insert_columns.push(column);
            insert_values.push(format!("${}", i + 1));

            quote! { .bind(self.#f.to_owned()) }
        })
        .collect::<Vec<_>>();

    let update_sql = "UPDATE users".to_string()
        + format!(" SET {}", set_columns.join(", ")).as_str()
        + " WHERE id = $1"
        + " RETURNING *";

    let insert_sql = "INSERT INTO users".to_string()
        + format!(" ( {} )", insert_columns.join(", ")).as_str()
        + " VALUES"
        + format!(" ( {} )", insert_values.join(", ")).as_str()
        + " RETURNING *";

    let output = quote! {
        impl #name {
            async fn save(&mut self, db: &sqlx::PgPool) {
                if self.id == 0 {
                    let instance: #name = sqlx::query_as(#insert_sql)#(#dynamic_bind)*.fetch_one(db).await.unwrap();
                    self.id = instance.id;
                } else {
                    let instance: #name = sqlx::query_as(#update_sql).bind(self.id)#(#dynamic_bind)*.fetch_one(db).await.unwrap();
                }
            }
        }
    };

    output.into()
}
