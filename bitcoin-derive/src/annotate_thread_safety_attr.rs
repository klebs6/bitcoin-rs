// ---------------- [ File: bitcoin-derive/src/annotate_thread_safety_attr.rs ]
use proc_macro::TokenStream;

/**
  | Use Clang's thread safety analysis annotations
  | when available. In other environments, the
  | macros receive empty definitions.  Usage
  | documentation:
  | https://clang.llvm.org/docs/ThreadSafetyAnalysis.html
  */
#[allow(non_snake_case)]
pub(crate) fn annotate_thread_safety_attr(
    macro_name: &'static str,
    semantics: &'static str,
    attr: proc_macro::TokenStream,
    item: proc_macro::TokenStream,
) -> proc_macro::TokenStream {
    {
        use tracing::{debug, trace};
        debug!(target: "bitcoinleveldb.thread_annotations", %macro_name, "applying attribute");
        trace!(target: "bitcoinleveldb.thread_annotations", macro_args = %attr.to_string());
    }

    {
        use proc_macro2::TokenStream as TokenStream2;
        use quote::{quote, ToTokens};
        use syn::{parse::Parser, Meta, Token};

        let attr2: TokenStream2 = attr.into();
        let item2: TokenStream2 = item.into();

        let parser = syn::punctuated::Punctuated::<Meta, Token![,]>::parse_terminated;
        let parsed = parser.parse2(attr2.clone()).unwrap_or_default();
        let args_rendered = if parsed.is_empty() {
            String::new()
        } else {
            parsed
                .iter()
                .map(|m| m.to_token_stream().to_string())
                .collect::<Vec<_>>()
                .join(", ")
        };

        let doc = if args_rendered.is_empty() {
            format!(
                "C++ Thread-Safety Annotation: {macro_name}\n\
                 - Semantics: {semantics}\n\
                 - Arguments: []\n\
                 - Source: leveldb/port/thread_annotations.h\n\
                 - Behavior in Rust: no-op attribute (records metadata for tracing & docs)."
            )
        } else {
            format!(
                "C++ Thread-Safety Annotation: {macro_name}\n\
                 - Semantics: {semantics}\n\
                 - Arguments: [{args_rendered}]\n\
                 - Source: leveldb/port/thread_annotations.h\n\
                 - Behavior in Rust: no-op attribute (records metadata for tracing & docs)."
            )
        };

        {
            use tracing::{info, trace};
            if args_rendered.is_empty() {
                info!(target: "bitcoinleveldb.thread_annotations", name = %macro_name, args = "[]", "attribute recorded");
            } else {
                info!(target: "bitcoinleveldb.thread_annotations", name = %macro_name, args = %args_rendered, "attribute recorded");
            }
            trace!(target: "bitcoinleveldb.thread_annotations", item_tokens = %item2.to_string());
        }

        let out = quote! {
            #[doc = #doc]
            #item2
        };
        out.into()
    }
}
