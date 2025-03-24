use std::collections::HashMap;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse::{Parse, ParseStream}, parse_macro_input, punctuated::Punctuated, Expr, ItemFn, Lit, Result, Token};

struct InstructionAttributeArgs {
    opcode: u8,
    prefixed: bool,
    func_args: Vec<Expr>
}

impl Parse for InstructionAttributeArgs {
    #[inline]
    fn parse(input: ParseStream) -> Result<Self> {
        let mut named_args: HashMap<String, Expr> = HashMap::new();

        let all_args: Punctuated<Expr, Token![,]> = Punctuated::parse_terminated(input)?;
        let mut first: Option<Expr> = None;
        let mut rest: Vec<Expr> = vec![];

        for arg in all_args {
            match arg {
                Expr::Assign(assign) => {
                    if let Expr::Path(path) = &*assign.left {
                        let name = path.path.get_ident().map(ToString::to_string).unwrap();
                        named_args.insert(name, *assign.right);
                    }
                }
                _ => {
                    // First positional argument (opcode)
                    if first.is_none() {
                        first = Some(arg);
                    } else {
                        rest.push(arg);
                    }
                }
            }
        }

        // Parse opcode
        let opcode = match first {
            Some(Expr::Lit(expr_lit)) => {
                if let Lit::Int(lit_int) = &expr_lit.lit {
                    lit_int.base10_parse::<u8>().expect("Opcode must be a u8")
                } else {
                    panic!("First argument must be an integer opcode");
                }
            }
            _ => panic!("First argument must be an integer opcode"),
        };

        Ok(Self {
            opcode,
            prefixed: named_args.get("prefixed").map_or(false, |v| {
                if let Expr::Lit(expr_lit) = v {
                    if let Lit::Bool(lit_bool) = &expr_lit.lit {
                        return lit_bool.value
                    }
                }

                false
            }),
            func_args: rest
        })
    }
}

#[proc_macro_attribute]
pub fn instruction(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attributes = parse_macro_input!(attr as InstructionAttributeArgs);
    let func = parse_macro_input!(item as ItemFn);

    let func_name = &func.sig.ident;
    let func_ret = &func.sig.output;
    let opcode = attributes.opcode;
    let prefixed = attributes.prefixed;
    let wrapper_name = syn::Ident::new(&format!("{}_{:#04x}{}", func_name, opcode, if prefixed { "_prefixed" } else { "" }), func_name.span());
    let arguments = attributes.func_args;

    let expanded = quote! {
        #func

        fn #wrapper_name(cpu: &mut CPU) #func_ret {
            #func_name(cpu, #(#arguments),*)
        }

        inventory::submit! {
            InstructionEntry {
                opcode: #opcode,
                prefixed: #prefixed,
                handler: #wrapper_name,
            }
        }
    };

    TokenStream::from(expanded)
}
