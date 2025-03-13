use proc_macro::TokenStream;
use proc_macro2::Literal;
use quote::quote;
use syn::{
    DeriveInput, Fields, Ident, Token,
    parse::{Parse, ParseStream},
    parse_macro_input,
};

#[proc_macro_derive(Extract, attributes(zhua))]
pub fn zhua(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);

    let name = input.ident;
    let data_struct = match input.data {
        syn::Data::Struct(data_struct) => data_struct,
        syn::Data::Enum(_) => {
            unimplemented!("enum is currently not implemented")
        }
        syn::Data::Union(_) => unimplemented!("union is not allowed"),
    };

    let expanded_lets = expand_lets(&data_struct.fields);
    let expanded_ok_self = expand_ok_self(&data_struct.fields);

    let expanded = quote! {

        impl zhua::Extract for #name {
            fn extract_from<'a, T>(able: T) -> Result<Self, zhua::Error>
            where
                T: zhua::extract::Extractable<'a>
            {
                #(#expanded_lets)*
                #expanded_ok_self
            }
        }

    };

    TokenStream::from(expanded)
}

struct Zhua {
    selector: Literal,
    _comma: Token![,],
    value: Ident,
}

impl Parse for Zhua {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            selector: input.parse()?,
            _comma: input.parse()?,
            value: input.parse()?,
        })
    }
}

fn expand_lets(fields: &Fields) -> impl Iterator<Item = proc_macro2::TokenStream> {
    fields.iter().map(|field| {
        let zhua: Zhua = field
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("zhua"))
            .next()
            .unwrap()
            .parse_args()
            .unwrap();
        let name = &field.ident;
        let ty = &field.ty;
        let sel = &zhua.selector;
        let val = &zhua.value;
        quote! {
            let #name = <#ty as zhua::FromSelect<_>>::from_selectable(able, #sel, #val)
                .map_err(|err| err.with_field(stringify!(#name)))?;
        }
    })
}

fn expand_ok_self(fields: &Fields) -> proc_macro2::TokenStream {
    let names = fields.iter().map(|f| &f.ident);
    quote! {
        Ok(Self {
            #(#names),*
        })
    }
}
