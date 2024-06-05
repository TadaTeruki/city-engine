pub mod container;
pub mod geometry;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Stage(usize);

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Group(usize);

impl Stage {
    pub fn new(stage: usize) -> Self {
        Self(stage)
    }

    pub fn as_num(&self) -> usize {
        self.0
    }

    pub fn incremented(self) -> Self {
        Self(self.0 + 1)
    }
}

impl Group {
    pub fn new(group: usize) -> Self {
        Self(group)
    }

    pub fn as_num(&self) -> usize {
        self.0
    }
}
