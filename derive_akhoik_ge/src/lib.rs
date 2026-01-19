use proc_macro::TokenStream;
use quote::quote;
use syn::{DeriveInput, parse_macro_input};

#[proc_macro_derive(Object3D)]
pub fn object_3d_derive_macro(input: TokenStream) -> TokenStream {
    // parse
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = ast.ident;

    // generate
    let expanded = quote! {
    impl Object3D for #ident {
        fn calculate_transform(&self) -> Mat4 {
            calculate_transform(self)
        }

        fn recalculate_transform(&mut self) {
            self.transform = calculate_transform(self);
        }

        fn get_position(&self) -> Vector3 {
            self.position
        }

        fn set_position(&mut self, pos: Vector3) {
            self.position = pos;
            self.recalculate_transform();
        }

        fn get_rotation(&self) -> Vector3 {
            self.rotation
        }

        fn set_rotation(&mut self, rot: Vector3) {
            self.rotation = rot;
        }

        fn get_front(&self) -> Vector3 {
            self.front
        }

        fn set_front(&mut self, front: Vector3) {
            self.front = front;
        }

        fn get_right(&self) -> Vector3 {
            self.right
        }

        fn set_right(&mut self, right: Vector3) {
            self.right = right;
        }

        fn get_up(&self) -> Vector3 {
            self.up
        }

        fn set_up(&mut self, up: Vector3) {
            self.up = up;
        }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(Object3DSize)]
pub fn object_3d_size_derive_macro(input: TokenStream) -> TokenStream {
    // parse
    let ast = parse_macro_input!(input as DeriveInput);

    let ident = ast.ident;

    // generate
    let expanded = quote! {
        impl Object3DSize for #ident {
            fn get_size(&self) -> Vector3 {
                self.size
            }

            fn set_size(&mut self, size: Vector3) {
                self.size = size;
            }
        }
    };

    TokenStream::from(expanded)
}
