use crate::prelude::*;

#[system(for_each)]
pub fn build_editor_render_graph_for_measurements(
    measurement: &BlockPerformanceMeasurement,
    #[resource] graph: &mut EditorRenderGraph
) {
    let timed_block = start_timed_block(CycleCounter::BuildEditorRenderGraphForMeasurements);

    let mut row = HashMap::default();
    row.insert(EditorRenderGraphDataItem::MeasurementName, EditorRenderGraphData::String { value: measurement.counter.to_string() });
    row.insert(EditorRenderGraphDataItem::CycleMeasurement, EditorRenderGraphData::Int { value: measurement.cycles as i64 });
    row.insert(EditorRenderGraphDataItem::CyclePercentage, EditorRenderGraphData::Float { value: (measurement.cycles as f32 / measurement.total_cycles as f32) * 100.0 });
    row.insert(EditorRenderGraphDataItem::HitMeasurement, EditorRenderGraphData::Int { value: measurement.hits as i64 });
    graph.add_row_data(EditorRenderGraphDataItem::MeasurementRow, row);
    
    timed_block.stop();
}