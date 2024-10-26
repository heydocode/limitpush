use bevy::prelude::*;
use bevy_panic_handler::PanicHandler;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(
        PanicHandler::new()
            .set_title_func(|info| {
                format!(
                    "File: {} | Line: {} | Column {}",
                    format_string(info.location().unwrap().file()),
                    info.location().unwrap().line(),
                    info.location().unwrap().column()
                )
            })
            .set_body_func(|info| {
                format!(
            "Catched a panic at line {}, column {}.\nIncluded message: {}\nFull file path: \"{}\"",
            info.location().unwrap().line(),
            info.location().unwrap().column(),
            info.payload()
                .downcast_ref::<String>()
                .cloned()
                .unwrap_or_else(|| info
                    .payload()
                    .downcast_ref::<&str>()
                    .unwrap_or(&"")
                    .to_string()),
            info.location().unwrap().file()
        )
            })
            .build(),
    );
}

fn format_string(info: &str) -> String {
    if info.len() > 40 {
        format!("...{}", &info[info.len() - 40..])
    } else {
        info.to_string()
    }
}
