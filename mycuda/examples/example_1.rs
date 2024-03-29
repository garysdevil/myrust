#[macro_use]
extern crate rustacuda;

use rustacuda::device::DeviceAttribute;
use rustacuda::prelude::*;
use std::error::Error;
use std::ffi::CString;

fn main() -> Result<(), Box<dyn Error>> {
    // Set up the context, load the module, and create a stream to run kernels in.
    // 设置上下文，加载模块，创建和内核交互的数据流
    rustacuda::init(CudaFlags::empty())?;
    println!("显卡数量: {}", Device::num_devices().unwrap());
    for device in Device::devices()? {
        let device = device?;
        let max_threads_per_block = device
            .get_attribute(DeviceAttribute::MaxThreadsPerBlock)
            .unwrap();
        let total_memory = device.total_memory().unwrap() as u64 / 1024 / 1024;
        println!(
            "{:?}, name={:?}, max_threads_per_block={}, total_memory={}",
            &device,
            device.name().unwrap(),
            max_threads_per_block,
            total_memory
        );
    }

    for device in Device::devices()? {
        let device = device?;
        detect(device)?;
    }
    Ok(())
}

fn detect(device: Device) -> Result<(), Box<dyn Error>> {
    let _ctx = Context::create_and_push(ContextFlags::MAP_HOST | ContextFlags::SCHED_AUTO, device)?;

    let ptx = CString::new(include_str!("../resources/add.ptx"))?;
    let module = Module::load_from_string(&ptx)?;
    let stream = Stream::new(StreamFlags::NON_BLOCKING, None)?;

    // Create buffers for data
    // 为数据创建buffers
    let mut in_x = DeviceBuffer::from_slice(&[1.0f32; 10])?;
    let mut in_y = DeviceBuffer::from_slice(&[2.0f32; 10])?;
    let mut out_1 = DeviceBuffer::from_slice(&[0.0f32; 10])?;
    let mut out_2 = DeviceBuffer::from_slice(&[0.0f32; 10])?;

    // This kernel adds each element in `in_x` and `in_y` and writes the result into `out`.
    //
    unsafe {
        // Launch the kernel with one block of one thread, no dynamic shared memory on `stream`.
        let result = launch!(module.sum<<<1, 1, 0, stream>>>(
            in_x.as_device_ptr(),
            in_y.as_device_ptr(),
            out_1.as_device_ptr(),
            out_1.len()
        ));
        result?;

        // Launch the kernel again using the `function` form:
        // 启动方式二 通过函数启动GPU内核
        let function_name = CString::new("sum")?;
        let sum = module.get_function(&function_name)?;
        // Launch with 1x1x1 (1) blocks of 10x1x1 (10) threads, to show that you can use tuples to configure grid and block size.
        let result = launch!(sum<<<(1, 1, 1), (10, 1, 1), 0, stream>>>(
            in_x.as_device_ptr(),
            in_y.as_device_ptr(),
            out_2.as_device_ptr(),
            out_2.len()
        ));
        result?;
    }

    // Kernel launches are asynchronous, so we wait for the kernels to finish executing.
    // 等待GPU内核执行结束
    stream.synchronize()?;

    // Copy the results back to host memory
    // 将GPU计算结果拷贝回主机内存
    let mut out_host = [0.0f32; 20];
    out_1.copy_to(&mut out_host[0..10])?;
    out_2.copy_to(&mut out_host[10..20])?;

    // 判断计算结果是否符合预期
    for x in out_host.iter() {
        assert_eq!(3.0 as u32, *x as u32);
    }

    println!("{:?}, Launched kernel successfully.", device);
    Ok(())
}
