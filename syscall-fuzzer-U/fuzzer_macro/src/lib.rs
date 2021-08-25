//! Procedural macros for basic data structures used in syscalls and syscall semantics
//! Support `Named` and `Unit` types.

extern crate proc_macro;
extern crate proc_macro2;

use proc_macro::TokenStream;
use quote::quote;
// use syscalls::*;
use syn;


/// Call `generate` method of each element and generate a new instance of the struct recursively
#[proc_macro_derive(Generate)]
pub fn generate_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_generate_macro(&ast)
}

// impl Generate for Nanosleep {
//     fn generate(gen: &mut StdRng) -> Nanosleep {
//         Nanosleep {
//             req: Timespec::generate(gen),
//             rem: Timespec::generate(gen),
//         }
//     }
// }

fn impl_generate_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let data = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("`Call` cannot be derived for type other than struct"),
    };

    let fields = match *data {
        syn::Fields::Named(ref fields) => &fields.named,
        syn::Fields::Unit => {
            let gen = quote! {
                impl Generate for #name {
                    fn generate(gen: &mut StdRng) -> #name {
                        #name
                    }
                }
            };
            return gen.into();
        }
        _ => panic!("Currently `Call` doesn't allow unnamed fields in struct"),
    };

    // Code starting here is very dirty and needs reconstruction in the future

    let mut buffers = vec![];
    let mut buffer_sizes: Vec<&syn::Field> = vec![];
    let mut others = vec![];

    for f in fields {
        match &f.ty {
            syn::Type::Path(ref p) => {
                let typename = p.path.segments.iter().next().unwrap().ident.to_string();
                if typename == "RetBuffer"
                    || typename == "ArgBuffer"
                    || typename == "AllocatedMemory"
                {
                    buffers.push(f);
                } else if typename == "BufferLength" {
                    buffer_sizes.push(f);
                } else {
                    others.push(f);
                }
            }
            _ => others.push(f),
        }
    }

    // let args = fields.into_iter().map(|field| {
    //     match &field.ident {
    //         Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
    //         _ => panic!("Some error occurs"),
    //     }
    // });

    // let types = fields.into_iter().map(|field| {
    //     &field.ty
    // });

    let r_buffers = &buffers;
    let r_others = &others;
    let r_buffer_sizes = &buffer_sizes;

    // let buf_args: Vec<proc_macro2::Ident> = r_buffers.into_iter().map(|field| {
    //     match &field.ident {
    //         Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
    //         _ => panic!("Some error occurs"),
    //     }
    // }).collect(); // to use it more than once
    let buf_args = r_buffers
        .into_iter()
        .map(|field| match &field.ident {
            Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
            _ => panic!("Some error occurs"),
        })
        .collect::<Vec<proc_macro2::Ident>>(); // to use it more than once
    let buf_types = r_buffers.into_iter().map(|field| &field.ty);

    let buf_size_args = r_buffer_sizes.into_iter().map(|field| match &field.ident {
        Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
        _ => panic!("Some error occurs"),
    });
    let buf_size_types = r_buffer_sizes.into_iter().map(|field| &field.ty);

    let other_args = r_others.into_iter().map(|field| match &field.ident {
        Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
        _ => panic!("Some error occurs"),
    });
    let other_types = r_others.into_iter().map(|field| &field.ty);

    // comma & semi-colon must be in the parentheses
    let gen = quote! {
        impl Generate for #name {
            fn generate(gen: &mut StdRng) -> #name {
                #(let #buf_args = #buf_types::generate(gen);)*
                #name {
                    #(#other_args: #other_types::generate(gen),)*
                    #(#buf_size_args: #buf_size_types(#buf_args.len()),)*
                    // #(#buf_args: #buf_types::generate(gen)),*
                    #(#buf_args: #buf_args);*
                }
            }
        }
    };
    gen.into()
}

/// Derive the `call` method which invokes the *raw* syscall
#[proc_macro_derive(Call)]
pub fn call_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_call_macro(&ast)
}

// impl Call for Nanosleep {
//     fn call(&self) -> Result<i64, i64> {
//         let ret = unsafe {syscall!(SYS_nanosleep, self.req.argumentize(), self.rem.argumentize()) };
//         ret
//     }
// }

fn impl_call_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let syscall_name = format!("SYS_{}", name.to_string().to_lowercase());
    let syscall_name = syn::Ident::new(&syscall_name, ast.ident.span());

    let data = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("`Call` cannot be derived for type other than struct"),
    };

    let fields = match *data {
        syn::Fields::Named(ref fields) => &fields.named,
        syn::Fields::Unit => {
            let gen = quote! {
                impl Call for #name {
                    fn call(&self) -> std::result::Result<i64, i64> {
                        let ret = unsafe {syscall!(#syscall_name)};
                        ret
                    }
                }
            };
            return gen.into();
        }
        _ => panic!("Currently `Call` doesn't allow unnamed fields in struct"),
    };

    let args = fields.into_iter().map(|field| match &field.ident {
        Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
        _ => panic!("Some error occurs"),
    });

    let gen = quote! {
        impl Call for #name {
            fn call(&self) -> std::result::Result<i64, i64> {
                let ret = unsafe {syscall!(#syscall_name, #(self.#args.argumentize()),*)};
                ret
            }
        }
    };
    gen.into()
}

/// Derive the `call_libc` method which invokes the syscall through a libc wrapper
#[proc_macro_derive(CallLibc)]
pub fn call_libc_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_call_libc_macro(&ast)
}

// impl CallLibc for Read {
//     fn call_libc(&self) -> Result<i64, i64> {
//         handle_result(unsafe {
//             libc::read(
//                 self.fd.0,
//                 self.buf.argumentize() as _,
//                 self.count.argumentize(),
//             )
//         })
//     }
// }

fn impl_call_libc_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let syscall_func = format!("{}", name.to_string().to_lowercase());
    let syscall_func = syn::Ident::new(&syscall_func, ast.ident.span());

    let data = match ast.data {
        syn::Data::Struct(ref s) => &s.fields,
        _ => panic!("`Call` cannot be derived for type other than struct"),
    };

    let fields = match *data {
        syn::Fields::Named(ref fields) => &fields.named,
        syn::Fields::Unit => {
            let gen = quote! {
                impl CallLibc for #name {
                    fn call_libc(&self) -> std::result::Result<i64, i64> {
                        handle_result(unsafe {libc::#syscall_func()} as _)
                    }
                }
            };
            return gen.into();
        }
        _ => panic!("Currently `Call` doesn't allow unnamed fields in struct"),
    };

    let args = fields.into_iter().map(|field| match &field.ident {
        Some(field) => syn::Ident::new(&field.to_string(), proc_macro2::Span::call_site()),
        _ => panic!("Some error occurs"),
    });

    let gen = quote! {
        impl CallLibc for #name {
            fn call_libc(&self) -> std::result::Result<i64, i64> {
                handle_result(unsafe {
                    libc::#syscall_func(
                        #(self.#args.argumentize() as _),*
                    )
                } as _)
            }
        }
    };
    gen.into()
}

/// Make a type can be used as an argument in raw syscall calling (fits into register)
/// The `#[derive(Argument)]` can only be used for struct that contains only one anonymous element
#[proc_macro_derive(Argument)]
pub fn argument_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();

    impl_argument_macro(&ast)
}

fn impl_argument_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let gen = quote! {
        impl Argument for #name {
            fn argumentize(&self) -> usize {
                self.0 as usize
            }
        }
    };
    gen.into()
}
