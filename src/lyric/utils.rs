use lrc_nom::{parse, LrcParseError};
use std::time::Duration;

use super::{Lyric, LyricLine, LyricLineOwned};

pub fn lrc_iter<'a>(
    lyric_lines: impl Iterator<Item = &'a str>,
) -> Result<Vec<LyricLine<'a>>, LrcParseError> {
    let mut lrc_vec: Vec<_> = parse(lyric_lines)?
        .into_iter()
        .filter_map(|lrc_item| match lrc_item {
            lrc_nom::LrcItem::Metadata(_) => None,
            lrc_nom::LrcItem::Lyric(lyric, timestamp) => Some(LyricLine {
                text: lyric,
                start_time: Duration::from_millis(timestamp as _),
            }),
        })
        .collect();
    // handling malformed LRC timestamp by sorting them here
    lrc_vec.sort_by(|left, right| left.start_time.cmp(&right.start_time));
    Ok(lrc_vec)
}

pub fn find_next_lyric<'a>(
    elapsed: &Duration,
    lyric: &'a [LyricLineOwned],
) -> Option<&'a LyricLineOwned> {
    lyric
        .iter()
        .take_while(
            |LyricLineOwned {
                 start_time: off, ..
             }| off <= elapsed,
        )
        .last()
}

pub struct LrcLyric {
    pub lyric: Option<String>,
    pub tlyric: Option<String>,
}

impl super::LyricStore for LrcLyric {
    fn get_lyric(&self) -> Lyric<'_> {
        let lyric = self.lyric.as_deref();
        match_lyric(lyric)
    }

    fn get_translated_lyric(&self) -> Lyric<'_> {
        let lyric = self.tlyric.as_deref();
        match_lyric(lyric)
    }
}

fn match_lyric(lyric: Option<&str>) -> Lyric<'_> {
    match lyric {
        Some("") | None => super::Lyric::None,
        Some(lyric) => {
            if let Ok(parsed) = super::utils::lrc_iter(lyric.split("\n")) {
                Lyric::LineTimestamp(parsed)
            } else {
                Lyric::NoTimestamp
            }
        }
    }
}
