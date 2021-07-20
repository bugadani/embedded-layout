extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{self, Data, Fields, GenericParam, LitInt, TypeParamBound};

#[proc_macro_derive(ViewGroup)]
pub fn derive_viewgroup(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    let struct_data = if let Data::Struct(ref data) = ast.data {
        data
    } else {
        panic!("derive(ViewGroup) only supports structs with named fields");
    };

    let fields = if let Fields::Named(ref fields) = struct_data.fields {
        fields
    } else {
        panic!("derive(ViewGroup) only supports structs with named fields");
    };

    let field_names = fields
        .named
        .iter()
        .map(|f| f.ident.clone().unwrap())
        .collect::<Vec<_>>();

    let translate = field_names
        .iter()
        .map(|f| quote!(#f: self.#f.translate(by),))
        .collect::<Vec<_>>();

    let draw = field_names
        .iter()
        .map(|f| quote!(self.#f.draw(display)?;))
        .collect::<Vec<_>>();

    let index = field_names
        .iter()
        .enumerate()
        .map(|(i, f)| quote!(#i => &self.#f,))
        .collect::<Vec<_>>();

    let index_mut = field_names
        .iter()
        .enumerate()
        .map(|(i, f)| quote!(#i => &mut self.#f,))
        .collect::<Vec<_>>();

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let name = &ast.ident;
    let field_count = format!("{}", field_names.len());
    let field_count = LitInt::new(&field_count, Span::call_site());

    let gen_view_group = quote! {
        impl #impl_generics embedded_layout::view_group::ViewGroup for #name #ty_generics #where_clause {
            fn len(&self) -> usize {
                #field_count
            }

            fn at(&self, index: usize) -> &dyn View {
                match index {
                    #(#index)*
                    _ => panic!()
                }
            }

            fn at_mut(&mut self, index: usize) -> &mut dyn View {
                match index {
                    #(#index_mut)*
                    _ => panic!()
                }
            }
        }

        impl #impl_generics embedded_graphics::transform::Transform for #name #ty_generics #where_clause {
            fn translate(&self, by: Point) -> Self {
                Self {
                    #(#translate)*
                }
            }

            fn translate_mut(&mut self, by: Point) -> &mut Self {
                embedded_layout::view_group::ViewGroupHelper::translate(self, by);
                self
            }
        }

        impl #impl_generics embedded_graphics::geometry::Dimensions for #name #ty_generics #where_clause {
            fn bounding_box(&self) -> embedded_graphics::primitives::Rectangle {
                embedded_layout::view_group::ViewGroupHelper::bounds(self)
            }
        }
    };

    let pixelcolor = ast.generics.params.iter().find_map(|p| {
        if let GenericParam::Type(tp) = p {
            if tp.bounds.iter().any(is_type_param("PixelColor")) {
                Some(tp.ident.clone())
            } else {
                None
            }
        } else {
            None
        }
    });

    let gen_drawable_impl = if let Some(pixelcolor) = pixelcolor {
        quote! {
            impl #impl_generics embedded_graphics::Drawable for #name #ty_generics #where_clause {
                type Color = #pixelcolor;
                type Output = ();

                fn draw<D: embedded_graphics::draw_target::DrawTarget<Color = #pixelcolor>>(&self, display: &mut D) -> Result<(), D::Error> {
                    #(#draw)*

                    Ok(())
                }
            }
        }
    } else {
        quote!()
    };

    let generated = quote! {
        #gen_view_group
        #gen_drawable_impl
    };
    generated.into()
}

fn is_type_param(ident: &'static str) -> impl Fn(&TypeParamBound) -> bool {
    move |f: &TypeParamBound| -> bool {
        if let TypeParamBound::Trait(tpb) = f {
            tpb.path
                .segments
                .iter()
                .last()
                .map(|segment| segment.ident == ident)
                .unwrap_or(false)
        } else {
            false
        }
    }
}
