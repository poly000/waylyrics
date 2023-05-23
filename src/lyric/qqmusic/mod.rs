use super::LyricStore;

pub struct QQMusicProvider;

pub struct QQLyric {
    origin: String,
    translated: String,
}

impl super::LyricProvider for QQMusicProvider {
    type Id = String;

    type LStore = QQLyric;

    const NAME: &'static str = "QQ音乐";

    fn new() -> Result<Box<Self>, Box<dyn std::error::Error>> {
        Ok(Box::new(QQMusicProvider))
    }

    fn query_lyric(
        &self,
        handle: &tokio::runtime::Handle,
        id: Self::Id,
    ) -> Result<Self::LStore, Box<dyn std::error::Error>> {
        handle.block_on(async { Err("not implemented".into()) })
    }

    fn search_song(
        &self,
        handle: &tokio::runtime::Handle,
        album: &str,
        artists: &[&str],
        title: &str,
    ) -> Result<Vec<super::SongInfo<Self::Id>>, Box<dyn std::error::Error>> {
        let _ = album;
        todo!()
    }
}

impl LyricStore for QQLyric {
    fn get_lyric(&self) -> super::Lyric<'_> {
        todo!()
    }

    fn get_translated_lyric(&self) -> super::Lyric<'_> {
        todo!()
    }
}
