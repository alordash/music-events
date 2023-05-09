#[macro_use]
extern crate quote;
#[macro_use]
extern crate syn;

extern crate proc_macro;

use proc_macro2::TokenStream;
use quote::ToTokens;
use syn::{spanned::Spanned, Data, DeriveInput, Fields, FieldsNamed, Index, Meta};

#[proc_macro_derive(Repository, attributes(table_name, model, entity))]
pub fn repository_macro(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let table_name = get_attribute(&input, "table_name");
    let fns = impl_repository_functions(&input, &table_name);

    let model = get_attribute(&input, "model");
    let entity = get_attribute(&input, "entity");

    let name = input.ident.clone();

    let struct_name: TokenStream = format!("{}sRepository", name.to_string()).parse().unwrap();

    let expanded = quote! {
        use std::sync::Arc;
        pub struct #struct_name {
            pool: Arc<PgPool>
        }
        impl #struct_name {
            pub fn new(pool: Arc<PgPool>) -> Self {
                #struct_name { pool }
            }
        }
        impl Repository for #struct_name {
            type Model = #model;
            type Entity = #entity;

            #fns
        }
    };

    println!("{}", expanded);

    proc_macro::TokenStream::from(expanded)
}

fn get_attribute(ast: &DeriveInput, attribute_name: &str) -> TokenStream {
    let attr_str = if let Some(attr) = ast.attrs.iter().find(|attr| {
        if let Meta::NameValue(ref nv) = attr.meta {
            nv.path.get_ident().unwrap().to_string() == attribute_name
        } else {
            false
        }
    }) {
        attr.meta
            .require_name_value()
            .unwrap()
            .value
            .to_token_stream()
            .to_string()
    } else {
        panic!("Attribute \"{}\" not found", attribute_name)
    };
    attr_str.trim_matches('"').parse().unwrap()
}

fn get_fields(ast: &DeriveInput) -> FieldsNamed {
    if let Data::Struct(ref obj) = &ast.data {
        if let Fields::Named(ref nf) = &obj.fields {
            nf
        } else {
            panic!("Only applyable to structs with named fields")
        }
    } else {
        panic!("Only applyable to structs")
    }
    .clone()
}

fn get_fields_group(fields: &FieldsNamed) -> TokenStream {
    let recurse_group_except_first_two = fields.named.iter().skip(2).map(|f| {
        let name = &f.ident;
        quote_spanned! {f.span()=>
            #name
        }
    });
    let second_field = fields.named[1].ident.clone();
    let group = quote! {
        ( #second_field #(,#recurse_group_except_first_two)*)
    };
    group
}

fn get_placeholders(fields: &FieldsNamed) -> TokenStream {
    let recurse_placeholders_except_first_and_last = fields
        .named
        .iter()
        .enumerate()
        .skip(1)
        .take(fields.named.len() - 2)
        .map(|(i, f)| {
            let index = Index::from(i + 1);
            quote_spanned! {f.span()=>
                $#index
            }
        });
    let placeholders = quote! {
        ( $1 #(,#recurse_placeholders_except_first_and_last)*)
    };
    placeholders
}

fn get_accessors(fields: &FieldsNamed) -> TokenStream {
    let recurse_accessors_except_first_two = fields.named.iter().skip(2).map(|f| {
        let name = &f.ident;
        quote_spanned! {f.span()=>
            value.#name()
        }
    });
    let second_field = fields.named[1].ident.clone();
    let accessors = quote! {
        value.#second_field() #(,#recurse_accessors_except_first_two)*
    };
    accessors
}

fn get_field_setters(fields: &FieldsNamed) -> TokenStream {
    let recurse_setters_except_first_two = fields.named.iter().enumerate().skip(2).map(|(i, f)| {
        let name = &f.ident;
        let index = Index::from(i + 1);
        quote_spanned! {f.span()=>
            #name = $#index
        }
    });
    let second_field = fields.named[1].ident.clone();
    let setters = quote! {
        #second_field = $2 #(,#recurse_setters_except_first_two)* WHERE id = $1
    };
    setters
}

fn impl_repository_functions(ast: &DeriveInput, table_name: &TokenStream) -> TokenStream {
    let fields = get_fields(ast);

    let group = get_fields_group(&fields);
    let placeholders = get_placeholders(&fields);
    let accessors = get_accessors(&fields);
    let field_setters = get_field_setters(&fields);

    let add_query = quote! {
        INSERT INTO #table_name #group
        VALUES #placeholders
        RETURNING id
    }
    .to_string();

    let get_all_query = quote! {
        SELECT *
        FROM #table_name
        ORDER BY id
    }
    .to_string();

    let get_by_id_query = quote! {
        SELECT *
        FROM #table_name
        WHERE id = $1
    }
    .to_string();

    let get_count_query = quote! {SELECT COUNT(id) FROM #table_name}.to_string();

    let get_ids_query = quote! {
        SELECT id
        FROM #table_name
        ORDER BY id
    }
    .to_string();

    let get_paginated = quote! {
        SELECT *
        FROM #table_name
        ORDER BY id
        LIMIT $1
        OFFSET $2
    }
    .to_string();

    let remove_query = quote! {
        DELETE FROM #table_name
        WHERE id = $1
    }
    .to_string();

    let update_query = quote! {
        UPDATE #table_name
        SET
        #field_setters
    }
    .to_string();

    let implementation = quote! {
        async fn add(&self, value: &Self::Model) -> Result<u64, Error> {
            let rec = sqlx::query!(#add_query, #accessors)
            .fetch_one(self.pool.as_ref())
            .await?;

            Ok(rec.id as u64)
        }

        async fn get_all(&self) -> Result<Vec<Self::Model>, Error> {
            let entities = sqlx::query_as!(Self::Entity, #get_all_query)
            .fetch_all(self.pool.as_ref())
            .await?;

            Ok(entities.into_iter().map(Self::Entity::into).collect())
        }

        async fn get_by_id(&self, id: u64) -> Result<Option<Self::Model>, Error> {
            let entity = sqlx::query_as!(Self::Entity, #get_by_id_query, id as i64)
            .fetch_optional(self.pool.as_ref())
            .await?;

            Ok(entity.map(Self::Entity::into))
        }

        async fn get_count(&self) -> Result<u64, Error> {
            let (count,): (i64,) = sqlx::query_as(#get_count_query)
                .fetch_one(self.pool.as_ref())
                .await?;

            Ok(count as u64)
        }

        async fn get_ids(&self) -> Result<Vec<i64>, Error> {
            let ids = sqlx::query!(#get_ids_query)
            .fetch_all(self.pool.as_ref())
            .await?;
            Ok(ids.into_iter().map(|r| r.id).collect())
        }

        async fn get_paginated(&self, count: i64, offset: i64) -> Result<Vec<Self::Model>, Error> {
            let entities = sqlx::query_as!(Self::Entity, #get_paginated, count, offset)
            .fetch_all(self.pool.as_ref())
            .await?;

            Ok(entities.into_iter().map(Self::Entity::into).collect())
        }

        async fn remove(&self, id: u64) -> Result<u64, Error> {
            let rows_affected = sqlx::query!(#remove_query, id as i64)
            .execute(self.pool.as_ref())
            .await?
            .rows_affected();

            Ok(rows_affected)
        }

        async fn update(&self, value: &Self::Model) -> Result<(), Error> {
            if value.id().is_none() {
                return Err(Error::RowNotFound);
            }
            sqlx::query!(#update_query, value.id().unwrap(), #accessors)
            .fetch_one(self.pool.as_ref())
            .await?;
            Ok(())
        }
    };

    implementation
}
