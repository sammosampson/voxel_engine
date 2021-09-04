use crate::debug;
use crate::rendering;
use super::wrapped;
use super::channel;
use glium::glutin::platform::run_return::EventLoopExtRunReturn;

pub fn create_system_event_loop() -> SystemEventLoop {
    SystemEventLoop {
        inner: glium::glutin::event_loop::EventLoop::new()
    }
}

pub struct SystemEventLoop {
    inner: glium::glutin::event_loop::EventLoop<()>
}

impl SystemEventLoop {
    pub fn get_loop(&self) -> &glium::glutin::event_loop::EventLoop<()> {
        &self.inner
    }

    pub fn run(
        &mut self,
        event_producer: &mut channel::SystemEventProducer,
        event_channel: &mut shrev::EventChannel::<super::super::SystemEvent>,
        renderer: &mut rendering::ScreenRenderer
    ) {
        let timed_block = debug::TimedBlock::start(debug::CycleCounter::EventLoop);
    
        self.inner.run_return(|event, _, flow| {
            match event {
                glium::glutin::event::Event::WindowEvent { window_id: _, event} => {
                    renderer.process_event(&event);
                    event_producer.push(wrapped::WrappedSystemEvent::from(event).into());
                }
                glium::glutin::event::Event::DeviceEvent { device_id: _, event } => {
                    event_producer.push(wrapped::WrappedSystemEvent::from(event).into());
                }
                _ => {}
            }
            *flow = glium::glutin::event_loop::ControlFlow::Exit;
        });
    
        event_producer.drain_to(event_channel);
        
        timed_block.stop();
    }
}
