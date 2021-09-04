pub mod rendering;
pub mod events;
pub mod debug;
pub mod math;
pub mod time;
pub mod mesh;
pub mod input;
pub mod terrain;
pub mod cameras;
pub mod lighting;
pub mod position;
pub mod rotation;

use legion::World;

pub struct HotLoadableApplicationState {
    world: legion::World, 
    resources: legion::Resources,
    schedule: legion::Schedule,
    event_loop: events::SystemEventLoop,
    screen_renderer: rendering::ScreenRenderer
}

impl HotLoadableApplicationState {
    pub fn new(
        world: legion::World, 
        resources: legion::Resources,
        schedule: legion::Schedule
    ) -> Self {
        debug::initialise_debugging();
        let event_loop = events::create_system_event_loop();
        let screen_renderer = rendering::ScreenRenderer::new(&event_loop.get_loop());
        
        Self {
            world, 
            resources,
            schedule,
            event_loop,
            screen_renderer
        }
    }

    pub fn world(&mut self) -> &mut World {
        &mut self.world
    }

    pub fn reload(&mut self, resources: legion::Resources, schedule: legion::Schedule) {
        //self.resources = resources;
        //self.schedule = schedule;
    }

    pub fn unload(&self) {
    }
            
    pub fn run_loop(&mut self) {
        self.process_events();
        self.execute_schedule();        
        self.render();
    }

    fn process_events(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<events::SystemEventProducer>().unwrap();
        let mut event_channel = &mut self.resources.get_mut::<shrev::EventChannel::<events::SystemEvent>>().unwrap();
        self.event_loop.run(&mut event_producer, &mut event_channel, &mut self.screen_renderer);
    }

    fn execute_schedule(&mut self) {
        &mut self.schedule.execute(&mut self.world, &mut self.resources);
    }

    fn render(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<events::SystemEventProducer>().unwrap();
        let mut world_graph = &mut self.resources.get_mut::<rendering::WorldRenderGraph>().unwrap();
        let mut editor_graph = &mut self.resources.get_mut::<rendering::EditorRenderGraph>().unwrap();
        self.screen_renderer.render(&mut editor_graph, &mut world_graph, &mut event_producer);
    }
    
    pub fn should_exit(&mut self) -> bool {
        ExitStateNotifier::should_exit(&mut self.resources)
    }
}

pub fn create_exit_state_notifier() -> ExitStateNotifier {
    ExitStateNotifier::default()
}

#[derive(Default)]
pub struct ExitStateNotifier {
    pub should_exit: bool
}

impl ExitStateNotifier {
    fn should_exit(resources: &mut legion::Resources) -> bool {
        let notifier = resources
            .get::<ExitStateNotifier>()
            .unwrap();
    
        notifier.should_exit
    }
}




