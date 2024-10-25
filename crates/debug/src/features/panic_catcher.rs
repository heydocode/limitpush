use bevy::{prelude::*, render::renderer::RenderAdapterInfo};
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
            "Catched a panic at line {}, column {}.\nIncluded message: {}\nConfiguration: {}",
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
            info.location().unwrap().file() // TODO! here implement all the configuration
            // including the used GPU driver and the target
        )
            })
            .build(),
    );

    app.add_systems(Startup, push_adapter_info);
}

// TODO! Here save the info pushed by push_adapter_info
// pub struct Drivernfo {

// }

// TODO! Change this function to create another bevy_egui window
// to display the current backend + indicate it in the
// unhandle panic cases because some issues can occur due
// do the backend, graphics card (name) or even vendor
// (really unsure about it but anyway :3)
fn push_adapter_info(adapter_info: Res<RenderAdapterInfo>) {
    println!("GPU Name: {}", adapter_info.name);
    println!("Vendor: {}", adapter_info.vendor);
    println!("Device Type: {:?}", adapter_info.device_type);
    println!("Backend: {:?}", adapter_info.backend);
}

fn format_string(info: &str) -> String {
    if info.len() > 40 {
        format!("...{}", &info[info.len() - 40..])
    } else {
        info.to_string()
    }
}
