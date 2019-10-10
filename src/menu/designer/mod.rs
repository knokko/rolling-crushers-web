mod pack_edit;
pub use pack_edit::*;

use crate::get_default_font;
use crate::menu::main::create_main_menu;

use std::cell::RefCell;
use std::rc::Rc;

use wasmuri_components::button::text::TextButton;
use wasmuri_components::helper::render::text::*;
use wasmuri_container::{
    Container,
    FlatContainer,
    layer::Layer
};
use wasmuri_core::color::*;
use wasmuri_core::util::Region;

const BACKGROUND_COLOR: Color = Color::from_rgb(30, 30, 150);
const BUTTON_COLORS: TextColors = TextColors::create_simple_button(Color::from_rgb(50, 200, 150));
const LABEL_COLORS: TextColors = TextColors::BLACK_LABEL;
const EDIT_COLORS: TextColors = TextColors::new(Color::BLACK, Color::BLACK, Color::from_rgb(190, 190, 190));

pub fn create_designer_menu() -> Rc<RefCell<dyn Container>> {
    create_level_pack_overview()
}

pub fn create_level_pack_overview() -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));
    let font = get_default_font();

    layer.add_component(TextButton::boxed(
        ButtonTextRenderHelper::simple_boxed("Back", font, button_location(Region::new(-1.0, 0.6, -0.6, 0.8)), BUTTON_COLORS),
        Box::new(|_a, params| {
            params.agent.change_container(create_main_menu());
    })));

    layer.add_component(TextButton::boxed(
        ButtonTextRenderHelper::simple_boxed("New level pack", font, button_location(Region::new(-1.0, -0.4, -0.4, -0.2)), BUTTON_COLORS),
        Box::new(|_a, params| {
            params.agent.change_container(create_level_pack_creation());
    })));

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}