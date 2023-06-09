use std::time::Duration;

use gtk::subclass::prelude::ObjectSubclassIsExt;

use crate::{
    app,
    lyric::{LyricOwned, SongInfo},
};

use super::LYRIC;

pub fn clear_lyric(window: &app::Window) {
    LYRIC.set((LyricOwned::None, LyricOwned::None));
    window.imp().lyric_playing.set(None);
    window.imp().lyric_playing_translation.set(None);
    window.imp().lyric_offset_ms.set(0);
}

pub fn match_likely_lyric<'a, Id>(
    album_title: Option<(&str, &str)>,
    length: Option<Duration>,
    search_result: &'a [SongInfo<Id>],
    length_toleration_ms: u128,
) -> Option<&'a Id> {
    length
        .and_then(|leng| {
            search_result.iter().find(|SongInfo { length, .. }| {
                length.as_millis().abs_diff(leng.as_millis()) <= length_toleration_ms
            })
        })
        .or_else(|| {
            album_title.and_then(|(_album, _title)| {
                search_result.iter().find(|SongInfo { title, album, .. }| {
                    title == _title && album.as_ref().is_some_and(|album| album == _album)
                })
            })
        })
        .or(search_result.get(0))
        .map(|song| &song.id)
}
