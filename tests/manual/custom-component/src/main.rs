// Copyright © Klarälvdalens Datakonsult AB, a KDAB Group company, info@kdab.com
// SPDX-License-Identifier: MIT

use std::error::Error;
use std::cell::OnceCell;
use slint::{ComponentHandle, CustomItemDelegate, ItemPainter, ItemPropertyMap, Weak};

#[derive(Default)]
pub struct FancyItemDelegate {
    fancy_logic: OnceCell<Weak<FancyLogic<'static>>>,
}

impl CustomItemDelegate for FancyItemDelegate {
    fn render(&self, painter: &dyn ItemPainter) {
        let fancy_logic = self.fancy_logic.get().and_then(|f| f.upgrade()).unwrap();

        painter.set_line_brush(slint::Brush::SolidColor(slint::Color::from_rgb_u8(255, 255, 255)));
        painter.set_line_width(2.0);
        painter.draw_rectangle(painter.size());

        let fancy_size = fancy_logic.get_fancy_size();
        painter.set_brush(slint::Brush::SolidColor(fancy_logic.get_color1()));
        painter.fill_rectangle(slint::LogicalSize::new(fancy_size, fancy_size));
        painter.draw_rectangle(slint::LogicalSize::new(fancy_size, fancy_size));

        let fancy_spacing = fancy_logic.get_fancy_spacing();
        painter.translate(slint::LogicalPosition::new(fancy_spacing, fancy_spacing));
        painter.set_brush(slint::Brush::SolidColor(fancy_logic.get_color2()));
        painter.fill_rectangle(slint::LogicalSize::new(fancy_size, fancy_size));
        painter.draw_rectangle(slint::LogicalSize::new(fancy_size, fancy_size));

        painter.translate(slint::LogicalPosition::new(fancy_spacing, fancy_spacing));
        painter.set_brush(slint::Brush::SolidColor(fancy_logic.get_color3()));
        painter.fill_rectangle(slint::LogicalSize::new(fancy_size, fancy_size));
        painter.draw_rectangle(slint::LogicalSize::new(fancy_size, fancy_size));
    }
}

impl FancyItemDelegate {
    fn init<T: ComponentHandle + 'static>(&self, handle: &T, property_map: ItemPropertyMap) {
        let fancy_logic = handle.find_global::<FancyLogic>().unwrap();
        self.fancy_logic.set(fancy_logic.as_weak()).ok();
    }
}

slint::include_modules!();

fn main() -> Result<(), Box<dyn Error>> {
    let ui = AppWindow::new()?;

    ui.run()?;

    Ok(())
}