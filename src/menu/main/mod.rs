use crate::get_default_font;
use crate::storage::{
    get_level_pack_names,
    get_pack,
    get_progress
};
use crate::levels::LevelPack;
use crate::menu::game::create_level_select;

use std::rc::Rc;
use std::cell::RefCell;

use wasmuri_components::button::text::TextButton;
use wasmuri_components::helper::render::text::*;
use wasmuri_container::{
    Container,
    FlatContainer,
};
use wasmuri_container::layer::{
    Layer,
    Region
};

use wasmuri_core::util::{
    Color,
    TextColors
};

pub const BACKGROUND_COLOR: Color = Color::from_rgb(100, 30, 150);
pub const BUTTON_COLOR: Color = Color::from_rgb(150, 0, 100);
pub const BUTTON_COLORS: TextColors = TextColors::create_simple_button(BUTTON_COLOR);

pub fn create_main_menu() -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(Box::new(TextButton::new(
        Box::new(ButtonTextRenderHelper::simple("Play", font, Region::new(-0.2, 0.6, 0.2, 0.8), BUTTON_COLORS)),
        Box::new(move |_a, params| {
            params.agent.change_container(create_play_menu());
    }))));

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

        layer.add_component(TextButton::boxed(
            ButtonTextRenderHelper::simple_boxed("Back", font, Region::new(-0.9, 0.6, -0.4, 0.8), 
            TextColors::create_simple_button(Color::from_rgb(200, 150, 0))),
            Box::new(|_a, params| {
                params.agent.change_container(create_main_menu());
            })
        ));

        let mut bottom_y = 0.7;
        for pack_name in pack_names {
            layer.add_component(TextButton::boxed(ButtonTextRenderHelper::simple_boxed(&pack_name, font, Region::new(-0.3, bottom_y, 0.3, bottom_y + 0.2), 
            TextColors::create_simple_button(Color::from_rgb(50, 50, 150))), Box::new(move |_, params| {
                params.agent.change_container(create_level_select(get_pack(&pack_name), get_progress(&pack_name)));
            })));
            bottom_y -= 0.25;
        }

        Rc::new(RefCell::new(FlatContainer::new(layer)))
    }
}