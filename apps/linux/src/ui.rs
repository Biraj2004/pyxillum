//! Linux GTK4 + libadwaita UI layout and widgets.

#[cfg(target_os = "linux")]
use std::sync::Arc;

#[cfg(target_os = "linux")]
use gtk4::prelude::*;
#[cfg(target_os = "linux")]
use libadwaita::prelude::*;
#[cfg(target_os = "linux")]
use pyxillum_core::{Action, Engine};

#[cfg(target_os = "linux")]
pub fn build_ui(app: &libadwaita::Application, engine: Arc<Engine>) {
    let window = libadwaita::ApplicationWindow::builder()
        .application(app)
        .title("Pyxillum")
        .default_width(1080)
        .default_height(720)
        .build();

    let toast_overlay = libadwaita::ToastOverlay::new();

    let split_view = libadwaita::NavigationSplitView::new();
    split_view.set_min_sidebar_width(220.0);
    split_view.set_max_sidebar_width(280.0);

    // Sidebar Navigation
    let sidebar_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    let sidebar_header = libadwaita::HeaderBar::new();
    sidebar_box.append(&sidebar_header);

    let nav_list = gtk4::ListBox::new();
    nav_list.add_css_class("navigation-sidebar");
    nav_list.append(&libadwaita::ActionRow::builder().title("Device Hub").build());
    nav_list.append(&libadwaita::ActionRow::builder().title("Dynamic FX").build());
    nav_list.append(
        &libadwaita::ActionRow::builder()
            .title("Solid Colour")
            .build(),
    );
    nav_list.append(
        &libadwaita::ActionRow::builder()
            .title("Music Rhythm")
            .build(),
    );
    nav_list.append(
        &libadwaita::ActionRow::builder()
            .title("DIY Sequencer")
            .build(),
    );
    nav_list.append(
        &libadwaita::ActionRow::builder()
            .title("Hardware Settings")
            .build(),
    );
    sidebar_box.append(&nav_list);

    let sidebar_page = libadwaita::NavigationPage::new(&sidebar_box, "Navigation");
    split_view.set_sidebar(Some(&sidebar_page));

    // Content View
    let content_box = gtk4::Box::new(gtk4::Orientation::Vertical, 0);
    let content_header = libadwaita::HeaderBar::new();

    // Master Power Toggle in Header
    let power_btn = gtk4::Button::from_icon_name("system-shutdown-symbolic");
    let eng_clone = engine.clone();
    power_btn.connect_clicked(move |_| {
        let _ = eng_clone.dispatch(Action::TogglePower);
    });
    content_header.pack_end(&power_btn);
    content_box.append(&content_header);

    // Bluetooth Warning Banner
    let banner = libadwaita::Banner::new("Bluetooth adapter is powered off or inaccessible.");
    banner.set_revealed(false);
    content_box.append(&banner);

    // Status & Main View Stack
    let status_page = libadwaita::StatusPage::builder()
        .icon_name("network-wireless-symbolic")
        .title("Pyxillum LED Controller")
        .description("Connected to virtual SP621E hardware simulator.")
        .build();
    content_box.append(&status_page);

    // Bottom LED Strip Monitor Canvas with tick callback pulling engine.frame()
    let drawing_area = gtk4::DrawingArea::new();
    drawing_area.set_height_request(24);
    drawing_area.set_margin_start(16);
    drawing_area.set_margin_end(16);
    drawing_area.set_margin_bottom(12);

    let eng_for_tick = engine.clone();
    drawing_area.add_tick_callback(move |widget, _| {
        let _frame = eng_for_tick.frame();
        widget.queue_draw();
        glib::ControlFlow::Continue
    });
    content_box.append(&drawing_area);

    let content_page = libadwaita::NavigationPage::new(&content_box, "Content");
    split_view.set_content(Some(&content_page));

    toast_overlay.set_child(Some(&split_view));
    window.set_content(Some(&toast_overlay));
    window.present();
}
