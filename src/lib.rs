use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DeriveInput, Error, Expr, ExprLit, Ident, Lit,
    LitInt, Meta, NestedMeta, Variant,
};

#[proc_macro_derive(Message, attributes(message))]
pub fn derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let output = process(input)
        .unwrap_or_else(|err| err.to_compile_error())
        .into();
    // eprintln!("{output}");
    let _ = output;
    output
}

fn process(input: DeriveInput) -> Result<TokenStream2, Error> {
    let inherent_impl = get_inherent_impl(&input)?;

    Ok(quote! {
        #inherent_impl
    })
}

fn get_discriminant(variant: &Variant, repr: &Ident) -> Result<TokenStream2, Error> {
    if let Some((_, discriminant)) = &variant.discriminant {
        if let Expr::Lit(ExprLit {
            lit: Lit::Int(lit), ..
        }) = &discriminant
        {
            Ok(if lit.suffix().is_empty() {
                let lit = LitInt::new(format!("{}{}", &lit, repr).as_str(), lit.span());
                quote!(#lit)
            } else {
                quote!(#lit)
            })
        } else {
            Ok(quote!((#discriminant)))
        }
    } else {
        Err(Error::new(
            variant.ident.span(),
            "enum variant must have a discriminant",
        ))
    }
}

fn get_repr(input: &DeriveInput) -> Result<Ident, Error> {
    match &input.data {
        Data::Enum(_) => {
            if let Some(repr) = input.attrs.iter().find(|&a| a.path.is_ident("repr")) {
                let meta = repr.parse_meta()?;
                if let Meta::List(list) = meta {
                    if list.nested.len() != 1 {
                        Err(Error::new(list.nested.span(), "Expected a single inttype"))
                    } else if let NestedMeta::Meta(Meta::Path(path)) = &list.nested[0] {
                        Ok(path.get_ident().unwrap().clone())
                    } else {
                        Err(Error::new(list.span(), "Expected `#[repr(...)]`"))
                    }
                } else {
                    Err(Error::new(meta.span(), "Expected `#[repr(...)]`"))
                }
            } else {
                Err(Error::new(
                    input.ident.span(),
                    "`#[repr(inttype)]` is required",
                ))
            }
        }
        _ => Err(Error::new(input.span(), "Expected an enum")),
    }
}

fn get_inherent_impl(input: &DeriveInput) -> Result<TokenStream2, Error> {
    let name = &input.ident;
    if let Data::Enum(data) = &input.data {
        let repr = &get_repr(input)?;
        let inner_name = format_ident!("{}Simple", name);

        let mut variant_names = vec![];
        let mut discriminants = vec![];

        for variant in data.variants.iter() {
            variant_names.push(&variant.ident);
            discriminants
                .push(get_discriminant(variant, repr).unwrap_or_else(|e| e.to_compile_error()));
        }

        return Ok(quote! {
            #[automatically_derived]
            impl #name {
                fn compile_time_check_for_message_tag_collisions() {
                    #[repr(#repr)]
                    enum #inner_name {
                        #(#variant_names = #discriminants,)*
                    }
                }
            }
        });
    }

    Err(Error::new(input.span(), "Expected an enum"))
}
