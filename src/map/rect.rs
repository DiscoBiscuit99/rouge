pub struct Rect {
    pub x: i16,
    pub y: i16,
    pub w: i16,
    pub h: i16,
}

impl Rect {
    // TODO: new, intersects(this, other), center
    pub fn new(x: i16, y: i16, w: i16, h: i16) -> Rect {
        Rect { x, y, w, h }
    }

    pub fn intersects(&self, other: &Rect) -> bool {
        self.x + self.w >= other.x && self.x <= other.x + other.w &&
        self.y + self.h >= other.y && self.y <= other.y + other.h
    }

    pub fn center(&self) -> (i16, i16) {
        (self.x + self.w / 2, self.y + self.h / 2)
    }
}

