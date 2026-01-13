#[derive(Debug, Clone, Copy, PartialOrd, PartialEq)]
pub struct DeltaTime {
    second: f32,
}

impl DeltaTime {
    pub fn new(delta: f32) -> DeltaTime {
        DeltaTime { second: delta }
    }

    pub const fn new_const(delta: f32) -> DeltaTime {
        DeltaTime { second: delta }
    }

    pub fn as_seconds(&self) -> f32 {
        self.second.max(0.0)
    }

    pub fn to_milliseconds(&self) -> f32 {
        self.second * 1000.0
    }

    pub fn to_microseconds(&self) -> f32 {
        self.second * 1_000_000.0
    }
}