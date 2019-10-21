use crate::get_default_font;
use crate::levels::LevelPack;
use crate::menu::main::create_main_menu;
use crate::progress::ProgressEntry;

use std::rc::Rc;
use std::cell::RefCell;

use wasmuri_components::button::text::TextButton;
use wasmuri_components::helper::render::text::*;
use wasmuri_container::{
    Container,
    FlatContainer
};
use wasmuri_container::layer::Layer;
use wasmuri_core::color::*;
use wasmuri_core::util::Region;

pub fn create_level_select(_levels: LevelPack, _progress: ProgressEntry) -> Rc<RefCell<dyn Container>> {
    let mut layer = Layer::new(Some(Color::from_rgb(20, 50, 80)));
    let font = get_default_font();

    layer.add_component(TextButton::celled(ButtonTextRenderHelper::simple_celled("Cancel", font, button_location(Region::new(-0.9, 0.7, -0.6, 0.9)), 
        TextColors::create_simple_button(Color::from_rgb(200, 100, 0))), Box::new(|agent, _, _| {
            agent.change_container(create_main_menu());
    })));

    Rc::new(RefCell::new(FlatContainer::new(layer)))
}