use image::GenericImageView;

pub struct Texture {
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
}

impl Texture {
    pub fn load(path: &str) -> Self {
        let img = image::open(path)
            .unwrap()
            .to_rgba8();

        let (w, h) = img.dimensions();

        Self {
            width: w as usize,
            height: h as usize,
            data: img.into_raw(),
        }
    }

    pub fn sample(
        &self,
        x: usize,
        y: usize,
    ) -> [u8; 4] {
        let tx =
            x % self.width;

        let ty =
            y % self.height;

        let idx =
            (ty * self.width + tx) * 4;

        [
            self.data[idx],
            self.data[idx + 1],
            self.data[idx + 2],
            self.data[idx + 3],
        ]
    }
}