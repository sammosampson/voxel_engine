use std::{fmt::Display, ops::Add};

use shrev;
use super::clock;

pub fn initialise() {
    unsafe {
        CYCLE_MEASUREMENTS.initialise();   
    }
}

macro_rules! replace_expression {
    ($_to_substitute:tt $substitute:expr) => {
        $substitute
    };
}

macro_rules! count {
    ($($argument:tt)*) => {
        0 $(+ replace_expression!($argument 1))*
    };
}

macro_rules! post_increment {
    ($i:ident) => { 
        { 
            let old = $i; 
            $i += 1; 
            old
        }
    };
}

macro_rules! measurement_channel_readers_lookup {
    ($($reader:ident),*) => {
        paste::item! {
            pub enum CycleCounterMeasurementsChannelReaders {
                $(
                $reader,
                )*
            }
            
            pub struct CycleCounterMeasurementsChannelReadersLookup {
                readers: [shrev::ReaderId<CycleMeasurement>; count!($($reader)*)],
                $(
                [<$reader:snake>]: usize,
                )*
            }
            
            impl CycleCounterMeasurementsChannelReadersLookup {
                pub fn new(channel: &mut shrev::EventChannel::<CycleMeasurement>) -> Self {
                    let mut i = 0usize;
                    Self {
                        readers: [$(replace_expression!($reader channel.register_reader()), )*],
                        $(
                        [<$reader:snake>]: post_increment!(i),                        
                        )*
                    }
                }
           
                pub fn get(&mut self, reader: CycleCounterMeasurementsChannelReaders) -> &mut shrev::ReaderId<CycleMeasurement> {
                    match reader {
                        $(
                        CycleCounterMeasurementsChannelReaders::$reader => &mut self.readers[self.[<$reader:snake>]],                        
                        )*
                    }
                }
            }
        }
    }
}

macro_rules! cycle_counter {
    ($($counter:ident),*) => {
        paste::item! {
            #[derive(Debug, Copy, Clone, Hash, PartialEq, Eq, PartialOrd, Ord)]
            pub enum CycleCounter {
                $(
                $counter,
                )*
            }

            impl CycleCounter {
                pub fn get_all() -> Vec<CycleCounter> {
                    let mut all = vec!();
                    $(
                    all.push(CycleCounter::$counter);
                    )*
                    all
                }
            }

            impl Display for CycleCounter {
                fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                    match self {
                        $(
                        CycleCounter::$counter => f.write_str(stringify!($counter)),
                        )*
                    }
                }
            }
        }
    }
}

cycle_counter! {
    EventLoop,
    MouseInput,
    EditorStateFromInput,
    EditorVisibilityFromInput,
    SetEditorStateOnRenderer,
    StoreWindowInformation,
    ConstantRotation,
    PositionChunks,
    TesselateChunkFrontMesh,
    TesselateChunkBackMesh,
    TesselateChunkTopMesh,
    TesselateChunkLeftMesh,
    TesselateChunkRightMesh,
    RevealTerrain,
    MergeChunkMesh,
    SetWorldNodeOrientation,
    BuildWorldGraphForMesh,
    BuildEditorRenderGraphForStatistics,
    BuildEditorRenderGraphForMeasurements,
    BuildEditorRenderGraphForWorldEntities,
    BuildEditorRenderGraphForEditorState,
    BuildEditorRenderGraphForCamera,
    BuildEditorRenderGraphForLog,
    BuildEditorRenderGraphForChunks,
    SetRenderLighting,
    Render,
    RenderStart,
    RenderEditor,
    RenderWorld,
    RenderEnd,
    FollowWithAttachedCamera,
    SetCameraToRenderViewMatrix,
    MoveCameraFromEditor,
    MoveEditorCameraFromMouseInput,
    MoveHeroFromMouseInput,
    ApplyHeadingAndVelocityToPosition,
    AggregateStatistics,
    AggregateWorldEvents,
    WorldEntitySelectionFromInput
}

measurement_channel_readers_lookup! {
    AggregateTotal,
    Aggregate
}

static mut CYCLE_MEASUREMENTS: CycleCounterMeasurementsChannel = CycleCounterMeasurementsChannel {
    inner: None,
    readers_lookup: None
};

pub fn publish_measurement(counter: CycleCounter, cycles: u64) {
    unsafe {
        CYCLE_MEASUREMENTS.publish_measurement(counter, cycles);   
    }
}

pub fn read_statistics<'a>(reader: CycleCounterMeasurementsChannelReaders) -> shrev::EventIterator<'a, CycleMeasurement> {
    unsafe {
        CYCLE_MEASUREMENTS.read_statistics(reader)
    }
}

pub struct CycleCounterMeasurementsChannel {
    inner: Option<shrev::EventChannel::<CycleMeasurement>>,
    readers_lookup: Option<CycleCounterMeasurementsChannelReadersLookup>
}

impl CycleCounterMeasurementsChannel {
    pub fn initialise(&mut self) {
        self.inner = Some(shrev::EventChannel::<CycleMeasurement>::new());
        self.readers_lookup = Some(CycleCounterMeasurementsChannelReadersLookup::new(self.inner.as_mut().unwrap()));
    }

    pub fn publish_measurement(&mut self, counter: CycleCounter, cycles: u64) {
        let measurement = CycleMeasurement {
            counter,
            cycles 
        };
        self.inner.as_mut().unwrap().single_write(measurement);
    }

    pub fn read_statistics(&mut self, reader: CycleCounterMeasurementsChannelReaders) -> shrev::EventIterator<CycleMeasurement> {
        self.inner.as_ref().unwrap().read(self.readers_lookup.as_mut().unwrap().get(reader))
    }
}

pub struct TimedBlock {
    counter: CycleCounter,
    start: clock::Start
}

impl TimedBlock {
    pub fn start(counter: CycleCounter) -> Self {
        Self {
            counter,
            start: clock::Start::now()
        }
    }

    pub fn stop(self) {
        let end = clock::Stop::now();
        let duration = end - self.start;
        publish_measurement(self.counter, duration.cycles());   
    }
}

#[derive(Debug)]
pub struct CycleMeasurement {
    pub counter: CycleCounter,
    pub cycles: u64
}

#[derive(Debug)]
pub struct BlockPerformanceMeasurement {
    pub counter: CycleCounter,
    pub cycles: u64,
    pub total_cycles: u64,
    pub hits: u64
}

impl From<(&CycleMeasurement, u64)> for BlockPerformanceMeasurement {
    fn from(measure_and_total_cycles: (&CycleMeasurement, u64)) -> Self {
        Self { 
            counter: measure_and_total_cycles.0.counter,
            cycles: measure_and_total_cycles.0.cycles,
            total_cycles: measure_and_total_cycles.1,
            hits: 1
        }
    }
}

impl From<CycleCounter> for BlockPerformanceMeasurement {
    fn from(counter: CycleCounter) -> Self {
        Self { 
            counter,
            cycles: 0,
            total_cycles: 0,
            hits: 0
        }
    }
}

impl Add<&CycleMeasurement> for BlockPerformanceMeasurement {
    type Output = BlockPerformanceMeasurement;

    fn add(self, rhs: &CycleMeasurement) -> Self::Output {
        Self {
            counter: self.counter,
            total_cycles: self.total_cycles,
            cycles: self.cycles + rhs.cycles,
            hits: self.hits + 1
        }
    }
}