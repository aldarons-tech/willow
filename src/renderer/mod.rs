// Willow Graphics API
//
// Copyright 2017 (c) Aldaron's Tech
// Copyright 2017 (c) Jeron Lau
// Licensed under the MIT LICENSE
//
// src/renderer/mod.rs

use ami::Void;

mod ffi;

use self::ffi::vulkan;
use self::ffi::NativeRenderer;
use RenderOps;

use window::Window;
use window::WindowConnection;

type VkQueue = usize;
type VkPhysicalDevice = usize;

type VkSurface = u64;
type VkSwapchain = u64;
type VkImage = u64;
type VkFramebuffer = u64;
type VkFence = u64;
type VkDescriptorPool = u64;
type VkDescriptorSetLayout = u64;
type VkDescriptorSet = u64;
type VkImageView = u64;
type VkDeviceMemory = u64;
type VkRenderPass = u64;
type VkPipelineLayout = u64;
type VkSemaphore = u64;
type VkBuffer = u64;
type VkShaderModule = u64;
type VkSampler = u64;
type VkPipeline = u64;

type VkC = u32; // Size of enum is 4 bytes

#[repr(C)]
#[derive(Copy, Clone)] // TODO: don't copy this.
pub struct Vw {
	pub instance: *mut Void, // Vulkan instance
	surface: VkSurface, // Surface that we render to.
	present_queue_index: u32,
	present_queue: VkQueue,
	gpu: VkPhysicalDevice,
	device: *mut Void, // The logical device
	command_buffer: *mut Void,
	swapchain: VkSwapchain,
	width:u32, height:u32, // Swapchain Dimensions.
	present_images: [VkImage; 2], // 2 for double-buffering
	frame_buffers: [VkFramebuffer; 2], // 2 for double-buffering
	color_format: VkC, // VkFormat
	image_count: u32, // 1 (single-buffering) or 2 (double-buffering)
	submit_fence: VkFence, // The submit fence
	present_image_views: [VkImageView; 2], // 2 for double-buffering
	depth_image: VkImage,
	depth_image_view: VkImageView,
	depth_image_memory: VkDeviceMemory,
	render_pass: VkRenderPass,
	next_image_index: u32,
	presenting_complete_sem: VkSemaphore,
	rendering_complete_sem: VkSemaphore,
	offsets: u64, // VkDeviceSize
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Shader {
	vertex: VkShaderModule,
	fragment: VkShaderModule,
	textures: u32,
	has_data: u8,
}

impl Shader {
	pub fn create(vw: &Vw, vert: &'static [u8], frag:&'static [u8],
		textures: u32) -> Shader
	{
		// TODO: has_data
		let mut shader = Shader { vertex: 0, fragment: 0,
			textures, has_data: 0 };
		unsafe {
			vw_vulkan_shader(&mut shader, *vw, &vert[0],
				vert.len() as u32, &frag[0], frag.len() as u32);
		}
		shader
	}
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct Style {
	pipeline: VkPipeline,
	pub descsetlayout: VkDescriptorSetLayout,
	pipeline_layout: VkPipelineLayout,
}

#[repr(C)]
#[derive(PartialEq,Copy,Clone)]
pub struct NativeTexture {
	mappable_image: VkImage,
	mappable_memory: VkDeviceMemory,
	image: VkImage,
	memory: VkDeviceMemory,
	sampler: VkSampler,
	view: VkImageView,
	w: u32,
	h: u32,
	size: u32,
	pitch: u32,
	staged: u8,
}

#[repr(C)]
pub struct VwShape {
	vertex_buffer_memory: VkDeviceMemory,
	vertex_input_buffer: VkBuffer,
	vertice_count: u32,
}

pub struct Shape {
	shape: VwShape,
	hastx: bool,
	instance: VwInstance,
}

// TODO
/*impl Shape {
	pub fn create(window: &mut Window, v: &[f32], style: style::Style) -> Shape {
		let size = v.len() as u32;
		let hastx = {
			match style {
				style::Style::Solid(_) => false,
				style::Style::Texture(_, _) => true,
				style::Style::Invisible => {
					panic!("Can't create a Sprite with \
						invisible style.")
				}
			}
		};
		let mut shape = VwShape {
			vertex_buffer_memory: 0,
			vertex_input_buffer: 0,
			vertice_count: size / 8,
		};
		unsafe { vw_vulkan_shape(&mut shape, window.vw, &v[0], size); }
		Shape {
			shape: shape,
			hastx: hastx,
			instances: Vec::new(),
		}
	}

	pub fn animate(window: &mut Window, index: usize, i: usize,
		texture: *const NativeTexture, style: Style)
	{
		let hastx = window.sprites[index].hastx;

		// Must be same style
		if hastx {
			if (texture as *const _ as usize) == 0 {
				panic!("Can't set Style of a Sprite initialized\
					with Style::Texture to Style::Solid");
			}
		} else {
			if (texture as *const _ as usize) != 0 {
				panic!("Can't set Style of a Sprite initialized\
					with Style::Solid to Style::Texture");
			}
		}

		// Free old Style, and set new uniform buffers.
		unsafe {
			vw_uniform_uniforms_free(&window.vw, &mut
				window.sprites[index].instances[i].instance);
			window.sprites[index].instances[i].instance =
				vw_vulkan_uniforms(&window.vw, style, texture,
					if hastx { 1 } else { 0 });
		}
		// TODO: Optimize when using same value from vw_vulkan_uniforms
		// ( Same todo as in extern )
		// Set texture
//		unsafe {
//			vw_vulkan_txuniform(&window.vw,
//				&mut window.sprites[index].shape.instances[i].instance, texture,
//				if window.sprites[index].shape.hastx { 1 } else { 0 });
//		}
		Shape::enable(window, index, i, true);
	}

	pub fn add(window: &mut Window, index: usize, tx: *const NativeTexture,
		style: Style)
	{
		let shape = &mut window.sprites[index];
		let mem = VwLinkedInstance {
			instance: unsafe {
				vw_vulkan_uniforms(&window.vw, style, tx,
					if shape.hastx { 1 } else { 0 })
			},
			enabled: true,
		};
		vulkan::copy_memory(window.vw.device,
			mem.instance.uniform_memory, &mem.matrix);
		shape.instances.push(mem);
	}

	pub fn draw(window: &mut Window, index: usize) {
		let shape = &window.sprites[index];
		for i in 0..shape.instances.len() {
			if !window.sprites[index].instances[i].enabled {
				continue;
			}
			unsafe {
				vw_vulkan_draw_shape(&mut window.vw,
					&shape.shape,
					&shape.instances[i].matrix[0],
					shape.instances[i].instance);
			}
			vulkan::cmd_draw(window.vw.command_buffer,
				shape.shape.vertice_count);
		}
	}

	pub fn matrix(window: &mut Window, index: usize, i: usize,
		matrix: [f32; 16])
	{
		window.sprites[index].instances[i].matrix = matrix;
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].instances[i].instance.uniform_memory,
			&window.sprites[index].instances[i].matrix);
	}

	pub fn vertices(window: &Window, index: usize, v: &[f32]) {
		vulkan::copy_memory(window.vw.device,
			window.sprites[index].shape.vertex_buffer_memory, v);
	}
}*/

#[repr(C)]
#[derive(Copy, Clone)]
pub struct VwInstance {
	matrix_buffer: VkBuffer,
	uniform_memory: VkDeviceMemory,
	pub desc_set: VkDescriptorSet,
	pub desc_pool: VkDescriptorPool,
	pub pipeline: Style,
}

extern {
	fn vw_vulkan_shape(a: *mut VwShape, b: Vw, c: *const f32, d: u32) -> ();
//
	fn vw_vulkan_shader(a: *mut Shader, b: Vw, c: *const u8, d: u32,
		e: *const u8, f: u32) -> ();
	fn vw_vulkan_pipeline(z: *mut Style, a: *mut Vw, b: *const Shader,
		c: u32);
	fn vw_vulkan_draw_begin(v: *mut Vw, r: f32, g: f32, b: f32) -> ();
	fn vw_uniform_uniforms_free(v: *const Vw, b: *mut VwInstance) -> ();
// TODO: Use for optimization instead of freeing and reallocating uniform
// buffers when pipeline doesn't change.
//	fn vw_vulkan_txuniform(vw: *const Vw, b: *mut VwInstance,
//		c: *const NativeTexture, d: u8) -> ();
	fn vw_instance_new(a: *const Vw, b: Style, floats: i32) -> VwInstance;
	fn vw_vulkan_uniforms(a: *const Vw, b: Style,
		c: *const NativeTexture, d: u8) -> VwInstance;
	fn vw_vulkan_draw_shape(v: *mut Vw, s: *const VwShape, f: VwInstance)
		-> ();
	fn vw_vulkan_draw_update(v: *mut Vw) -> ();
	fn vw_vulkan_resize(v: *mut Vw) -> ();
	fn vw_vulkan_swapchain_delete(v: *mut Vw) -> ();
}

/*pub fn make_styles(vw: &mut Vw, extrashaders: &[Shader], shaders: &mut Vec<Style>)
{
	let mut shadev = Vec::new();
	let default_shaders = [
//		Shader::create(vw, include_bytes!("res/texture-vert.spv"),
//			include_bytes!("res/texture-frag.spv"), 1),
	];
	shadev.extend(default_shaders.iter().cloned());
	shadev.extend(extrashaders.iter().cloned());

	*shaders = vec![Style { pipeline: 0, descsetlayout: 0,
		pipeline_layout: 0 }; shadev.len()];
	unsafe {
		vw_vulkan_pipeline(&mut shaders[0], vw, &shadev[0],
			shadev.len() as u32);
	}
}*/

pub fn close(renderer: &mut Vw) {
	unsafe { vw_vulkan_swapchain_delete(renderer); }
}

impl Vw {
	pub fn new(window_name: &str, window_connection: WindowConnection) -> Vw {
		let instance = vulkan::Instance::create(window_name);
		let surface = vulkan::Surface::create(&instance, window_connection);
		let gpu = vulkan::Gpu::create(&surface);
		let gpu_interface = vulkan::GpuInterface::create(&instance, &gpu);
		let queue = vulkan::Queue::create(&gpu_interface, &gpu);
		let command_buffer = vulkan::CommandBuffer::create(&gpu_interface,&gpu);

		let mut vw = Vw {
			instance: instance.native.native(),
			surface: surface.native,
			present_queue_index: gpu.present_queue_index,
			present_queue: queue.native,
			gpu: gpu.native,
			device: gpu_interface.native,
			command_buffer: command_buffer.native,
			swapchain: 0,
			width: 0, height: 0,
			present_images: [0, 0],
			frame_buffers: [0, 0],
			color_format: 0,
			image_count: 0,
			submit_fence: 0,
			present_image_views: [0, 0],
			depth_image: 0,
			depth_image_view: 0,
			depth_image_memory: 0,
			render_pass: 0,
			next_image_index: 0,
			presenting_complete_sem: 0,
			rendering_complete_sem: 0,
			offsets: 0,
		};

		unsafe {
			vw_vulkan_resize(&mut vw);
		}

		vw
	}
}

fn projection(ratio: f32, fov: f32) -> ::Transform {
	let scale = (fov * 0.5 * ::std::f32::consts::PI / 180.).tan().recip();
	let xscale = scale * ratio;

	matrix![
		xscale,	0.,	0.,	0.,
		0.,	scale,	0.,	0.,
		0.,	0.,	1.,	0.,
		0.,	0.,	1., 	1.,
	]
}

pub struct Renderer {
	vw: Vw,
	shapes: Vec<Shape>,
	style_solid: Style,
	projection: ::Transform,
}

impl Renderer {
	pub fn new(window_name: &str, window_connection: WindowConnection)
		-> Renderer
	{
//		let native = NativeRenderer::new(window_name,
//			window_connection.clone());

		let mut vw = Vw::new(window_name, window_connection);
		let shapes = Vec::new();
		let shadev = vec![
			Shader::create(&vw, include_bytes!("../native_renderer/vulkan/res/solid-vert.spv"),
				include_bytes!("../native_renderer/vulkan/res/solid-frag.spv"), 0)
		];
		let mut styles = vec![Style { pipeline: 0, descsetlayout: 0,
			pipeline_layout: 0 }; shadev.len()];

		unsafe {
			vw_vulkan_pipeline(&mut styles[0], &mut vw, &shadev[0],
				shadev.len() as u32);
		}

		let projection = projection(vw.height as f32 / vw.width as f32,
			90.0);

		Renderer { vw, shapes, style_solid: styles[0], projection }
	}

	pub fn update(&mut self) {
//		let color = self.color;

		unsafe {
			vw_vulkan_draw_begin(&mut self.vw, 0.0, 0.0, 1.0);
		}

		for shape in &self.shapes {
			unsafe {
				vw_vulkan_draw_shape(&mut self.vw, &shape.shape,
					shape.instance);
			}

			vulkan::cmd_draw(self.vw.command_buffer,
				shape.shape.vertice_count);
		}

		unsafe {
			vw_vulkan_draw_update(&mut self.vw);
		}
	}

	pub fn resize(&mut self, size: (u32, u32)) {
		self.vw.width = size.0;
		self.vw.height = size.1;

		unsafe {
			vw_vulkan_swapchain_delete(&mut self.vw);
			vw_vulkan_resize(&mut self.vw);
		}

		self.shapes.clear();
		self.projection = projection(size.1 as f32/size.0 as f32, 90.0);
	}

	pub fn solid(&mut self, vertices: Vec<f32>, color: ::Color) -> usize {
		let size = vertices.len() as u32;
		let hastx = false;
		let mut shape = VwShape {
			vertex_buffer_memory: 0,
			vertex_input_buffer: 0,
			vertice_count: size / 4,
		};

		unsafe {
			vw_vulkan_shape(&mut shape, self.vw, vertices.as_ptr(),
				size);
		}

		let a = self.shapes.len();

		// Add an instance
		let instance = unsafe {
			vw_instance_new(&self.vw, self.style_solid, 4)
		};

		let matrix = [ color.0, color.1, color.2, color.3 ];

		vulkan::copy_memory(self.vw.device,
			instance.uniform_memory, &matrix);

		println!("PUSH SHAPE");
		self.shapes.push(Shape { shape, hastx, instance });

		a
	}

	pub fn get_projection(&self) -> ::Transform {
		::Transform(self.projection.0)
	}
}
