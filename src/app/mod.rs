mod window;
use std::time::SystemTime;

use gtk::{prelude::*, subclass::prelude::ObjectSubclassIsExt, Application, Label};
pub use window::Window;

const WINDOW_MIN_HEIGHT: i32 = 120;
const LABEL_ORIGINAL: &str = "origin";
const LABEL_TRANSLATED: &str = "translated";

pub mod utils;

pub fn build_main_window(
    app: &Application,
    full_width_label: bool,
    hide_label_on_empty_text: bool,
    click_pass_through: bool,
    origin_lyric_in_above: bool,
    enable_filter_regex: bool,
    cache_lyrics: bool,
    length_toleration_ms: u128,
) -> Window {
    let window = Window::new(app);

    window.set_size_request(500, WINDOW_MIN_HEIGHT);
    window.set_title(Some("Waylyrics"));
    window.set_decorated(false);
    window.present();

    let verical_box = gtk::Box::builder()
        .baseline_position(gtk::BaselinePosition::Center)
        .orientation(gtk::Orientation::Vertical)
        .build();
    verical_box.set_vexpand(true);
    verical_box.set_valign(gtk::Align::Center);

    let (olabel, tlabel) = build_labels(
        full_width_label,
        hide_label_on_empty_text,
        enable_filter_regex,
    );

    let lock_label = Label::builder().name("lock").label("🔒").build();
    lock_label.set_halign(gtk::Align::Center);
    lock_label.set_valign(gtk::Align::Start);

    let sibling: Option<&Label> = None;
    verical_box.insert_child_after(&lock_label, sibling);
    verical_box.insert_child_after(&olabel, Some(&lock_label));
    verical_box.insert_child_after(&tlabel, Some(&olabel));

    if !origin_lyric_in_above {
        verical_box.reorder_child_after(&olabel, Some(&tlabel));
    }

    window.set_child(Some(&verical_box));
    *window.imp().original.borrow_mut() = olabel;
    *window.imp().translated.borrow_mut() = tlabel;

    if click_pass_through {
        utils::set_click_pass_through(&window.surface(), true)
    }

    window.set_icon_name(Some(crate::APP_ID));
    window.imp().lyric_start.set(Some(SystemTime::now()));
    window.imp().cache_lyrics.set(cache_lyrics);
    window.imp().length_toleration_ms.set(length_toleration_ms);

    window
}

pub fn get_label_act<R>(window: &Window, translated: bool, act: impl FnOnce(&Label) -> R) -> R {
    let label = if !translated {
        &window.imp().original
    } else {
        &window.imp().translated
    };

    act(&label.borrow())
}

fn build_labels(
    full_width_label: bool,
    hide_label_on_empty_text: bool,
    enable_filter_regex: bool,
) -> (Label, Label) {
    let olabel = Label::builder()
        .label("Waylyrics")
        .name(LABEL_ORIGINAL)
        .build();
    let tlabel = Label::builder()
        .label("")
        .name(LABEL_TRANSLATED)
        .visible(false)
        .build();

    for label in [&olabel, &tlabel] {
        utils::setup_label(label, hide_label_on_empty_text, enable_filter_regex);
    }
    olabel.set_vexpand(false);
    tlabel.set_vexpand(false);

    if !full_width_label {
        olabel.set_halign(gtk::Align::Center);
        tlabel.set_halign(gtk::Align::Center);
    }

    (olabel, tlabel)
}
