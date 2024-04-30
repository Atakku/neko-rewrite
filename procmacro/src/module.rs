#![feature(proc_macro_quote)]
// Copyright 2023 Atakku <https://atakku.dev>
//
// This project is dual licensed under MIT and Apache.

use darling::{FromMeta, FromVariant};
use proc_macro::TokenStream;
use proc_macro2::Ident;
use std::vec;
use syn::{parse_macro_input, ItemEnum, Variant};

//#[proc_macro_attribute]
//pub fn your_attr(args: TokenStream, input: TokenStream) -> TokenStream {
//    let attr_args = match NestedMeta::parse_meta_list(args.into()) {
//        Ok(v) => v,
//        Err(e) => { return TokenStream::from(Error::from(e).write_errors()); }
//    };
//    let _input = syn::parse_macro_input!(input as ItemEnum);
//
//    let _args = match MacroArgs::from_list(&attr_args) {
//        Ok(v) => v,
//        Err(e) => { return TokenStream::from(e.write_errors()); }
//    };
//
//    // do things with `args`
//    unimplemented!()
//}

#[derive(Default, FromMeta, Debug)]
#[darling(default)]
struct Lorem {
  #[darling(rename = "sit")]
  ipsum: bool,
  dolor: Option<String>,
}

#[derive(FromMeta, Debug)]
//#[darling(attributes(my_crate), forward_attrs(allow, doc, cfg))]
struct Enum {
  ident: syn::Ident,
  //attrs: Vec<syn::Attribute>,
  //data: Data<EnumVariant, ()>,
  lorem: Option<Lorem>,
}

#[derive(Debug, FromVariant)]
#[darling(attributes(my_crate), forward_attrs(allow, doc, cfg))]
struct EnumVariant {
  // The name of the enum variant
  pub ident: syn::Ident,
  pub attrs: Vec<syn::Attribute>,
}

//#[macro_export]
//macro_rules! schema {
//  ($(
//    //$(#[$meta:meta])*
//    $($(#[$tp:ident($($tpp:expr),*)])+)?
//    $vis:vis enum $ident:ident {
//      $($(#[$param:ident($($tt:expr),*)])+ $field:ident),*$(,)?
//    }
//  )*) => {
//    $(
//      #[derive(sea_query::Iden)]
//      //$(#[$meta])*
//      #[allow(dead_code)]
//      $vis enum $ident {
//        Table, $($field),*
//      }
//
//      impl $crate::Table for $ident {
//        fn create() -> $crate::sea_query::TableCreateStatement {
//          $crate::sea_query::Table::create().table($ident::Table).if_not_exists()
//          $(.col(&mut $crate::sea_query::ColumnDef::new($ident::$field)$(.$param($($tt),*))*))*
//          $($(.$tp($($tpp),*))*)?
//          .to_owned()
//        }
//      }
//    )*
//
//    pub fn create_tables() -> Vec<sea_query::TableCreateStatement> {
//      use $crate::Table;
//      vec![$($ident::create()),*]
//    }
//  };
//}

#[proc_macro_derive(Poggers, attributes(my_crate))]
pub fn derive_iden(input: TokenStream) -> TokenStream {
  //let input = parse_macro_input!(input);
  //let gaw = Enum::from_derive_input(&input);

  //println!("{:?}", gaw.unwrap().data.take_enum().unwrap().get(0).unwrap());
  //Ok(quote! {
  //  struct Cock;
  //
  //}).unwrap_or_else(|e: darling::Error | e.write_errors().into())
  todo!()
}

use quote::quote;

#[proc_macro_attribute]
pub fn schema(_args: TokenStream, input: TokenStream) -> TokenStream {
  let mut input = parse_macro_input!(input as ItemEnum);
  input.variants.push(Variant {
    attrs: vec![],
    discriminant: None,
    fields: syn::Fields::Unit,
    ident: Ident::from_string("Table").unwrap(),
  });

  for var in &mut input.variants {
    let mut gwei: Vec<TokenStream> = vec![];
    var.attrs.retain(|i| {
      gwei.push(quote! {
        //.param()
      }.into());
      false
    });

    let gwaa = quote! {
      .col(&mut neko_sqlx::sea_query::ColumnDef::new()#(#gwei)*)
    };
  }

  //println!("{:?}", input);
  use darling::ToTokens;
  let ident = input.ident.clone();
  //input.into_token_stream().into();
  let output = input.into_token_stream();

  let quot = quote! {
    #output
    //impl neko_sqlx::Table for
    impl neko_sqlx::Table for #ident {
      fn create() -> neko_sqlx::sea_query::TableCreateStatement {
        neko_sqlx::sea_query::Table::create().table(Self::Table).if_not_exists()
      }
    }
  };

  quot.into()
}
