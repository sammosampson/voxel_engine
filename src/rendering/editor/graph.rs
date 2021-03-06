use std::collections::HashMap;
use std::fmt::Display;
use crate::math;
use crate::events;
use crate::debug;

pub struct EditorRenderGraph {
    controls: Vec<EditorRenderGraphNode>, 
    data: HashMap<EditorRenderGraphDataItem, EditorRenderGraphData>,
    state: debug::EditorState,
    editor_visible: bool
}

impl EditorRenderGraph {
    pub fn new() -> Self {
        Self {
            controls: vec!(),
            data: HashMap::default(),
            state: debug::EditorState::default(),
            editor_visible: false
        }
    }

    pub fn editor_visible(&self) -> bool {
        self.editor_visible
    }

    pub fn set_state(&mut self, state: &debug::EditorState, visible: bool) {
        self.state = state.clone();
        self.editor_visible = visible;
    }
    
    pub fn add_control(&mut self, control: EditorRenderGraphNode) {
        self.controls.push(control);
    }

    pub fn add_data(&mut self, item: EditorRenderGraphDataItem, value: EditorRenderGraphData) {
        self.data.insert(item, value);
    }

    pub fn add_list_item(&mut self, list: EditorRenderGraphDataItem, value: String, selected: bool) {
        if !self.data.contains_key(&list) {
            self.data.insert(list, EditorRenderGraphData::Nodes { data: vec!() });
        }
        let nodes = self.data.get_mut(&list).unwrap();
        match nodes {
            EditorRenderGraphData::Nodes { data } => {
                data.push((value, selected));
            },
            _ => {}
        }
    }

    pub fn add_float_data(&mut self, item: EditorRenderGraphDataItem, value: f32) {
        self.add_data(item, EditorRenderGraphData::Float { value })
    }

    pub fn add_boolean_data(&mut self, item: EditorRenderGraphDataItem, value: bool) {
        self.add_data(item, EditorRenderGraphData::Boolean { value })
    }

    pub fn add_vector4_data(&mut self, item: EditorRenderGraphDataItem, value: math::Vector4) {
        self.add_data(item, EditorRenderGraphData::Vector4 { value })
    }

    pub fn add_row_data(&mut self, item: EditorRenderGraphDataItem, children: HashMap<EditorRenderGraphDataItem, EditorRenderGraphData> ) {
        if !self.data.contains_key(&item) {
            self.data.insert(item, EditorRenderGraphData::Cells { data: vec!() });
        }
        
        let data_item = self.data.get_mut(&item).unwrap();
        match data_item {
            EditorRenderGraphData::Cells { data } => {
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

    pub fn is_window_visible(&self, window_name: &str) -> bool {
        self.state.is_window_visible(window_name)
    }

    pub fn clear_data(&mut self) {
        self.data = HashMap::default();
    }
}

pub enum EditorRenderGraphNode {
    SideBar { name: String, children: Vec<EditorRenderGraphNode> },
    Window { name: String, children: Vec<EditorRenderGraphNode> },
    ScrollArea { id: String, children: Vec<EditorRenderGraphNode> },
    Grid { name: String, children: Vec<EditorRenderGraphNode> },
    Rows { children: Vec<EditorRenderGraphNode>, titles: Vec<EditorRenderGraphDataItem>, item: EditorRenderGraphDataItem },
    Row { children: Vec<EditorRenderGraphNode> },
    Tree { item: EditorRenderGraphDataItem, item_click_handler: Box<dyn Fn(String) -> events::EditorEvent>  },
    Toggle { item: EditorRenderGraphDataItem, click_handler: Box<dyn Fn(bool) -> events::EditorEvent> },
    Label { item: EditorRenderGraphDataItem },
    Text { item: EditorRenderGraphDataItem },
    Numeric { item: EditorRenderGraphDataItem },
    Vector { item: EditorRenderGraphDataItem, change_handler: Box<dyn Fn(math::Vector4) -> events::EditorEvent>}
}

pub enum EditorRenderGraphData {
    Cells { data: Vec<HashMap<EditorRenderGraphDataItem, EditorRenderGraphData>> },
    Nodes { data: Vec<(String, bool)> },
    String { value: String },
    Boolean { value: bool },
    Float { value: f32 },
    Int { value: i64 },
    Vector4 { value: math::Vector4 },
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum EditorRenderGraphDataItem {
    MeasurementWindowVisibiity,
    MeasurementRow,
    MeasurementName,
    CycleMeasurement,
    CyclePercentage,
    HitMeasurement,
    ElapsedTime,
    CameraWindowVisibiity,
    CameraPosition,
    CameraDirection,
    CameraUp,
    EntityWindowVisibiity,
    EntityNode,
    ChunksWindowVisibiity,
    ChunkRow,
    ChunkX,
    ChunkZ
}

impl Display for EditorRenderGraphDataItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            EditorRenderGraphDataItem::MeasurementWindowVisibiity => f.write_str("Measurements"),
            EditorRenderGraphDataItem::CameraWindowVisibiity => f.write_str("Camera"),
            EditorRenderGraphDataItem::EntityWindowVisibiity => f.write_str("Entities"),
            EditorRenderGraphDataItem::ChunksWindowVisibiity => f.write_str("Chunks"),
            EditorRenderGraphDataItem::CycleMeasurement => f.write_str("Cycles"),
            EditorRenderGraphDataItem::CyclePercentage => f.write_str("Cycles %"),
            EditorRenderGraphDataItem::HitMeasurement => f.write_str("Hits"),
            EditorRenderGraphDataItem::ElapsedTime => f.write_str("Elapsed time (ms)"),
            EditorRenderGraphDataItem::CameraPosition => f.write_str("Position"),
            EditorRenderGraphDataItem::CameraDirection => f.write_str("Direction"),
            EditorRenderGraphDataItem::CameraUp => f.write_str("Up"),
            EditorRenderGraphDataItem::ChunkX => f.write_str("Chunk X"),
            EditorRenderGraphDataItem::ChunkZ => f.write_str("Chunk Y"),
            _ => f.write_str("")
        }
    }
}
