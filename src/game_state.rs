#[derive(Clone, Copy, PartialEq, Eq)]
pub enum GameState {
    Playing,
    Editing
}

pub fn create_exit_state_notifier() -> ExitStateNotifier {
    ExitStateNotifier::default()
}

#[derive(Default)]
pub struct ExitStateNotifier {
    pub should_exit: bool
}

impl ExitStateNotifier {
    pub fn should_exit(resources: &mut legion::Resources) -> bool {
        let notifier = resources
            .get::<ExitStateNotifier>()
            .unwrap();
    
        notifier.should_exit
    }
}