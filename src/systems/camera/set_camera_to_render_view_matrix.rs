use crate::prelude::*;

#[system(for_each)]
pub fn set_camera_to_render_view_matrix(
    camera: &Camera, 
    #[resource] graph: &mut WorldRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::SetCameraToRenderViewMatrix);

    graph.view = Matrix4x4::view(
        camera.position, 
        camera.direction, 
        camera.up
    );

    timed_block.stop();
}