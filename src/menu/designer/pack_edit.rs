use std::cell::RefCell;
use std::rc::Rc;

use crate::get_default_font;
use crate::levels::LevelPackBuilder;
use crate::storage::get_level_pack_names;

use super::{
    BACKGROUND_COLOR,
    BUTTON_COLORS,
    LABEL_COLORS,
    EDIT_COLORS,
    create_level_pack_overview
};

use wasmuri_components::button::text::TextButton;
use wasmuri_components::passive::PassiveText;
use wasmuri_components::input::text::TextEditField;
use wasmuri_components::helper::render::text::{
    ButtonTextRenderHelper,
    SimpleTextRenderHelper
};
use wasmuri_container::{
    Container,
    FlatContainer,
    layer::Layer,
    layer::Region
};

pub fn create_level_pack_creation() -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(TextButton::boxed(ButtonTextRenderHelper::simple_boxed("Cancel", font, Region::new(-0.9, 0.55, -0.3, 0.75), BUTTON_COLORS),
        Box::new(|_, params| {
            params.agent.change_container(create_level_pack_overview());
    })));

    layer.add_component(PassiveText::boxed(SimpleTextRenderHelper::boxed("Creating a new level pack...", font, Region::new(-0.5, 0.8, 0.5, 1.0), LABEL_COLORS)));

    layer.add_component(PassiveText::boxed(SimpleTextRenderHelper::boxed("Name: ", font, Region::new(-0.8, 0.0, -0.4, 0.2), LABEL_COLORS)));
    layer.add_component(Box::new(TextEditField::new("Edit".to_string(), font, SimpleTextRenderHelper::boxed("Edit", font, Region::new(-0.4, 0.0, 0.4, 0.2), EDIT_COLORS))));

    layer.add_component(TextButton::boxed(ButtonTextRenderHelper::simple_boxed("Create", font, Region::new(-0.3, -0.8, 0.3, -0.6), BUTTON_COLORS),
        Box::new(|_, params| {
            let existing_names = get_level_pack_names();
            // TODO Read the name from the name text field, compare it with the existing names and then go ahead if the name wasn't in use
    })));

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}

pub fn create_level_pack_edit(pack: LevelPackBuilder) -> Rc<RefCell<dyn Container>>{
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(TextButton::boxed(
        ButtonTextRenderHelper::simple_boxed("Save and quit", font, Region::new(-1.0, 0.7, -0.2, 0.9), BUTTON_COLORS),
        Box::new(|_a, params| {
            // TODO Save
            params.agent.change_container(create_level_pack_overview());
    })));
    layer.add_component(TextButton::boxed(
        ButtonTextRenderHelper::simple_boxed("Quit without saving", font, Region::new(-1.0, 0.45, -0.2, 0.65), BUTTON_COLORS),
        Box::new(|_a, params| {
            params.agent.change_container(create_level_pack_overview());
    })));

    layer.add_component(PassiveText::boxed(SimpleTextRenderHelper::boxed(&pack.name, font, Region::new(-0.2, 0.7, 0.6, 0.9), LABEL_COLORS)));

    for level in &pack.levels {
        layer.add_component(TextButton::boxed(ButtonTextRenderHelper::simple_boxed(level.get_name(), font, Region::new(-1.0, -0.8, -0.2, -0.6), BUTTON_COLORS), 
        Box::new(|_, _params| {
            // TODO Edit the level
    })));
    }

    layer.add_component(TextButton::boxed(ButtonTextRenderHelper::simple_boxed("Create new level", font, Region::new(-1.0, -0.8, -0.2, -0.6), BUTTON_COLORS), 
        Box::new(|_, _params| {
            // TODO Create a new level...
    })));

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}