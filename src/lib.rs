// lib.rs -- Aldaron's Device Interface / GPU
// Copyright (c) 2018  Jeron A. Lau <jeron.lau@plopgrizzly.com>
// Licensed under the MIT LICENSE

//! Aldaron's Device Interface / GPU is a library developed by Plop Grizzly for
//! interfacing with the GPU to render graphics or do fast calculations.

extern crate adi_gpu_opengl;
extern crate adi_gpu_vulkan;
extern crate adi_gpu_base;
extern crate ami;

pub use ami::Mat4;

use adi_gpu_base as base;

pub use base::{
	afi, Graphic,
	Model, TexCoords, Gradient, Shape,
	Input, Key, Click, Msg, // Window
	Display as DisplayTrait, Texture as TextureTrait, // Traits
};

/// Create a new Vulkan / OpenGL Display.
pub fn new_display<G: AsRef<Graphic>>(title: &str, icon: G)
	-> Result<Display, String>
{
	let mut err = "".to_string();

	// Try Vulkan first.
	match adi_gpu_vulkan::Display::new(title, &icon) {
		Ok(vulkan) => return Ok(Display::Vulkan(vulkan)),
		Err(vulkan) => err.push_str(vulkan),
	}

	// Fallback on OpenGL/OpenGLES
	err.push('\n');
	match adi_gpu_opengl::Display::new(title, &icon) {
		Ok(opengl) => return Ok(Display::OpenGL(opengl)),
		Err(opengl) => err.push_str(opengl),
	}

	// No more options
	Err(err)
}

/// To render anything with adi_gpu, you have to make a `Display`
pub enum Display {
	Vulkan(adi_gpu_vulkan::Display),
	OpenGL(adi_gpu_opengl::Display),
}

impl DisplayTrait for Display {
	type Texture = Texture;

	fn new<G: AsRef<Graphic>>(_title: &str, _icon: G)
		-> Result<Self, &'static str>
	{
		Err("Use new_display(), not Display::new()")
	}

	fn color(&mut self, color: (f32, f32, f32)) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.color(color)
			}
			Display::OpenGL(ref mut display) => {
				display.color(color)
			}
		}
	}

	fn update(&mut self) -> Option<Input> {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.update()
			}
			Display::OpenGL(ref mut display) => {
				display.update()
			}
		}
	}

	fn camera(&mut self, xyz: (f32,f32,f32), rotate_xyz: (f32,f32,f32)) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.camera(xyz, rotate_xyz)
			}
			Display::OpenGL(ref mut display) => {
				display.camera(xyz, rotate_xyz)
			}
		}
	}

	fn model(&mut self, vertices: &[f32]) -> Model {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.model(vertices)
			}
			Display::OpenGL(ref mut display) => {
				display.model(vertices)
			}
		}
	}

	fn fog(&mut self, fog: Option<(f32, f32)>) -> () {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.fog(fog)
			}
			Display::OpenGL(ref mut display) => {
				display.fog(fog)
			}
		}
	}

	fn texture<G: AsRef<Graphic>>(&mut self, graphic: G) -> Texture {
		match *self {
			Display::Vulkan(ref mut display) => {
				Texture::Vulkan(display.texture(graphic))
			}
			Display::OpenGL(ref mut display) => {
				Texture::OpenGL(display.texture(graphic))
			}
		}
	}

	fn gradient(&mut self, colors: &[f32]) -> Gradient {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.gradient(colors)
			}
			Display::OpenGL(ref mut display) => {
				display.gradient(colors)
			}
		}
	}

	fn texcoords(&mut self, texcoords: &[f32]) -> TexCoords {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.texcoords(texcoords)
			}
			Display::OpenGL(ref mut display) => {
				display.texcoords(texcoords)
			}
		}
	}

	fn set_texture(&mut self, texture: &mut Self::Texture, pixels: &[u32]) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.set_texture(
					match *texture {
						Texture::Vulkan(ref mut texture) => {
							texture
						}
						_ => panic!("mismatch"),
					}, pixels)
			}
			Display::OpenGL(ref mut display) => {
				display.set_texture(
					match *texture {
						Texture::OpenGL(ref mut texture) => {
							texture
						}
						_ => panic!("mismatch"),
					}, pixels)
			}
		}
	}

	#[inline(always)]
	fn shape_solid(&mut self, model: &Model, transform: Mat4,
		color: [f32; 4], blending: bool, fog: bool, camera: bool)
		-> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_solid(model, transform, color,
					blending, fog, camera)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_solid(model, transform, color,
					blending, fog, camera)
			}
		}
	}

	#[inline(always)]
	fn shape_gradient(&mut self, model: &Model, transform: Mat4,
		colors: Gradient, blending: bool, fog: bool, camera: bool)
		-> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_gradient(model, transform, colors,
					blending, fog, camera)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_gradient(model, transform, colors,
					blending, fog, camera)
			}
		}
	}

	#[inline(always)]
	fn shape_texture(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, blending: bool, fog: bool,
		camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_texture(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, blending, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_texture(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, blending, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_faded(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, alpha: f32, fog: bool,
		camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_faded(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, alpha, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_faded(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, alpha, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_tinted(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, tint: [f32; 4], blending: bool,
		fog: bool, camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_tinted(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tint, blending, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_tinted(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tint, blending, fog, camera
				)
			}
		}
	}

	#[inline(always)]
	fn shape_complex(&mut self, model: &Model, transform: Mat4,
		texture: &Texture, tc: TexCoords, tints: Gradient,
		blending: bool, fog: bool, camera: bool) -> Shape
	{
		match *self {
			Display::Vulkan(ref mut display) => {
				display.shape_complex(
					model, transform,
					match texture {
						Texture::Vulkan(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tints, blending, fog, camera
				)
			}
			Display::OpenGL(ref mut display) => {
				display.shape_complex(
					model, transform,
					match texture {
						Texture::OpenGL(texture) => {
							texture
						}
						_ => panic!("mismatch"),
					},
					tc, tints, blending, fog, camera
				)
			}
		}
	}

	fn transform(&mut self, shape: &mut Shape, transform: Mat4) {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.transform(shape, transform)
			}
			Display::OpenGL(ref mut display) => {
				display.transform(shape, transform)
			}
		}
	}

	fn resize(&mut self, wh: (u32, u32)) -> () {
		match *self {
			Display::Vulkan(ref mut display) => {
				display.resize(wh)
			}
			Display::OpenGL(ref mut display) => {
				display.resize(wh)
			}
		}
	}

	fn wh(&self) -> (u32, u32) {
		match *self {
			Display::Vulkan(ref display) => {
				display.wh()
			}
			Display::OpenGL(ref display) => {
				display.wh()
			}
		}
	}
}

pub enum Texture {
	Vulkan(adi_gpu_vulkan::Texture),
	OpenGL(adi_gpu_opengl::Texture)
}

impl base::Texture for Texture {
	/// Get the width and height.
	fn wh(&self) -> (u32, u32) {
		let this: &base::Texture = match *self {
			Texture::Vulkan(ref texture) => {
				texture
			}
			Texture::OpenGL(ref texture) => {
				texture
			}
		};
		this.wh()
	}
}
