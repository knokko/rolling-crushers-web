use std::cell::RefCell;
use std::rc::Rc;

use crate::get_default_font;
use crate::levels::*;
use crate::storage::get_level_pack_names;

use super::level::create_level_edit;

use super::*;

use wasmuri_components::button::text::TextButton;
use wasmuri_components::passive::PassiveText;
use wasmuri_components::input::text::TextEditField;
use wasmuri_components::behavior::render::*;
use wasmuri_container::*;
use wasmuri_container::layer::Layer;
use wasmuri_core::util::*;

pub fn create_level_pack_creation() -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    let info_component_text = SimpleTextRenderController::celled("Creating a new level pack...", font, label_location(Region::new(-5000, 8000, 5000, 10_000), TextAlignment::Center), LABEL_COLORS);
    let info_component_ref = Rc::clone(&info_component_text);
    layer.add_component(PassiveText::celled((Rc::clone(&info_component_text) as Rc<RefCell<dyn ComponentBehavior>>, info_component_text)));

    layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple("Cancel", font, button_location(Region::new(-9000, 5500, -3000, 7500)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            agent.change_container(create_level_pack_overview());
    })));

    layer.add_component(PassiveText::celled(SimpleTextRenderController::tuple("Name:", font, label_location(Region::new(-8000, 0, -4100, 2000), TextAlignment::RightCenter), LABEL_COLORS)));
    
    let name_field = TextEditField::celled(EditTextRenderController::simple_tuple("", font, edit_location(Region::new(-4000, 0, 4000, 2000)), EDIT_COLORS));
    let name_field_ref = Rc::clone(&name_field);
    layer.add_component(name_field);

    layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple("Create", font, button_location(Region::new(-3000, -8000, 3000, -6000)), BUTTON_COLORS),
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
            let new_pack = LevelPackBuilder::new_empty(&chosen_name);
            agent.change_container(create_level_pack_edit(new_pack));
    })));

    FlatContainer::celled(layer)
}

pub fn create_level_pack_edit(pack: LevelPackBuilder) -> Rc<RefCell<dyn Container>>{
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(PassiveText::celled(SimpleTextRenderController::tuple(&pack.name, font, label_location(Region::new(-1500, 7000, 6000, 9000), TextAlignment::Center), LABEL_COLORS)));

    layer.add_component(TextButton::celled(
        ButtonTextRenderController::simple_tuple("Save and quit", font, button_location(Region::new(-10_000, 7000, -2000, 9000)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            // TODO Save
            agent.change_container(create_level_pack_overview());
    })));
    layer.add_component(TextButton::celled(
        ButtonTextRenderController::simple_tuple("Quit without saving", font, button_location(Region::new(-10_000, 4500, -2000, 6500)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            agent.change_container(create_level_pack_overview());
    })));
    layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple("Create new level", font, button_location(Region::new(-10_000, -8000, -2000, -6000)), BUTTON_COLORS), 
        Box::new(|agent, _, _| {
            agent.change_container(create_level_edit(LevelBuilder::new_empty()));
    })));

    // TODO Maybe change type of Region properties from i16 to i32
    let mut base_y = 4500;
    for level in &pack.levels {
        let capture_level = level.clone();
        layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple(level.get_name(), font, left_button_location(Region::new(1000, base_y, 8000, base_y + 2000)), BUTTON_COLORS), 
            Box::new(move |agent, _, _| {
                agent.change_container(create_level_edit(LevelBuilder::from_level(capture_level.clone())));
        })));
        base_y -= 2500;
    }

    FlatContainer::celled(layer)
}