use std::rc::Rc;

use garden_sections::{DefaultSectionCreator, Section};

pub struct SectionsComponent {
    default_section_creator: Rc<DefaultSectionCreator<Section>>,
}

impl SectionsComponent {
    fn new(default_section_creator: Rc<DefaultSectionCreator<Section>>) -> Self {
        Self {
            default_section_creator,
        }
    }

    pub fn get_default_section_creator(&self) -> Rc<DefaultSectionCreator<Section>> {
        Rc::clone(&self.default_section_creator)
    }
}

pub fn compose_component() -> SectionsComponent {
    let default_section_creator = Rc::new(DefaultSectionCreator::new());

    let sections_component = SectionsComponent::new(default_section_creator);

    sections_component
}
