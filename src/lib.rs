extern crate proc_macro;

use proc_macro::{TokenStream};
use quote::ToTokens;
use syn::fold::Fold;
use syn::{Expr, Pat, Ident};

#[proc_macro_attribute]
pub fn print_input(attr: TokenStream, item: TokenStream) -> TokenStream {
    dbg!(&attr);
    dbg!(&item);
    item
}

#[proc_macro_attribute]
pub fn tailrec(_attr: TokenStream, item: TokenStream) -> TokenStream {
    tailrec_parse(item)
}

fn tailrec_parse(item: TokenStream) -> TokenStream {
    let original_fn: syn::ItemFn = match syn::parse(item.clone()) {
        Ok(ast) => ast,
        Err(e) => panic!("tailrec macro may only be used on functions. Error: {}", e),
    };

    //let body_tokens = *original_fn.block;

    let args = original_fn.sig.inputs.iter().map(|f| match f {
        syn::FnArg::Typed(pt) => *pt.clone().pat,
        _ => panic!("tailrec is not intended to be used on methods with a reciever."),
    }).collect::<Vec<Pat>>();

    let mut tr: Tailrec = Tailrec {
        arg_num: args.len(),
        arg_names: args,
        func_name: original_fn.sig.ident.clone(),
    };// */
    let body_tokens = tr.fold_block(*original_fn.block);

    let mut bindings = quote::quote! {};

    for arg in original_fn.sig.inputs.iter() {
        let pat = match arg {
            syn::FnArg::Typed(pat) => pat,
            other => panic!("tailrec is not intended to be used on methods with a reciever."),
        };
        let (name, ty) = (&pat.pat, &pat.ty);
        let binding = quote::quote! {
            let mut #name = #name;
        };
        bindings.extend(binding);
    }

    let loop_tokens = quote::quote! {
        loop {
            let res = { #body_tokens };
            return res;
        }
    };

    let signature = original_fn.sig;
    let new_fn = quote::quote! {
        #signature {
            #bindings
            #loop_tokens
        }
    };
    new_fn.into()
}

struct Tailrec {
    arg_num: usize,
    arg_names: Vec<Pat>,
    func_name: Ident,
}

impl Tailrec {
}

impl Fold for Tailrec {
    fn fold_expr(&mut self, expr: Expr) -> Expr {
        use syn::ExprReturn;
        use syn::Lit;
        match &expr {
            Expr::Return(e) => if let Some(e) = &e.expr {
                let e: &Expr = &*e;
                match e {
                    Expr::Call(c) => {
                        let f = &*c.func;
                        match f {
                            Expr::Path(ep) => {
                                let seg = &ep.path.segments;
                                if seg.len() == 1 && seg.first().unwrap().ident == self.func_name {
                                    // YES WE FOUND THE ONE

                                    let args = &c.args;
                                    let i = (0..self.arg_num).map(syn::Index::from);
                                    let arg_names = &self.arg_names;

                                    let bind = quote::quote! {
                                        {
                                            let arguments__ = (#args);
                                            #(#arg_names = arguments__.#i;)*
                                            continue;
                                        }
                                    };

                                    bind.into()

                                    //unimplemented!()

                                } else {
                                    syn::fold::fold_expr(self, expr)
                                }
                            },
                            _ => syn::fold::fold_expr(self, expr),
                        }
                    },
                    _ => syn::fold::fold_expr(self, expr),
                }
            } else {
                syn::fold::fold_expr(self, expr)
            },
            _ => syn::fold::fold_expr(self, expr),
        }
    }
}
