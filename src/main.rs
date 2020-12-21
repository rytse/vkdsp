use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;
use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;
use vulkano::command_buffer::CommandBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;
use vulkano::sync::GpuFuture;

fn main() {
    // Get instance, device, and queues
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("failed to create instance");
    let physical = PhysicalDevice::enumerate(&instance).next().expect("no device available");
    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("couldn't find a graphical queue family");
    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                    [(queue_family, 0.5)].iter().cloned()).expect("failed to create device")

    };
    let queue = queues.next().unwrap();

    // Create data buffers
    let from_data = 0 .. 64;
    let to_data = (0 .. 64).map(|_| 0u8);
    let from_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, from_data).expect("failed to create buffer :(");
    let to_buffer = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), false, to_data).expect("failed to create buffer :(");

    // Create command buffer
    let mut command_builder = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap();
    command_builder.copy_buffer(from_buffer.clone(), to_buffer.clone()).unwrap();
    let command_buffer = command_builder.build().unwrap();

    // Execute command, wait until finished
    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap().wait(None).unwrap(); 
    
    // Read the two buffers after the operation, confirm that they are equal
    let from_content = from_buffer.read().unwrap();
    let to_content = to_buffer.read().unwrap();
    assert_eq!(&*to_content, &*from_content);
}
