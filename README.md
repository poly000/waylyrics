# Waylyrics

[![tg-group](https://img.shields.io/badge/tg%20group-open-blue)](https://t.me/waylyrics)

Simple universal on screen lyrics made with GTK4 and ❤️.

![](https://github.com/poly000/waylyrics/assets/34085039/43037cb4-9a07-4e77-b112-1408365199e2)

- [Waylyrics](#waylyrics)
  - [Build/Install](#buildinstall)
  - [Requirement](#requirement)
    - [Recommended Players](#recommended-players)
    - [MPRIS-unfriendly Players](#mpris-unfriendly-players)
  - [Directories](#directories)
  - [Approach](#approach)
  - [Alternatives](#alternatives)
  - [Credit](#credit)
  - [License](#license)

## Build/Install

Check [INSTALLATION.md](INSTALLATION.md)

## Requirement

- A player supports MPRIS well.
- In particular, a wm allows you set windows as top-level one's.

### Recommended Players

online:
- [Qcm](https://github.com/hypengw/Qcm)
- [Electron-NCM](https://github.com/Rocket1184/electron-netease-cloud-music)
- [YesPlayMusic](https://github.com/qier222/YesPlayMusic)
- [Telegram](https://t.me/Music163Bot)
- [FeelUOwn](https://github.com/feeluown/FeelUOwn/), with the latest code

local:
- [mpv-mpris](https://github.com/hoyon/mpv-mpris)
- [VLC](https://www.videolan.org)

### MPRIS-unfriendly Players

[netease-cloud-music-gtk]: https://github.com/gmg137/netease-cloud-music-gtk
[flutter-netease-music]: https://github.com/boyan01/flutter-netease-music
[youtube-music]: https://github.com/th-ch/youtube-music


| Player                    | OSD | issue                                     |
| ------------------------- | --- | ----------------------------------------- |
| [netease-cloud-music-gtk] | X   | gives 0 position                          |
| Firefox                   | X   | do not provide position call              |
| Chrom*                    | X   | break the "unique" gurantee for TrackID   |
| [youtube-music]           | X   | break the "unique" gurantee for TrackID   |
| qqmusic                   | O   | no info other than title/artist avaliable |
| [flutter-netease-music]   | X   | no mpris support                          |

## Directories

Typically,

```
~/.cache/waylyrics/XX/...
~/.config/waylyrics/...
~/.local/share/waylyrics/themes/...
```

## Approach

Current approach my seems dirty:

1. get the likely actived player when none is connnected, and disconnect from one only if it's not avaliable more
2. keep sync with 2s interval and 20ms refresh for lyrics
3. for Qcm, Feeluown-netease, ElectronNCM, we could get song id directly
4. use the length-matched result (or first result if former is not found) of `search_song` and sync START in each run, fetch lyric only when needed

## Alternatives

[YesPlayMusicOSD]: https://github.com/shih-liang/YesPlayMusicOSD
[waybar-netease-music-lyrics]: https://github.com/kangxiaoju/waybar-netease-music-lyrics

For Sway users, you may want use [waybar-netease-music-lyrics].

[YesPlayMusicOSD] have great lyrics support

BruceZhang1993's [lyricsSeeker](https://github.com/BruceZhang1993/LyricsSeeker) is still WIP, but it may have better-looking and better integration with KDE.

Copay's [caraoke-plasmoid](https://github.com/Copay/caraoke-plasmoid) is currently Plasma-only, though it is easy to remove plasmoid components

## Credit

[gtk4-rs]: https://github.com/gtk-rs/gtk4-rs

- [gtk4-rs], Rust bind to GTK-4.


## License

[![FOSSA Status](https://app.fossa.com/api/projects/git%2Bgithub.com%2Fpoly000%2Fwaylyrics.svg?type=large)](https://app.fossa.com/projects/git%2Bgithub.com%2Fpoly000%2Fwaylyrics?ref=badge_large)
