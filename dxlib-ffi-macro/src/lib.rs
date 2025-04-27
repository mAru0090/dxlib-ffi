// src/lib.rs

extern crate proc_macro;

use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    FnArg, Ident, LitStr, Pat, PatType, Signature, Token, TypePath, Type, TypeReference,
};

/// マクロ入力全体を受け取る構造体
struct DxlibGenInput {
    lib_name: LitStr,
    fns:     Punctuated<Signature, Token![,]>,
}

impl Parse for DxlibGenInput {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // 1) 文字列リテラルとしてライブラリ名をパース
        let lib_name: LitStr = input.parse()?;
        // 2) カンマをスキップ
        input.parse::<Token![,]>()?;
        // 3) 以降を Signature のカンマ区切りリストとしてパース
        let fns = Punctuated::<Signature, Token![,]>::parse_terminated(input)?;
        Ok(DxlibGenInput { lib_name, fns })
    }
}

#[proc_macro]
pub fn dxlib_gen(input: TokenStream) -> TokenStream {
    let DxlibGenInput { lib_name, fns } = parse_macro_input!(input as DxlibGenInput);

    // CString を使うための import
    let mut output = quote! {
        use std::ffi::CString;
    };

    for sig in fns.iter() {
        let wrapper_name = &sig.ident;
        let extern_name  = format_ident!("dx_{}", wrapper_name);
        let output_ty    = &sig.output;
        let generics     = &sig.generics;

        let mut wrapper_args = Vec::new();
        let mut extern_args  = Vec::new();
        let mut convert_stmts = Vec::new();
        let mut call_idents   = Vec::new();

        for arg in sig.inputs.iter() {
            if let FnArg::Typed(PatType { pat, ty, .. }) = arg {
                let ident = match &**pat {
                    Pat::Ident(pi) => &pi.ident,
                    _ => panic!("パターン付き引数は未対応です"),
                };

                // &str の場合は CString に変換
                if let Type::Reference(TypeReference { elem, .. }) = &**ty {
                    if let Type::Path(TypePath { path, .. }) = &**elem {
                        if path.is_ident("str") {
                            wrapper_args.push(quote! { #ident: &str });
                            extern_args .push(quote! { #ident: *const i8 });
                            convert_stmts.push(quote! {
                                let #ident = {
                                    let c = CString::new(#ident).unwrap();
                                    let ptr = c.as_ptr();
                                    std::mem::forget(c);
                                    ptr
                                };
                            });
                            call_idents.push(quote! { #ident });
                            continue;
                        }
                    }
                }

                // それ以外はそのまま
                wrapper_args.push(quote! { #ident: #ty });
                extern_args .push(quote! { #ident: #ty });
                call_idents.push(quote! { #ident });
            }
        }

        // ── ここが変更点 ──
        // 定数ではなくパースした lib_name を使う
        let extern_block = quote! {
            #[link(name = #lib_name)]
            unsafe extern "stdcall" {
                fn #extern_name(#(#extern_args),*) #output_ty;
            }
        };

        let wrapper_fn = quote! {
            pub fn #wrapper_name #generics( #(#wrapper_args),* ) #output_ty {
                // &str → *const i8 変換
                #(#convert_stmts)*

                // unsafe で外部関数を呼び出し
                unsafe { #extern_name(#(#call_idents),*) }
            }
        };

        output.extend(extern_block);
        output.extend(wrapper_fn);
    }

    TokenStream::from(output)
}

