use std::ops::Add;

struct CanvasPoint {
    x: u32,
    y: u32
}

impl CanvasPoint {
    pub fn new(x: u32, y: u32) -> Self {
        CanvasPoint { x, y }
    }
}

impl Add for CanvasPoint {
    type Output = CanvasPoint;

    fn add(self, rhs: CanvasPoint) -> Self::Output {
        CanvasPoint::new(self.x + rhs.x, self.y + rhs.y)
    }
}

struct Line {
    first: CanvasPoint,
    second: CanvasPoint
}

struct Canvas {
    width: u32,
    height: u32,
    text: String
}

impl Canvas {
    fn put_underline(&mut self, start: CanvasPoint, length: usize) {

    }

    fn put_text(&mut self, start: CanvasPoint, text: String) {

    }

    fn text(mut self) -> String {
        unimplemented!()
    }
}


//struct
