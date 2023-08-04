use std::marker::PhantomData;

pub struct Section {}

pub trait CreateDefaultSection<TSection> {
    fn create_default_section(&self) -> TSection;
}

pub struct DefaultSectionCreator<TSection> {
    section_type: PhantomData<TSection>,
}

impl<TSection> DefaultSectionCreator<TSection> {
    pub fn new() -> Self {
        Self {
            section_type: PhantomData,
        }
    }
}

impl<TSection> CreateDefaultSection<TSection> for DefaultSectionCreator<TSection> {
    fn create_default_section(&self) -> TSection {
        todo!()
    }
}
