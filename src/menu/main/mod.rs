use crate::get_default_font;
use crate::storage::{
    get_level_pack_names,
    get_pack,
    get_progress
};
use crate::levels::LevelPack;
use crate::menu::game::create_level_select;
use crate::menu::designer::create_designer_menu;

use std::rc::Rc;
use std::cell::RefCell;

use wasmuri_components::button::text::TextButton;
use wasmuri_components::behavior::render::*;
use wasmuri_container::{
    Container,
    FlatContainer,
};
use wasmuri_container::layer::Layer;

use wasmuri_core::color::*;
use wasmuri_core::util::Region;

pub const BACKGROUND_COLOR: Color = Color::from_rgb(100, 30, 150);
pub const BUTTON_COLOR: Color = Color::from_rgb(150, 0, 100);
pub const BUTTON_COLORS: TextColors = TextColors::create_simple_button(BUTTON_COLOR);

pub fn create_main_menu() -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(TextButton::celled(
        ButtonTextRenderController::simple_tuple("Play", font, button_location(Region::new(-2000, 6000, 2000, 8000)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            agent.change_container(create_play_menu());
    })));

    layer.add_component(TextButton::celled(
        ButtonTextRenderController::simple_tuple("Level Designer", font, button_location(Region::new(-4000, 3000, 4000, 5000)), BUTTON_COLORS),
        Box::new(|agent, _, _| {
            agent.change_container(create_designer_menu());
    })));

    let container = FlatContainer::new(layer);
    Rc::new(RefCell::new(container))
}

pub fn create_play_menu() -> Rc<RefCell<dyn Container>> {
    let pack_names = get_level_pack_names();

    // TODO Remove the && false once I'm ready
    if pack_names.len() == 1 && false {
        create_level_select(LevelPack::get_standard(), get_progress(&pack_names[0]))
    } else {
        let mut layer = Layer::new(Some(BACKGROUND_COLOR));
        let font = get_default_font();

        layer.add_component(TextButton::celled(
            ButtonTextRenderController::simple_tuple("Back", font, button_location(Region::new(-9000, 6000, -4000, 8000)), 
            TextColors::create_simple_button(Color::from_rgb(200, 150, 0))),
            Box::new(|agent, _, _| {
                agent.change_container(create_main_menu());
            })
        ));

        let mut bottom_y = 7000;
        for pack_name in pack_names {
            layer.add_component(TextButton::celled(ButtonTextRenderController::simple_tuple(&pack_name, font, left_button_location(Region::new(-3000, bottom_y, 3000, bottom_y + 2000)), 
            TextColors::create_simple_button(Color::from_rgb(50, 50, 150))), Box::new(move |agent, _, _| {
                agent.change_container(create_level_select(get_pack(&pack_name), get_progress(&pack_name)));
            })));
            bottom_y -= 2500;
        }

        FlatContainer::celled(layer)
    }
}