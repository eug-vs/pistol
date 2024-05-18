#[derive(Debug, Clone)]
pub struct Buffer {
    pub width: f32,
    pub height: f32,
    pub palette: Vec<char>,
}

impl Buffer {
    pub fn from_height(height: f32, aspect_ratio: f32) -> Self {
        Self {
            height,
            width: height * aspect_ratio,
            palette: "$@B%8&WM#oahkbdpqwmZO0QLCJUYXzcvunxrjft/\\|()1{}[]?-_+~<>i!lI;:,\"^`'. "
                .chars()
                .collect(),
        }
    }
}
