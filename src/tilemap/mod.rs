use crate::{Input, Shader, Sprite, Texture, Window};
use alexandria::{ConstantBuffer, Texture2D, UpdateRegion, Vector2};
use std::{cell::RefCell, rc::Rc};

mod spritesheet;

pub use spritesheet::*;

pub struct Tilemap {
    shader: Rc<RefCell<Shader>>,
    sprites: Box<[(Sprite, bool)]>,
    textures: Box<[(Texture2D<u16>, f32)]>,

    buffer: TilemapBuffer,
    cbuffer: ConstantBuffer<TilemapBuffer>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct TilemapBuffer {
    size: Vector2,
    sprite_size: f32,
    reserved: f32,
}

impl Tilemap {
    /* Creates a new tilemap
    The layers input takes one triple for each layer in the tilemap
    The first value is the layers texture, which should be made with a Spritesheet
    The second is the initial value of the layer, as a key returned by the spritesheet
    The final value is the number of sprites

    The z_order and z_order_diff will determine the first layer's z_order and how the following
    layers z_orders will be offset.
    */
    pub fn new<I: Input>(
        width: usize,
        height: usize,
        initial_z_order: f32,
        z_order_diff: f32,
        layers: &[(Texture, u16, u16)],
        window: &mut Window<I>,
    ) -> Self {
        let shader = window.get_tilemap_shader();

        let mut sprites = Vec::with_capacity(layers.len());
        let mut textures = Vec::with_capacity(layers.len());

        let mut z_order = initial_z_order;
        for (texture, initial_value, num_sprites) in layers {
            // Build the sprite
            let mut sprite = Sprite::new(Some(texture.clone()));
            sprite
                .transform_mut()
                .set_scale(Vector2::new(width as f32, height as f32));
            sprite
                .transform_mut()
                .set_position(Vector2::new((width as f32) / 2.0, (height as f32) / 2.0));
            sprite.transform_mut().set_z_order(z_order);
            sprites.push((sprite, true));
            z_order += z_order_diff;

            // Build the data texture
            textures.push((
                Texture2D::new(
                    &vec![*initial_value; width * height],
                    width,
                    height,
                    1,
                    alexandria::SampleType::Point,
                    window.inner(),
                )
                .unwrap(),
                1.0 / (*num_sprites as f32),
            ));
        }

        let buffer = TilemapBuffer {
            size: Vector2::new(width as f32, height as f32),
            sprite_size: 0.0,
            reserved: 0.0,
        };
        let cbuffer = ConstantBuffer::new(buffer.clone(), 2, window.inner()).unwrap();

        Tilemap {
            shader,
            sprites: sprites.into_boxed_slice(),
            textures: textures.into_boxed_slice(),
            buffer,
            cbuffer,
        }
    }

    pub fn set_tile(&mut self, layer: usize, x: usize, y: usize, tile: u16) {
        assert!(x < self.buffer.size.x() as usize);
        assert!(y < self.buffer.size.y() as usize);
        assert!(layer < self.textures.len());

        self.textures[layer]
            .0
            .update_region(UpdateRegion::new(x, y, 1, 1), &[tile]);
    }

    pub fn render<I: Input>(&mut self, window: &mut Window<I>) {
        self.shader.borrow_mut().set_active(window);
        for i in 0..self.sprites.len() {
            if self.sprites[i].1 {
                self.buffer.sprite_size = self.textures[i].1;
                self.cbuffer.set_data(self.buffer.clone()).unwrap();
                self.cbuffer.set_active();

                self.textures[i].0.set_active();
                self.sprites[i].0.render(window);
                self.textures[i].0.clear_active();

                self.cbuffer.clear_active();
            }
        }
    }
}
