use proc_macro::TokenStream;
use syn::{DeriveInput, parse_macro_input};

mod memory_access;

#[proc_macro_derive(MemoryAccess, attributes(memory))]
pub fn derive_memory_access(input: TokenStream) -> TokenStream {
    memory_access::derive_memory_access(parse_macro_input!(input as DeriveInput))
}
