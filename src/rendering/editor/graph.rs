use std::collections::HashMap;
use std::fmt::Display;
use crate::math;
use crate::events;
use crate::debug;

pub struct EditorRenderGraph {
    controls: Vec<EditorRenderGraphNode>, 
    data: HashMap<EditorRenderGraphDataItem, EditorRenderGraphData>,
    state: debug::Editor
}

impl EditorRenderGraph {
    pub fn new() -> Self {
        Self {
            controls: vec!(),
            data: HashMap::default(),
            state: debug::Editor::default()
        }
    }

    pub fn editor_visible(&self) -> bool {
        self.state.editor_visible
    }

    pub fn set_state(&mut self, state: &debug::Editor) {
        self.state = *state;
    }
    
    pub fn add_control(&mut self, control: EditorRenderGraphNode) {
        self.controls.push(control);
    }

    pub fn add_data(&mut self, item: EditorRenderGraphDataItem, value: EditorRenderGraphData) {
        self.data.insert(item, value);
    }

    pub fn add_float_data(&mut self, item: EditorRenderGraphDataItem, value: f32) {
        self.add_data(item, EditorRenderGraphData::Float { value })
    }

    pub fn add_vector4_data(&mut self, item: EditorRenderGraphDataItem, value: math::Vector4) {
        self.add_data(item, EditorRenderGraphData::Vector4 { value })
    }

    pub fn add_row_data(&mut self, item: EditorRenderGraphDataItem, children: HashMap<EditorRenderGraphDataItem, EditorRenderGraphData> ) {
        if !self.data.contains_key(&item) {
            self.data.insert(item, EditorRenderGraphData::Rows { data: vec!() });
        }
        
        let data_item = self.data.get_mut(&item).unwrap();
        match data_item {
            EditorRenderGraphData::Rows { data } => {
                data.push(children);
            },
            _ => {}
        }
    }

    pub fn controls(&self) -> &Vec<EditorRenderGraphNode> {
        &self.controls
    }

    pub fn data(&self) -> &HashMap<EditorRenderGraphDataItem, EditorRenderGraphData> {
        &self.data
    }

    pub fn clear_data(&mut self) {
        self.data = HashMap::default();
    }
}

pub enum EditorRenderGraphNode {
    Window { name: String, children: Vec<EditorRenderGraphNode> },
    ScrollArea { id: String, children: Vec<EditorRenderGraphNode> },
    Grid { name: String, children: Vec<EditorRenderGraphNode> },
    Row { children: Vec<EditorRenderGraphNode> },
    Rows { children: Vec<EditorRenderGraphNode>, titles: Vec<EditorRenderGraphDataItem>, item: EditorRenderGraphDataItem },
    Label { item: EditorRenderGraphDataItem },
    Text { item: EditorRenderGraphDataItem },
    Numeric { item: EditorRenderGraphDataItem },
    Vector { item: EditorRenderGraphDataItem, change_handler: Box<dyn Fn(math::Vector4) -> events::EditorEvent>}
}

pub enum EditorRenderGraphData {
    Rows { data: Vec<HashMap<EditorRenderGraphDataItem, EditorRenderGraphData>> },
    String { value: String },
    Float { value: f32 },
    Int { value: u64 },
    Vector4 { value: math::Vector4 }
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EditorRenderGraphDataItem {
    MeasurementRow,
    MeasurementName,
    CycleMeasurement,
    CyclePercentage,
    HitMeasurement,
    ElapsedTime,
    CameraPosition,
    CameraDirection,
    CameraUp,
}

impl Display for EditorRenderGraphDataItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorRenderGraphDataItem::CycleMeasurement => f.write_str("Cycles"),
            EditorRenderGraphDataItem::CyclePercentage => f.write_str("Cycles %"),
            EditorRenderGraphDataItem::HitMeasurement => f.write_str("Hits"),
            EditorRenderGraphDataItem::ElapsedTime => f.write_str("Elapsed time (ms)"),
            EditorRenderGraphDataItem::CameraPosition => f.write_str("Position"),
            EditorRenderGraphDataItem::CameraDirection => f.write_str("Direction"),
            EditorRenderGraphDataItem::CameraUp => f.write_str("Up"),
            _ => f.write_str("")
        }
    }
}
