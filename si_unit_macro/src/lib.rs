use proc_macro::TokenStream;

use crate::unit_impl::unit_impl;


mod unit_impl;

#[proc_macro]
pub fn unit(ts: TokenStream) -> TokenStream {
    unit_impl(ts.into()).into()
}