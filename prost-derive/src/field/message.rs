use anyhow::{bail, Error};
use proc_macro2::TokenStream;
use quote::{quote, ToTokens};
use syn::Meta;

use crate::field::{
    field_ref, set_bool, set_option, tag_attr, word_attr, wrapper_attr, Label, Wrapper,
};

#[derive(Clone)]
pub struct Field {
    pub label: Label,
    pub tag: u32,
    pub wrapper: Option<Wrapper>,
}

impl Field {
    pub fn new(attrs: &[Meta], inferred_tag: Option<u32>) -> Result<Option<Field>, Error> {
        let mut message = false;
        let mut label = None;
        let mut tag = None;
        let mut boxed = false;
        let mut wrapper = None;

        let mut unknown_attrs = Vec::new();

        for attr in attrs {
            if word_attr("message", attr) {
                set_bool(&mut message, "duplicate message attribute")?;
            } else if word_attr("boxed", attr) {
                set_bool(&mut boxed, "duplicate boxed attribute")?;
            } else if let Some(w) = wrapper_attr(attr) {
                set_option(&mut wrapper, w, "duplicate wrapper attribute")?;
            } else if let Some(t) = tag_attr(attr)? {
                set_option(&mut tag, t, "duplicate tag attributes")?;
            } else if let Some(l) = Label::from_attr(attr) {
                set_option(&mut label, l, "duplicate label attributes")?;
            } else {
                unknown_attrs.push(attr);
            }
        }

        if wrapper.is_none() && boxed {
            wrapper = Some(Wrapper::Box);
        }

        if !message {
            return Ok(None);
        }

        if !unknown_attrs.is_empty() {
            bail!(
                "unknown attribute(s) for message field: #[prost({})]",
                quote!(#(#unknown_attrs),*)
            );
        }

        let tag = match tag.or(inferred_tag) {
            Some(tag) => tag,
            None => bail!("message field is missing a tag attribute"),
        };

        Ok(Some(Field {
            label: label.unwrap_or(Label::Optional),
            tag,
            wrapper,
        }))
    }

    pub fn new_oneof(attrs: &[Meta]) -> Result<Option<Field>, Error> {
        if let Some(mut field) = Field::new(attrs, None)? {
            if let Some(attr) = attrs.iter().find(|attr| Label::from_attr(attr).is_some()) {
                bail!(
                    "invalid attribute for oneof field: {}",
                    attr.path().into_token_stream()
                );
            }
            field.label = Label::Required;
            Ok(Some(field))
        } else {
            Ok(None)
        }
    }

    pub fn encode(&self, ident: TokenStream) -> TokenStream {
        let tag = self.tag;
        let field = field_ref(&self.wrapper, ident);
        match self.label {
            Label::Optional => quote! {
                if let Some(msg) = #field {
                    ::prost::encoding::message::encode(#tag, msg, buf);
                }
            },
            Label::Required => quote! {
                ::prost::encoding::message::encode(#tag, #field, buf);
            },
            Label::Repeated => quote! {
                for msg in #field {
                    ::prost::encoding::message::encode(#tag, msg, buf);
                }
            },
        }
    }

    pub fn merge(&self, ident: TokenStream) -> TokenStream {
        match self.label {
            Label::Optional => quote! {
                ::prost::encoding::message::merge(wire_type,
                                                 #ident.get_or_insert_with(::core::default::Default::default),
                                                 buf,
                                                 ctx)
            },
            Label::Required => quote! {
                ::prost::encoding::message::merge(wire_type, #ident, buf, ctx)
            },
            Label::Repeated => quote! {
                ::prost::encoding::message::merge_repeated(wire_type, #ident, buf, ctx)
            },
        }
    }

    pub fn encoded_len(&self, ident: TokenStream) -> TokenStream {
        let tag = self.tag;
        let field = field_ref(&self.wrapper, ident);
        match self.label {
            Label::Optional => quote! {
                (#field).as_ref().map_or(0, |msg| ::prost::encoding::message::encoded_len(#tag, msg))
            },
            Label::Required => quote! {
                ::prost::encoding::message::encoded_len(#tag, #field)
            },
            Label::Repeated => quote! {
                ::prost::encoding::message::encoded_len_repeated(#tag, #field)
            },
        }
    }

    pub fn clear(&self, ident: TokenStream) -> TokenStream {
        match self.label {
            Label::Optional => quote!(#ident.take()),
            Label::Required => quote!(#ident.clear()),
            Label::Repeated => quote!(#ident.clear()),
        }
    }
}
