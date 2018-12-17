#[derive(Debug)]
pub struct Data {
    value: Option<u8>,
    size: Option<(u32, u32)>,
    pixels: Vec<u8>
}

impl Data {
    pub fn new() -> Self {
        Self {
            value: None,
            size: None,
            pixels: Vec::new()
        }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.size = Some((width, height));
    }

    pub fn get_size(&self) -> Option<(u32, u32)> {
        self.size
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = Some(value);
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }

    pub fn set_pixels(&mut self, pixels: Vec<u8>) {
        self.pixels = pixels;
    }

    pub fn get_pixels(&self) -> Vec<u8> {
        self.pixels.clone()
    }
}
