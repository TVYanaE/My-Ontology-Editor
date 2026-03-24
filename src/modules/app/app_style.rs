use std::collections::BTreeMap;

use eframe::CreationContext;
use eframe::egui::{FontFamily, FontId, TextStyle};

pub fn set_app_style(ctx: &CreationContext) {
    use FontFamily::{Monospace, Proportional};

    let text_styles: BTreeMap<TextStyle, FontId> = [
        (TextStyle::Heading, FontId::new(25.0, Proportional)),
        (TextStyle::Name("Heading2".into()), FontId::new(22.0, Proportional)),
        (TextStyle::Name("ContextHeading".into()), FontId::new(19.0, Proportional)),
        (TextStyle::Body, FontId::new(16.0, Proportional)),
        (TextStyle::Monospace, FontId::new(12.0, Monospace)),
        (TextStyle::Button, FontId::new(16.0, Proportional)),
        (TextStyle::Small, FontId::new(8.0, Proportional)),
    ]
    .into();
    ctx.egui_ctx.all_styles_mut(|style|{
        style.text_styles = text_styles.clone();
    });
}
