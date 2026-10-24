use proc_macro2::TokenStream;
use quote::{ToTokens, format_ident, quote};
use si_unit_parser::UnitFixedPointDecimal;
use syn::{Ident, LitFloat, LitInt, parse::Parse};

pub(crate) fn unit_impl(ts: TokenStream) -> TokenStream {
    let v = match syn::parse2::<Dec>(ts) {
        Ok(v) => v,
        Err(e) => {
            return e.to_compile_error().to_token_stream();
        }
    };

    let integer_digits: Box<[Ident]> = v
        .dec
        .integer()
        .into_iter()
        .map(|c| format_ident!("{}", format!("{:?}", c)))
        .collect();
    let decimal_digits: Box<[Ident]> = v
        .dec
        .decimal()
        .into_iter()
        .map(|c| format_ident!("{}", format!("{:?}", c)))
        .collect();

    let unit_type_name = format_ident!("{}", v.dec.unit_type().to_type_string());

    quote! {
        slackers_si_units::#unit_type_name ::new(
            <crate::__BACKER_TYPE__ as slackers_si_units::Backer>::from_fixed_decimal(
                &[
                    #(slackers_si_units::Digit:: #integer_digits, )*
                ],
                &[
                    #(slackers_si_units::Digit:: #decimal_digits, )*
                ]
            )
        )
    }
}

#[derive(Debug)]
pub struct Dec {
    dec: UnitFixedPointDecimal,
}

impl Parse for Dec {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let ispan = input.span();

        let s = if input.peek(LitFloat) {
            input.parse::<LitFloat>()?.to_string()
        } else if input.peek(LitInt) {
            input.parse::<LitInt>()?.to_string()
        } else {
            return Err(syn::Error::new(
                ispan,
                "could not find float or int to parse",
            ));
        };

        let dec = UnitFixedPointDecimal::parse_require_unit(s)
            .ok_or(syn::Error::new(ispan, "expected unit"))?;
        Ok(Self { dec })
    }
}
