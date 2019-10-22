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
use wasmuri_components::helper::render::text::*;
use wasmuri_container::{
    Container,
    FlatContainer,
    layer::Layer
};
use wasmuri_core::util::*;

pub fn create_level_pack_creation() -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(TextButton::celled(ButtonTextRenderHelper::simple_celled("Cancel", font, button_location(Region::new(-0.9, 0.55, -0.3, 0.75)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            agent.change_container(create_level_pack_overview());
    })));

    let info_component_text = SimpleTextRenderHelper::celled("Creating a new level pack...", font, label_location(Region::new(-0.5, 0.8, 0.5, 1.0), TextAlignment::Center), LABEL_COLORS);
    let info_component_ref = Rc::clone(&info_component_text);
    layer.add_component(PassiveText::celled(info_component_text));

    layer.add_component(PassiveText::celled(SimpleTextRenderHelper::celled("Name: ", font, label_location(Region::new(-0.8, 0.0, -0.4, 0.2), TextAlignment::RightCenter), LABEL_COLORS)));
    
    let name_field = Rc::new(RefCell::new(TextEditField::new("".to_string(), EditTextRenderHelper::simple_celled("", font, edit_location(Region::new(-0.4, 0.0, 0.4, 0.2)), EDIT_COLORS))));
    let name_field_ref = Rc::clone(&name_field);
    layer.add_component(name_field);

    layer.add_component(TextButton::celled(ButtonTextRenderHelper::simple_celled("Create", font, button_location(Region::new(-0.3, -0.8, 0.3, -0.6)), BUTTON_COLORS),
        Box::new(move |agent, _, _| {
            let existing_names = get_level_pack_names();
            let name_field_borrow = name_field_ref.borrow();
            let chosen_name = name_field_borrow.get_current_text();
            for existing in existing_names {
                if existing == chosen_name {
                    let mut info_component_borrow = info_component_ref.borrow_mut();
                    info_component_borrow.set_text("A level pack with that name already exists");
                    return;
                }
            }
            let new_pack = LevelPackBuilder::new_empty(chosen_name);
            agent.change_container(create_level_pack_edit(new_pack));
    })));

    FlatContainer::celled(layer)
}

pub fn create_level_pack_edit(pack: LevelPackBuilder) -> Rc<RefCell<dyn Container>>{
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(TextButton::celled(
        ButtonTextRenderHelper::simple_celled("Save and quit", font, button_location(Region::new(-1.0, 0.7, -0.2, 0.9)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            // TODO Save
            agent.change_container(create_level_pack_overview());
    })));
    layer.add_component(TextButton::celled(
        ButtonTextRenderHelper::simple_celled("Quit without saving", font, button_location(Region::new(-1.0, 0.45, -0.2, 0.65)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            agent.change_container(create_level_pack_overview());
    })));

    layer.add_component(PassiveText::celled(SimpleTextRenderHelper::celled(&pack.name, font, label_location(Region::new(-0.2, 0.7, 0.6, 0.9), TextAlignment::Center), LABEL_COLORS)));

    for level in &pack.levels {
        layer.add_component(TextButton::celled(ButtonTextRenderHelper::simple_celled(level.get_name(), font, button_location(Region::new(-1.0, -0.8, -0.2, -0.6)), BUTTON_COLORS), 
        Box::new(|_agent, _, _| {
            // TODO Edit the level
    })));
    }

    layer.add_component(TextButton::celled(ButtonTextRenderHelper::simple_celled("Create new level", font, button_location(Region::new(-1.0, -0.8, -0.2, -0.6)), BUTTON_COLORS), 
        Box::new(|_agent, _, _| {
            // TODO Create a new level...
    })));

    FlatContainer::celled(layer)
}