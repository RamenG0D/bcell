static mut COUNTER: usize = 0;

#[proc_macro]
pub fn iota(input: proc_macro::TokenStream) -> proc_macro::TokenStream {
    // input should be empty
    if !input.is_empty() {
        // compile error
        return syn::Error::new_spanned(
            syn::parse::<proc_macro2::TokenStream>(input).unwrap(),
            "iota!() takes no arguments",
        )
        .to_compile_error()
        .into();
    }

    let value = unsafe { COUNTER };

    unsafe {
        COUNTER += 1;
    }

    quote::quote! { #value }.into()
}
