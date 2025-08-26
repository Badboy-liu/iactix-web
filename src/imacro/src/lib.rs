use proc_macro::TokenStream;
use quote::quote;

use syn::{parse_macro_input, ItemMod, ItemFn, Attribute, ReturnType, Type};
// #[macro_use]
// #[macro_export]
// macro_rules! register_modules {
//     ($app:expr,[$($m:ident),*]) => {{
//         let mut app = $app;
//         $(
//         app = app.configure($m::config);
//         )*
//         app
//     }};
// }


#[proc_macro_derive(JSON)]
pub fn JSON(input: TokenStream) -> TokenStream {
    let ast:syn::DeriveInput = syn::parse(input).unwrap();
    let name = &ast.ident;

    let genc = quote! {

        impl Responder for #name {
            type Body = actix_web::body::BoxBody;

            fn respond_to(self, _: &actix_web::HttpRequest) -> HttpResponse<Self::Body> {
                HttpResponse::Ok().json(self) // 自动转 JSON
            }
        }
    };
    genc.into()
}






#[proc_macro_attribute]
pub fn body(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);

    let vis = &input.vis;
    let sig = &input.sig;
    let attrs = &input.attrs;
    let block = &input.block;
    let asyncness = &sig.asyncness;
    let ident = &sig.ident;

    // 判断返回类型
    let ret_ty = match &sig.output {
        ReturnType::Type(_, ty) => Some(ty.as_ref()),
        ReturnType::Default => None,
    };
    let inputs = &sig.inputs;
    
    // 根据返回类型生成不同包装
    let body_wrap = if let Some(ty) = ret_ty {
        match ty {
            Type::Path(path) => {
                let last = &path.path.segments.last().unwrap().ident;
                if last == "String" || last == "str" {
                    quote! {
                        let result = #block;
                        ::actix_web::HttpResponse::Ok().body(result)
                    }
                } else {
                    quote! {
                        let result = #block;
                        ::actix_web::HttpResponse::Ok().json(result)
                    }
                }
            }
            _ => quote! {
                let result = #block;
                ::actix_web::HttpResponse::Ok().json(result)
            },
        }
    } else {
        // 无返回值
        quote! {
            #block;
            ::actix_web::HttpResponse::Ok().finish()
        }
    };

    let expanded = quote! {
        #(#attrs)*
        #vis #asyncness fn #ident(#inputs) -> impl ::actix_web::Responder {
            #body_wrap
        }
    };

    TokenStream::from(expanded)
}



#[proc_macro_attribute]
pub fn auto_config(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemMod);

    let mod_name = &input.ident;
    let content = if let Some((_, items)) = &input.content {
        items
    } else {
        return syn::Error::new_spanned(&input, "expected inline module").to_compile_error().into();
    };

    // 找出带 #[get(...)] 或 #[post(...)] 的函数
    let mut services = Vec::new();
    for item in content {
        if let syn::Item::Fn(ItemFn { sig, attrs, .. }) = item {
            if has_actix_route(attrs) {
                let name = &sig.ident;
                services.push(quote! { .service(#name) });
            }
        }
    }

    let expanded = quote! {
        pub mod #mod_name {
            use super::*;

            #(#content)*

            pub fn config(cfg: &mut actix_web::web::ServiceConfig) {
                cfg #(#services)* ;
            }
        }
    };

    expanded.into()
}

fn has_actix_route(attrs: &[Attribute]) -> bool {
    attrs.iter().any(|a| {
        a.path().is_ident("get") || a.path().is_ident("post") ||
            a.path().is_ident("put") || a.path().is_ident("delete")
    })
}
