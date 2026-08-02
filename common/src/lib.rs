use url::Url;

pub struct NodeId(pub u32);
pub struct FileId(pub u128);

pub struct Node {
    id: NodeId,
    position: Option<(f32, f32)>,
    data: NodeData,
    contents: Vec<NodeId>,
}

pub enum NodeData {
    Text(TextData),
    Embed(Embed),
    Image {
        embed: Embed,
        scale: f32,
    },
    Video {
        embed: Embed,
        scale: f32,
        start_time: Option<f32>,
        end_time: Option<f32>,
        volume: f32,
    },
    Audio {
        embed: Embed,
        start_time: Option<f32>,
        end_time: Option<f32>,
        volume: f32,
    },
    Location(LocationData),
}

pub enum Embed {
    Link(Url),
    File(FileId),
}

pub enum TextData {
    Inline(String),
    Document { file: FileId, preview: bool },
}

pub enum LocationData {
    Address(String),
    Coordinates { latitude: f64, longitude: f64 },
}
