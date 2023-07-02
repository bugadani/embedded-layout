extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{
    self, parse_macro_input, Data, DeriveInput, Fields, FieldsNamed, FieldsUnnamed, GenericParam,
    LitInt, TypeParamBound,
};

#[proc_macro_derive(ViewGroup)]
pub fn derive_viewgroup(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);

    let empty_vg_instance = quote!(unsafe { &embedded_layout::view_group::EMPTY_VIEW_GROUP });
    let empty_vg_instance_mut =
        quote!(unsafe { &mut embedded_layout::view_group::EMPTY_VIEW_GROUP });

    let (field_count_impl, index_impl, index_mut_impl, translate_impl, draw_impl) = match &ast.data
    {
        Data::Struct(struct_data) if matches!(&struct_data.fields, Fields::Named(_)) => {
            let fields = if let Fields::Named(fields) = &struct_data.fields {
                fields
            } else {
                panic!("Programming error: matches! should have prevent from taking this arm");
            };

            let field_names = fields
                .named
                .iter()
                .map(|f| f.ident.clone().unwrap())
                .collect::<Vec<_>>();

            let field_count = format!("{}", field_names.len());
            let field_count = LitInt::new(&field_count, Span::call_site());

            let translate = field_names
                .iter()
                .map(|f| quote!(#f: self.#f.clone().translate(by),))
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

            let field_count_impl = quote! {
                #field_count
            };

            let index_impl = quote! {
                match index {
                    #(#index)*
                    _ => #empty_vg_instance
                }
            };

            let index_mut_impl = quote! {
                match index {
                    #(#index_mut)*
                    _ => #empty_vg_instance_mut
                }
            };

            let translate_impl = quote! {
                Self {
                    #(#translate)*
                }
            };

            let draw_impl = quote! {
                #(#draw)*
            };

            (
                field_count_impl,
                index_impl,
                index_mut_impl,
                translate_impl,
                draw_impl,
            )
        }
        Data::Enum(enum_data) => {
            let mut enum_field_counts = Vec::new();
            let mut enum_translates = Vec::new();
            let mut enum_indexes = Vec::new();
            let mut enum_mut_indexes = Vec::new();
            let mut enum_draws = Vec::new();

            enum_data.variants.iter().for_each(|variant| {
                let variant_name = &variant.ident;

                let (enum_field_count, enum_translate, enum_index, enum_mut_index, enum_draw) =
                    match &variant.fields {
                        Fields::Named(FieldsNamed { named, .. }) => {
                            let field_idents = named
                                .iter()
                                .map(|field| field.ident.as_ref().unwrap())
                                .collect::<Vec<_>>();

                            let fields_count = named.iter().count();
                            let enum_field_count = quote! {
                                Self::#variant_name { ..  } => {
                                    #fields_count
                                }
                            };

                            let translate_fields = field_idents
                                .iter()
                                .map(|f| quote!(#f: #f.clone().translate(by)));
                            let enum_translate = quote! {
                                Self::#variant_name { #(#field_idents,)* } => {
                                    Self::#variant_name {
                                        #(#translate_fields,)*
                                    }
                                }
                            };

                            let fields_index = field_idents
                                .iter()
                                .enumerate()
                                .map(|(i, f)| quote!(#i => #f,))
                                .collect::<Vec<_>>();
                            let enum_index = quote! {
                                Self::#variant_name { #(#field_idents,)* } => {
                                    match index {
                                        #(#fields_index)*
                                        _ => #empty_vg_instance,
                                    }
                                }
                            };
                            let enum_mut_index = quote! {
                                Self::#variant_name { #(#field_idents,)* } => {
                                    match index {
                                        #(#fields_index)*
                                        _ => #empty_vg_instance_mut,
                                    }
                                }
                            };

                            let fields_draw =
                                field_idents.iter().map(|f| quote!(#f.draw(display)?;));
                            let enum_draw = quote! {
                                Self::#variant_name { #(#field_idents,)* } => {
                                    #(#fields_draw)*
                                }
                            };

                            (
                                enum_field_count,
                                enum_translate,
                                enum_index,
                                enum_mut_index,
                                enum_draw,
                            )
                        }
                        Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
                            let field_idents = unnamed
                                .iter()
                                .enumerate()
                                .map(|(num, _)| format_ident!("__self_{}", num))
                                .collect::<Vec<_>>();

                            let fields_count = unnamed.iter().count();
                            let enum_field_count = quote! {
                                Self::#variant_name(..) => {
                                    #fields_count
                                }
                            };

                            let translate_fields =
                                field_idents.iter().map(|f| quote!(#f.clone().translate(by), ));
                            let enum_translate = quote! {
                                Self::#variant_name(#(#field_idents),*) => {
                                    Self::#variant_name(
                                        #(#translate_fields)*
                                    )
                                }
                            };

                            let fields_index = field_idents
                                .iter()
                                .enumerate()
                                .map(|(i, f)| quote!(#i => #f,))
                                .collect::<Vec<_>>();
                            let enum_index = quote! {
                                Self::#variant_name(#(#field_idents),*) => {
                                    match index {
                                        #(#fields_index)*
                                        _ => #empty_vg_instance,
                                    }
                                }
                            };

                            let enum_mut_index = quote! {
                                Self::#variant_name(#(#field_idents),*) => {
                                    match index {
                                        #(#fields_index)*
                                        _ => #empty_vg_instance_mut,
                                    }
                                }
                            };

                            let field_draws =
                                field_idents.iter().map(|f| quote!(#f.draw(display)?;));
                            let enum_draw = quote! {
                                Self::#variant_name(#(#field_idents),*) => {
                                    #(#field_draws)*
                                }
                            };

                            (
                                enum_field_count,
                                enum_translate,
                                enum_index,
                                enum_mut_index,
                                enum_draw,
                            )
                        }
                        Fields::Unit => {
                            let enum_field_count = quote! {
                                Self::#variant_name => 0,
                            };
                            let enum_translate = quote! {
                                Self::#variant_name => Self::#variant_name,
                            };
                            let enum_index = quote! {
                                Self::#variant_name => {
                                    #empty_vg_instance
                                }
                            };

                            let enum_mut_index = quote! {
                                Self::#variant_name => {
                                    #empty_vg_instance_mut
                                }
                            };
                            let enum_draw = quote! {
                                Self::#variant_name => {}
                            };
                            (
                                enum_field_count,
                                enum_translate,
                                enum_index,
                                enum_mut_index,
                                enum_draw,
                            )
                        }
                    };

                enum_field_counts.push(enum_field_count);
                enum_translates.push(enum_translate);
                enum_indexes.push(enum_index);
                enum_mut_indexes.push(enum_mut_index);
                enum_draws.push(enum_draw);
            });

            let field_count_impl = quote! {
                match self {
                    #(#enum_field_counts)*
                }
            };

            let index_impl = quote! {
                match self {
                    #(#enum_indexes)*
                }
            };

            let index_mut_impl = quote! {
                match self {
                    #(#enum_mut_indexes)*
                }
            };

            let translate_impl = quote! {
                match self {
                    #(#enum_translates)*
                }
            };

            let draw_impl = quote! {
                match self {
                    #(#enum_draws)*
                }
            };

            (
                field_count_impl,
                index_impl,
                index_mut_impl,
                translate_impl,
                draw_impl,
            )
        }
        _ => panic!("derive(ViewGroup) only supports structs with named fields and enums"),
    };

    let (impl_generics, ty_generics, where_clause) = ast.generics.split_for_impl();

    let name = &ast.ident;

    let gen_view_group = quote! {
        impl #impl_generics embedded_layout::view_group::ViewGroup for #name #ty_generics #where_clause {
            fn len(&self) -> usize {
                #field_count_impl
            }

            fn at(&self, index: usize) -> &dyn embedded_layout::View {
                #index_impl
            }

            fn at_mut(&mut self, index: usize) -> &mut dyn embedded_layout::View {
                #index_mut_impl
            }
        }

        impl #impl_generics embedded_graphics::transform::Transform for #name #ty_generics #where_clause {
            fn translate(&self, by: Point) -> Self {
                #translate_impl
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
                    #draw_impl

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

    TokenStream::from(generated)
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
