mod application;
mod systems;
mod terrain;
mod graph;

#[no_mangle]
pub extern "C" fn initialise<'a>() -> *mut voxel_engine_app::HotLoadableApplicationState {
    let application_state: voxel_engine_app::HotLoadableApplicationState = application::Application::build().into();
    Box::into_raw(Box::new(application_state))
}

#[no_mangle]
pub unsafe extern "C" fn update(application_state: *mut voxel_engine_app::HotLoadableApplicationState) -> bool {
    if application_state.is_null() {
        panic!("[ FATAL ] app_update: app state is null!");
    }

    let application_state_mut = &mut *application_state;
    application_state_mut.run_loop();
    !application_state_mut.should_exit()
}

#[no_mangle]
pub unsafe extern "C" fn shutdown(application_state: *mut voxel_engine_app::HotLoadableApplicationState) {
    if application_state.is_null() {
        panic!("[ FATAL ] app_update: app state is null!");
    }

    std::ptr::drop_in_place(application_state);
    std::alloc::dealloc(application_state as *mut u8, std::alloc::Layout::new::<voxel_engine_app::HotLoadableApplicationState>());
}

#[no_mangle]
pub unsafe extern "C" fn unload(application_state: *mut voxel_engine_app::HotLoadableApplicationState) {
    println!("unloading app");

    if application_state.is_null() {
        panic!("[ FATAL ] game_update: game state is null!");
    }

    let application_state_mut = &mut *application_state;
    application_state_mut.unload();
}

#[no_mangle]
pub unsafe extern "C" fn reload(application_state: *mut voxel_engine_app::HotLoadableApplicationState) {
    println!("reloading app");

    if application_state.is_null() {
        panic!("[ FATAL ] game_update: game state is null!");
    }
    
    let application_state_mut = &mut *application_state;
    application::Application::reload(application_state_mut);

    println!("reloaded app");
}