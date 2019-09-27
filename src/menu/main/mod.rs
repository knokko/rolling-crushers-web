use std::rc::Rc;
use std::cell::RefCell;

use wasmuri_components::button::text::TextButton;
use wasmuri_components::passive::PassiveText;
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

use wasmuri_text::Font;

pub const BACKGROUND_COLOR: Color = Color::from_rgb(100, 30, 150);
pub const BUTTON_COLOR: Color = Color::from_rgb(150, 0, 100);
pub const BUTTON_COLORS: TextColors = TextColors::create_simple_button(BUTTON_COLOR);

pub fn create_main_menu(font: &Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));

    let font_clone = Rc::clone(font);

    layer.add_component(Box::new(TextButton::new(
        Box::new(ButtonTextRenderHelper::simple("New game", font, Region::new(-0.2, 0.6, 0.2, 0.8), BUTTON_COLORS)),
        Box::new(move |_a, params| {
            params.agent.change_container(create_new_game_menu(&font_clone));
    }))));

    let container = FlatContainer::new(layer);
    Rc::new(RefCell::new(container))
}

pub fn create_new_game_menu(font: &Rc<Font>) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(BACKGROUND_COLOR));

    layer.add_component(Box::new(PassiveText::new(Box::new(SimpleTextRenderHelper::new("New game...", font, Region::new(-0.3, 0.7, 0.3, 0.9), TextColors::BLACK_LABEL)))));

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}