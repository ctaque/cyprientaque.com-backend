extern crate proc_macro;
use quote::quote;
use syn::{ self };

#[proc_macro_derive(HttpAll)]
pub fn http_all(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_http_all_macro(&ast)
}

fn impl_http_all_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait]
        impl HttpAll for #name {
            async fn http_all(query: web::Query<HttpAllOptionalQueryParams>) -> Result<HttpResponse, HttpResponse>{
                let params = query.into_inner();
                let result = #name::all(params).await;
                match result {
                    Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
                    Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HttpFind)]
pub fn http_find(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_http_find_macro(&ast)
}

fn impl_http_find_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait]
        impl HttpFind for #name {
            async fn http_find(info: web::Path<FindInfo>) -> Result<HttpResponse, HttpResponse> {
                let result = #name::find(info.id.into()).await;
                match result {
                    Ok(res) => Ok(HttpResponse::Ok().body(json!(res))),
                    Err(err) => Err(HttpResponse::NotFound().body("ENTITY_NOT_FOUND"))
                }
            }
        }
    };
    gen.into()
}

#[proc_macro_derive(HttpDelete)]
pub fn http_delete(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_http_delete_macro(&ast)
}

fn impl_http_delete_macro(ast: &syn::DeriveInput) -> proc_macro::TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        #[async_trait]
        impl HttpDelete for #name {
            async fn http_delete(info: web::Path<DeleteInfo>) -> Result<HttpResponse, HttpResponse> {
                let result: Result<#name, Error> = #name::find(info.id.into()).await;

                match result {
                    Ok(entity) => {
                        match entity.delete().await {
                            Ok(e) => Ok(HttpResponse::Ok().body(json!(e))),
                            Err(err) => Err(HttpResponse::InternalServerError().body(err.to_string()))
                        }
                    }
                    Err(err) => Err(HttpResponse::NotFound().body("ENTITY_NOT_FOUND"))
                }
            }
        }
    };
    gen.into()
}
