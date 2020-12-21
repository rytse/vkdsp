use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;

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
use vulkano::format::Format;
use vulkano::image::Dimensions;
use vulkano::image::StorageImage;

use image::{ImageBuffer, Rgba};
use vulkano::format::ClearValue;

fn main() {
    let instance =
        Instance::new(None, &InstanceExtensions::none(), None).expect("failed to create instance");

    let physical = PhysicalDevice::enumerate(&instance)
        .next()
        .expect("no device available");

    for family in physical.queue_families() {
        println!(
            "Found a queue family with {:?} queue(s)",
            family.queues_count()
        );
    }

    let queue_family = physical
        .queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");

    /*Device::new(physical, &Features::none(), &DeviceExtensions::none(),
    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")*/
    let (device, mut queues) = {
        Device::new(
            physical,
            &Features::none(),
            &DeviceExtensions {
                khr_storage_buffer_storage_class: true,
                ..DeviceExtensions::none()
            },
            [(queue_family, 0.5)].iter().cloned(),
        )
        .expect("failed to create device")
    };
    let queue = queues.next().unwrap();

    //cpu buffer

    let data = 12;
    let buffer = CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data)
        .expect("failed to create buffer");

    struct MyStruct {
        a: u32,
        b: bool,
    }

    let data = MyStruct { a: 5, b: true };

    let buffer =
        CpuAccessibleBuffer::from_data(device.clone(), BufferUsage::all(), false, data).unwrap();

    let iter = (0..128).map(|_| 5u8);
    let buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, iter).unwrap();

    let mut content = buffer.write().unwrap();
    //for mystruct buffer
    //content.a *=2;
    //content.b = false;

    // "array" buffer

    content[12] = 83;
    content[7] = 3;

    //creating buffers
    //dumby int data (0,64)
    let source_content = 0..64;
    //source cpu buffer
    let source =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, source_content)
            .expect("failed to create buffer");

    //destination content
    //I think has to be the same size as source
    let dest_content = (0..64).map(|_| 0); //64 0s
                                           //actually make destination buffer - pass it dest_content
    let dest =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, dest_content)
            .expect("failed to create buffer");

    //command buffer code
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder.copy_buffer(source.clone(), dest.clone()).unwrap();
    let command_buffer = builder.build().unwrap();

    //execute command buffer code
    let finished = command_buffer.execute(queue.clone()).unwrap();

    //this is weird... something something wiat for command buffer
    //until gpu executes
    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    //yay now we can read stuff
    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();
    assert_eq!(&*src_content, &*dest_content);

    //---------------------------------------------
    // This part is interesting, we are actually gonna
    // do a gpu operation, multiply 65546 values by 12
    //---------------------------------------------

    let data_iter = 0..65536;
    //let mut array = vec![vec![0; 65536]; 2];
    //let arr = vec![1; 8];
    //let arr = arr.into_iter();
    //let arr = (vec![arr; 4]).into_iter();
    //let data_array = data_iter.[into_]iter().collect::<Vec<u32>>().as_slice().try_into().unwrap();
    //let data_iter = 0..131072;

    let data_buffer =
        CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, data_iter)
            .expect("failed to create buffer");

    //inserting GLSL code into rust

    mod cs {
        vulkano_shaders::shader! {
            ty: "compute",
            src: "
	#version 450

	layout(local_size_x = 8, local_size_y = 1, local_size_z = 1) in;

	layout(set = 0, binding = 0) buffer Data {
	    uint data[];
	} buf;

	void main() {
	    uint idx = gl_GlobalInvocationID.x;
	    buf.data[idx] = idx;
	}"
        }
    }
    //call the shader we just wrote?
    let shader = cs::Shader::load(device.clone()).expect("failed to create shader module");

    //call compute pipeline
    let compute_pipeline = Arc::new(
        ComputePipeline::new(device.clone(), &shader.main_entry_point(), &())
            .expect("failed to create compue pipeline"),
    );

    //descriptor magic
    let layout = compute_pipeline.layout().descriptor_set_layout(0).unwrap();

    let set = Arc::new(
        PersistentDescriptorSet::start(layout.clone())
            .add_buffer(data_buffer.clone())
            .unwrap()
            .build()
            .unwrap(),
    );

    //dispatch code
    //this looks similar
    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .dispatch([1024, 1, 1], compute_pipeline.clone(), set.clone(), ())
        .unwrap();
    let command_buffer = builder.build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();

    finished
        .then_signal_fence_and_flush()
        .unwrap()
        .wait(None)
        .unwrap();

    let content = data_buffer.read().unwrap();

    for (n, val) in content.iter().enumerate() {
        assert_eq!(*val, n as u32);
        //println!("{:?}", val);
    }

    /*

    //----------------------------------------------------------------
    // image stuff
    //----------------------------------------------------------------
    let image = StorageImage::new(device.clone(), Dimensions::Dim2d { width: 1024, height: 1024 },
                              Format::R8G8B8A8Unorm, Some(queue.family())).unwrap();


    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder.clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap();
    let command_buffer = builder.build().unwrap();


    let buf = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false,
                                         (0 .. 1024 * 1024 * 4).map(|_| 0u8))
                                         .expect("failed to create buffer");



    let mut builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    builder
        .clear_color_image(image.clone(), ClearValue::Float([0.0, 0.0, 1.0, 1.0])).unwrap()
        .copy_image_to_buffer(image.clone(), buf.clone()).unwrap();
    let command_buffer = builder.build().unwrap();


    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
    .wait(None).unwrap();

    let buffer_content = buf.read().unwrap();
    let image = ImageBuffer::<Rgba<u8>, _>::from_raw(1024, 1024, &buffer_content[..]).unwrap();


    image.save("image.png").unwrap();
    */
    println!("Everything succeded!");
}
