use crate::EXCLUDED_REGEXES;

use gtk::{
    cairo::{RectangleInt, Region},
    gdk::Surface,
    prelude::*,
    Label,
};

pub fn set_click_pass_through(surface: &Surface, enabled: bool) {
    if enabled {
        surface.set_input_region(&Region::create());
    } else {
        surface.set_input_region(&Region::create_rectangle(&RectangleInt::new(
            0,
            0,
            i32::MAX,
            i32::MAX,
        )))
    }
}

pub fn merge_css(css: &str) {
    use gtk::gdk::Display as GdkDisplay;

    let css_provider = gtk::CssProvider::new();
    css_provider.load_from_data(css);

    gtk::style_context_add_provider_for_display(
        &GdkDisplay::default().expect("Could not connect to a display."),
        &css_provider,
        gtk::STYLE_PROVIDER_PRIORITY_APPLICATION,
    );
}

pub fn has_filtered_word(text: &str) -> bool {
    EXCLUDED_REGEXES.with_borrow(|regex_set| regex_set.is_match(text))
}

pub fn setup_label(label: &Label, hide_empty_label: bool, hide_filtered_words: bool) {
    match (hide_empty_label, hide_filtered_words) {
        (true, false) => {
            label.connect_label_notify(|label| {
                label.set_visible(!label.label().is_empty());
            });
        }
        (false, true) => {
            label.connect_label_notify(|label| {
                label.set_visible(!has_filtered_word(label.label().as_str()));
            });
        }
        (true, true) => {
            label.connect_label_notify(|label| {
                let label_text = label.label();

                label
                    .set_visible(!has_filtered_word(label_text.as_str()) && !label_text.is_empty());
            });
        }
        (false, false) => (),
    };
}
