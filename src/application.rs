use crate::prelude::*;

pub struct Application {
    pub world: World, 
    pub resources: Resources,
    schedule: Schedule,
    event_loop: SystemEventLoop,
    screen_renderer: ScreenRenderer
}

impl Application {
    pub fn build() -> Self {
        initialise_debugging();
        let event_loop = create_system_event_loop();
        let screen_renderer = ScreenRenderer::new(&event_loop.get_loop());

        let world = build_world();
        let resources = build_resources();
        let schedule = build_schedule();
       
        Self {
            world,
            resources, 
            schedule,
            event_loop,
            screen_renderer
        }
    }

    pub fn run(&mut self) {
        loop {
            if self.should_exit() {
                return;
            }
    
            self.run_loop();
        }
    }

    fn run_loop(&mut self) {
        self.process_events();
        self.execute_schedule();        
        self.render();
    }

    fn process_events(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<SystemEventProducer>().unwrap();
        let mut event_channel = &mut self.resources.get_mut::<shrev::EventChannel::<SystemEvent>>().unwrap();
        self.event_loop.run(&mut event_producer, &mut event_channel, &mut self.screen_renderer);
    }

    fn execute_schedule(&mut self) {
        &mut self.schedule.execute(&mut self.world, &mut self.resources);
    }

    fn render(&mut self) {
        let mut event_producer = &mut self.resources.get_mut::<SystemEventProducer>().unwrap();
        let mut world_graph = &mut self.resources.get_mut::<WorldRenderGraph>().unwrap();
        let mut editor_graph = &mut self.resources.get_mut::<EditorRenderGraph>().unwrap();
        self.screen_renderer.render(&mut editor_graph, &mut world_graph, &mut event_producer);
    }
    
    pub fn should_exit(&mut self) -> bool {
        ExitStateNotifier::should_exit(&mut self.resources)
    }
}

fn build_resources() -> Resources {
    let exit_state_notifier = create_exit_state_notifier();
    let system_event_producer = create_system_event_producer();
    let mut system_event_channel = create_system_event_channel();
    let event_channel_registrar = create_system_event_channel_registrar(&mut system_event_channel);
    let editor_render_graph = EditorRenderGraph::new();
    let world_render_graph = WorldRenderGraph::new();
        
    let mut resources = Resources::default();
    &mut resources.insert(exit_state_notifier);
    &mut resources.insert(system_event_producer);
    &mut resources.insert(system_event_channel);
    &mut resources.insert(event_channel_registrar);
    &mut resources.insert(editor_render_graph);
    &mut resources.insert(world_render_graph);
    resources
}