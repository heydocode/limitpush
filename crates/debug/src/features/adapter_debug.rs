use bevy::{prelude::*, render::renderer::RenderAdapterInfo};
use bevy_egui::{egui, EguiContexts};

pub(super) fn plugin(app: &mut App) {
    app.init_resource::<Drivernfo>();
    app.add_systems(Startup, (push_adapter_info, print_driver_info).chain());
    app.add_systems(Update, driver_ui);
}

// TODO! Here save the info pushed by push_adapter_info
#[derive(Resource, Default)]
pub struct Drivernfo {
    pub name: String,
    pub vendor: u32,
    pub device: u32,
    pub device_type: String,
    pub driver: String,
    pub driver_info: String,
    pub backend: String,
}

impl Drivernfo {
    pub fn new(
        name: String,
        vendor: u32,
        device: u32,
        device_type: String,
        driver: String,
        driver_info: String,
        backend: String,
    ) -> Self {
        Self {
            name,
            vendor,
            device,
            device_type,
            driver,
            driver_info,
            backend,
        }
    }

    pub fn iter(driver_info: Res<Drivernfo>) -> Vec<String> {
        Vec::from([
            driver_info.name.clone(),
            driver_info.vendor.to_string(),
            driver_info.device.to_string(),
            driver_info.device_type.clone(),
            driver_info.driver.clone(),
            driver_info.driver_info.clone(),
            driver_info.backend.clone(),
        ])
    }
}

fn push_adapter_info(adapter_info: Res<RenderAdapterInfo>, mut driver_info: ResMut<Drivernfo>) {
    let device_type = format!("{:?}", adapter_info.device_type);
    let backend = format!("{:?}", adapter_info.backend);

    // println!("GPU Name: {}", adapter_info.name);
    // println!("Vendor: {}", adapter_info.vendor);
    // println!("Device Type: {}", device_type);
    // println!("Backend: {}", backend);

    *driver_info = Drivernfo::new(
        adapter_info.name.clone(),
        adapter_info.vendor,
        adapter_info.device,
        device_type,
        adapter_info.driver.clone(),
        adapter_info.driver_info.clone(),
        backend,
    );
}

fn print_driver_info(driver_info: Res<Drivernfo>) {
    let iterable: Vec<String> = Drivernfo::iter(driver_info);
    for element in iterable {
        println!("{}", element);
    }
}

fn driver_ui(mut contexts: EguiContexts, driver_info: Res<Drivernfo>) {
    //     let extra_text = format!("RENDER DEVICE:
    // {} {}
    // USED DRIVER:
    // {}", driver_info.device_type, driver_info.name, driver_info.backend);
    let iterable: Vec<String> = Drivernfo::iter(driver_info);
    let mut extra_text = String::new();
    for element in iterable {
        extra_text += &element;
        extra_text += "\n";
    }
    extra_text.pop();
    egui::Window::new("EXTRA").show(contexts.ctx_mut(), |ui| {
        #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
        {
            let mut frame = egui::Frame::default().inner_margin(4.0).begin(ui);
            {
                frame.content_ui.label("HOVER TO SHOW");
            }
            let response = frame.allocate_space(ui);
            if response.hovered() {
                frame.content_ui.label(extra_text);
                frame.allocate_space(ui);
            }

            frame.paint(ui);
        }
        #[cfg(not(any(target_os = "windows", target_os = "macos", target_os = "linux")))]
        {
            let mut frame = egui::Frame::default().inner_margin(4.0).begin(ui);
            {
                frame.content_ui.label(extra_text);
            }

            frame.paint(ui);
        }
    });
}
