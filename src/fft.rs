use num::complex::Complex;
use std::f32::consts::PI;

use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::buffer::CpuBufferPool;
use vulkano::buffer::ImmutableBuffer;

//command buffer takes n commands i think
use vulkano::command_buffer::AutoCommandBufferBuilder;

//command buffer synchornization library - well it does this
//probably toher stuff - need it to execute command buffer
use vulkano::command_buffer::CommandBuffer;

//so i can execute command buffer and no fuck the gpu?
//think ajax promise eli
use vulkano::sync::GpuFuture;

//shade rpipeline stuff
use std::sync::Arc;
use vulkano::pipeline::ComputePipeline;

//descriptor set libraries
use vulkano::descriptor::descriptor_set::PersistentDescriptorSet;
use vulkano::descriptor::PipelineLayoutAbstract;

//this is image shit
use vulkano::device::Queue;
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;

use image::{ImageBuffer, Rgba};
use vulkano::format::ClearValue;

fn _gen_twiddle_table(n: u32) -> Vec<Complex<f32>> {
    let table = vec![Complex::<f32> { re: 0.0, im: 0.0 }; n as usize];
    for k in 0..n {
        table[k as usize] = Complex::i()
            .scale(-2.0 * PI * (k as f32) / (n as f32))
            .exp()
    }

    table
}

fn fft_setup(device: Arc<Device>, queue: Arc<Queue>) -> Result<Vec<Complex<f32>>, &'static str> {
    // Create I/O and twiddle buffers for the DFT
    let in_samps = (0..64)
        .map(|k| Complex::<f32> {
            re: k as f32,
            im: -k as f32,
        })
        .collect();
    let in_buf =
        CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, in_samps)
            .expect("failed to create buffer");
    let twiddle_table = _gen_twiddle_table(64);
    let (twiddle_buf, twiddle_buf_execf) =
        ImmutableBuffer::from_data(twiddle_table, BufferUsage::all(), queue).unwrap();
    // TODO wait for twiddle_buf to finish loading
    let out_spec = (0..64)
        .map(|k| Complex::<f32> { re: 0.0, im: 0.0 })
        .collect();
    let out_buf =
        CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, out_spec)
            .expect("failed to create buffer");

    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            src: "
#version 450

layout(location = 0) in vec2 in_samps;
layout(location = 0) in vec2 twiddle_table;
layout(location = 0) out vec2 out_spec;

void rmder(int a, int b) {
    return a - a * (a / b);
}

void main() {
    int i, j;
    // TODO (ryan): see if we actually have to flush this in GLSL
    for (i = 0; i < 64; i++) {
         out_spec[i] = 0;
    }

    for (i = 0; i < 64; i++) {
        for (j = 0; j < 64; j++) {
            out_spec[i] += in_samps[j] *
            twiddle_table[rmder(i * j, 64)];
        } 
    }
}"
        }
    }

    // Set up shader pipeline and descriptors
    let shader = cs::Shader::load(device.clone()).expect("failed to load shader");
    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compute pipeline"),
    );
    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();
    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_buffer(in_buf.clone())
            .unwrap()
            .add_buffer(twiddle_buf.clone())
            .unwrap()
            .add_buffer(out_buf.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    // Build the shader
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .dispatch([256, 1, 1], compute_pipeline.clone(), set.clone(), ())
        .unwrap();
    let command_buffer = builder.build().unwrap();

    // Run and wait for it to finish
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let content = out_spec.read().unwrap();
    Ok(content)
}
