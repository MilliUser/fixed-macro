use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, spanned::Spanned, Expr, ExprArray, ExprLit, Ident, ItemConst, Lit};

#[proc_macro_attribute]
pub fn to_fixed(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemConst);

    let const_name = input.ident.clone();
    let const_ty = input.ty.clone();

    // Sabit nokta türünü kontrol etme
    let (int_bits, frac_bits) = match &*input.ty {
        syn::Type::Array(syn::TypeArray { ref elem, .. }) => {
            // Dizi türü, array elemanının tipini alıyoruz
            let elem_type = elem.clone();

            // Array eleman türü üzerinden işlem yapıyoruz
            match &*elem_type {
                syn::Type::Path(type_path) => {
                    let segments = &type_path.path.segments;
                    if let Some(segment) = segments.last() {
                        let type_name = &segment.ident.to_string();

                        // `U{int_bits}F{frac_bits}` formatını ayırma
                        if let Some(captures) = type_name.split_once('F') {
                            // 'U' kısmından integer bit sayısını çıkarma
                            let int_part_str = &captures.0[1..]; // 'U' kısmını atla
                            let int_part: usize = int_part_str.parse().unwrap();

                            // 'F' kısmından fraction bit sayısını çıkarma
                            let frac_part_str = &captures.1;
                            let frac_part: usize = frac_part_str.parse().unwrap();

                            (int_part, frac_part)
                        } else {
                            panic!("Expected a fixed-point type like U2F14, U16F16, etc.");
                        }
                    } else {
                        panic!("Unexpected type format, no segments found.");
                    }
                }
                _ => panic!("Expected a fixed-point type like U2F14, U16F16, etc."),
            }
        }
        syn::Type::Path(type_path) => {
            // Eğer doğrudan bir türse
            let segments = &type_path.path.segments;
            if let Some(segment) = segments.last() {
                let type_name = &segment.ident.to_string();

                // `U{int_bits}F{frac_bits}` formatını ayırma
                if let Some(captures) = type_name.split_once('F') {
                    // 'U' kısmından integer bit sayısını çıkarma
                    let int_part_str = &captures.0[1..]; // 'U' kısmını atla
                    let int_part: usize = int_part_str.parse().unwrap();

                    // 'F' kısmından fraction bit sayısını çıkarma
                    let frac_part_str = &captures.1;
                    let frac_part: usize = frac_part_str.parse().unwrap();

                    (int_part, frac_part)
                } else {
                    panic!("Expected a fixed-point type like U2F14, U16F16, etc.");
                }
            } else {
                panic!("Unexpected type format, no segments found.");
            }
        }
        _ => {
            panic!("Expected a fixed-point type like U2F14, U16F16, etc.");
        }
    };

    // Türü doğru şekilde genişletiyoruz
    let fixed_type = Ident::new(&format!("U{}F{}", int_bits, frac_bits), const_ty.span());

    // Array içindeki her öğe için sabit nokta dönüşümü
    if let syn::Expr::Array(ExprArray { ref elems, .. }) = *input.expr {
        let fixed_values = elems.iter().map(|expr| {
            match expr {
                // Eğer expr bir literal floatsa
                Expr::Lit(ExprLit {
                    lit: Lit::Float(float),
                    ..
                }) => {
                    let value: f32 = float.base10_parse().unwrap();

                    // Sabit nokta dönüşümünü hesaplıyoruz
                    let scaled_value = value * (1 << frac_bits) as f32;
                    let rounded_value = scaled_value.round(); 

                    // Sabit nokta dönüşümünü yapıyoruz
                    let bits = rounded_value as u16;
                    quote! {
                        fixed::types::#fixed_type::from_bits(#bits)
                    }
                },
                _ => panic!("Expected a floating-point literal."),
            }
        });

        let expanded = quote! {
            const #const_name: #const_ty = [
                #(#fixed_values),*
            ];
        };

        TokenStream::from(expanded)
    } else {
        panic!("Expected an array expression.");
    }
}
