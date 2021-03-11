extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{self, punctuated::Punctuated, Data, Fields, GenericParam, Lifetime, LifetimeDef, Token};

#[proc_macro_derive(ViewGroup)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;

    let field_count = quote!(0);
    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

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

    let mut drawable_generics = ast.generics.clone();

    if !drawable_generics.params.empty_or_trailing() {
        drawable_generics
            .params
            .push_punct(Token![,](Span::call_site()));
    }
    drawable_generics
        .params
        .push_value(GenericParam::Lifetime(LifetimeDef {
            attrs: Vec::new(),
            lifetime: Lifetime::new("'drawable", Span::call_site()),
            colon_token: None,
            bounds: Punctuated::new(),
        }));

    let (drawable_impl_generics, _, drawable_where_clause) = drawable_generics.split_for_impl();

    let gen = quote! {
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
            fn top_left(&self) -> Point {
                embedded_layout::view_group::ViewGroupHelper::bounds(self).top_left
            }
            fn bottom_right(&self) -> Point {
                embedded_layout::view_group::ViewGroupHelper::bounds(self).bottom_right
            }
            fn size(&self) -> Size {
                embedded_layout::utils::rect_helper::RectSize::size(
                    embedded_layout::view_group::ViewGroupHelper::bounds(self)
                )
             }
        }

        impl #drawable_impl_generics embedded_graphics::drawable::Drawable<C> for &'drawable #name #ty_generics #drawable_where_clause {
            fn draw<D: embedded_graphics::DrawTarget<C>>(self, display: &mut D) -> Result<(), D::Error> {
                #(#draw)*

                Ok(())
            }
        }
    };
    gen.into()
}
