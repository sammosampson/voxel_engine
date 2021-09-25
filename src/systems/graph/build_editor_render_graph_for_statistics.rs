
use crate::prelude::*;

#[system(for_each)]
pub fn build_editor_render_graph_for_statistics(time: &ElapsedTime, #[resource] graph: &mut EditorRenderGraph) {
    let timed_block = start_timed_block(CycleCounter::BuildEditorRenderGraphForStatistics);
    graph.add_float_data(EditorRenderGraphDataItem::ElapsedTime, time.seconds * 1000.0);    
    timed_block.stop();
}