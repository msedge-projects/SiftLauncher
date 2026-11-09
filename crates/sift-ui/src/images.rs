//! Project icons, fetched off the frame thread and cached as textures.
//!
//! The reference's cards carry a project's icon at 64 points. Drawing that from a URL means
//! three separate costs -- the network, the decode, and the texture upload -- and the one
//! rule that matters is that **none of them happen on the frame thread**. So a request is a
//! thread, the result arrives over a channel, and the frame that finds it ready uploads once
//! and keeps the `TextureHandle` for ever after.
//!
//! Three states, keyed by URL: `Loading` (a worker is out, and the caller keeps asking for
//! frames so the upload happens promptly), `Ready` (a texture), `Failed` (no retry this run
//! -- a missing icon is a grey tile, not a loop of requests).
//!
//! The fetch goes through `sift_net::download_file`, the launcher's own client, so the
//! timeout, the user agent and the proxy settings are the ones the rest of the app uses
//! rather than a second opinion.

use std::collections::HashMap;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::time::Duration;

use egui::{self, ColorImage, TextureHandle};

/// What the cache knows about one URL.
enum Entry {
    /// A worker is fetching it.
    Loading,
    /// The texture, uploaded, the tick it was last asked for, and **how many bytes it occupies**.
    ///
    /// The byte count is what makes the cap honest: ninety-six 96-point icons is three megabytes, but ninety-six
    /// 1920x1080 screenshots is more than eight hundred. A cap on the *number* of entries is a cap on nothing.
    Ready(TextureHandle, u64, usize),
    /// The fetch or the decode failed. Not retried this run.
    Failed,
}

/// How many textures the cache keeps before it drops the coldest.
///
/// **A launcher's memory should not depend on how much its reader has browsed.** Every icon is
/// 96 points square, so this cap is a few megabytes of texture at most; without it, a session
/// that scrolls Discover all afternoon keeps every icon it has ever seen and the idle-RSS
/// measurement stops being repeatable. 256 is far more than one screen of cards plus the
/// project pages a reader moves between, so eviction is invisible in use.
// 256 decoded icons was a guess at "many"; the Discover page shows twenty cards and the library a
// handful of rows, so most of those textures were held for a screen that never draws them. Ninety-
// six still covers several pages of re-visits.
const CACHE_LIMIT: usize = 96;

/// The cache's real budget: **64 MiB of textures**, whichever limit is reached first.
///
/// Sized for a screenshot library: two full-screen 1440x900 shots per megabyte each, plus a wall of icons, keeps any
/// view the reader scrolls to resident while leaving the process's footprint predictable. Before this, the only cap was
/// the entry count, and a reader paging through a screenshot folder could reach hundreds of megabytes without any single
/// entry looking expensive.
// **8 MiB, not 64.** A single full-size screenshot is 5.2 MB of texture and its decode a second 5.2, and at a 64 MiB
// budget neither was ever evicted: one preview lifted the process from 13 720 kB to 28 108 kB and closing the viewer did not
// give a byte back. At 8 MiB the viewer's own texture is over budget on its own and goes as soon as the reader leaves it,
// while the grid's thumbnails -- a few hundred kilobytes each -- still hold several screens of scroll.
const CACHE_BYTES: usize = 8 * 1024 * 1024;

/// A decoded image's size as bytes, which is what the texture will occupy once uploaded.
fn texture_bytes(size: [usize; 2]) -> usize {
    size[0].saturating_mul(size[1]).saturating_mul(4)
}

/// The icon cache: one entry per URL, and the channel its workers answer on.
pub struct Images {
    entries: HashMap<String, Entry>,
    sender: Sender<(String, Result<ColorImage, String>)>,
    receiver: Receiver<(String, Result<ColorImage, String>)>,
    /// Requests still out, so the caller can keep frames coming while they are.
    inflight: usize,
    /// A monotonic counter used as the last-used stamp; every `get` advances it.
    tick: u64,
}

impl Default for Images {
    fn default() -> Images {
        let (sender, receiver) = channel();
        Images {
            entries: HashMap::new(),
            sender,
            receiver,
            inflight: 0,
            tick: 0,
        }
    }
}

impl Images {
    /// Drop the coldest ready entries until the cache is inside its limit.
    ///
    /// Only `Ready` entries are candidates: a `Loading` one is a request in flight and a
    /// `Failed` one costs a `String`, and dropping either would re-ask for something the
    /// cache has already answered.
    fn evict(&mut self) {
        let ready = self
            .entries
            .iter()
            .filter(|(_, entry)| matches!(entry, Entry::Ready(_, _, _)))
            .count();
        let bytes: usize = self
            .entries
            .values()
            .filter_map(|entry| match entry {
                Entry::Ready(_, _, bytes) => Some(*bytes),
                _ => None,
            })
            .sum();
        if ready <= CACHE_LIMIT && bytes <= CACHE_BYTES {
            return;
        }
        // Whichever budget is over decides how many go; the coldest go first either way.
        let mut stamps: Vec<(u64, String)> = self
            .entries
            .iter()
            .filter_map(|(url, entry)| match entry {
                Entry::Ready(_, last, _) => Some((*last, url.clone())),
                _ => None,
            })
            .collect();
        // Coldest first; the count can only be over the limit by one per insert, but the
        // loop costs nothing either way.
        stamps.sort_unstable();
        let over_entries = ready.saturating_sub(CACHE_LIMIT);
        let mut dropped = 0;
        let mut freed = 0;
        let mut bytes_now = bytes;
        for (_, url) in stamps {
            if dropped >= over_entries && bytes_now <= CACHE_BYTES {
                break;
            }
            if let Some(Entry::Ready(_, _, bytes)) = self.entries.get(&url) {
                freed += *bytes;
            }
            bytes_now = bytes_now.saturating_sub(freed.min(bytes_now));
            self.entries.remove(&url);
            dropped += 1;
            freed = 0;
            // One entry is never dropped: the texture on screen must stay resident this frame.
            if self.entries.len() <= 1 {
                break;
            }
        }
    }

    /// The texture for a file on disk, decoded the first time and cached under its path.
    ///
    /// Synchronous, unlike [`Images::get`]: the file is on the disk the caller has just read, and a
    /// screenshot decodes in a few milliseconds -- a worker would cost a frame of latency for it.
    pub fn get_file(&mut self, ctx: &egui::Context, path: &std::path::Path) -> Option<TextureHandle> {
        self.get_file_with(ctx, path, egui::TextureOptions::LINEAR, "smooth", 0)
    }

    /// The same file, decoded no larger than `cap` on its longest side. **A screenshot is 1440x900 and a card draws it
    /// 510 wide**, so decoding it whole spends 5.2 MB of texture and 5.2 MB of intermediate bitmap to fill a quarter of
    /// that -- and both were held for the life of the process. The cap is a separate cache entry from the full-size one,
    /// so the viewer can still open the original after a card has shown the thumbnail.
    pub fn get_file_capped(
        &mut self,
        ctx: &egui::Context,
        path: &std::path::Path,
        cap: u32,
    ) -> Option<TextureHandle> {
        self.get_file_with(ctx, path, egui::TextureOptions::LINEAR, "smooth", cap)
    }

    /// The same file, sampled nearest instead of smooth.
    ///
    /// A screenshot is a photograph and wants bilinear filtering; a Minecraft skin is eight
    /// texels of face blown up to a hundred points, and smoothing it turns a pixel-art face
    /// into a gradient. The two samplings are separate cache entries, because the texture is
    /// uploaded with its sampling baked in and one path cannot be both.
    pub fn get_file_nearest(
        &mut self,
        ctx: &egui::Context,
        path: &std::path::Path,
    ) -> Option<TextureHandle> {
        self.get_file_with(ctx, path, egui::TextureOptions::NEAREST, "nearest", 0)
    }

    /// Decode `path` once and cache it under its path and sampling.
    fn get_file_with(
        &mut self,
        ctx: &egui::Context,
        path: &std::path::Path,
        options: egui::TextureOptions,
        sampling: &str,
        cap: u32,
    ) -> Option<TextureHandle> {
        // The sampling is part of the key: a path asked for both ways gets one texture each,
        // and the suffix keeps that key from colliding with a URL. The cap joins it for the same reason:
        // the thumbnail and the original are different textures.
        let key = if cap == 0 {
            format!("file:{sampling}:{}", path.display())
        } else {
            format!("file:{sampling}:{cap}:{}", path.display())
        };
        self.tick += 1;
        let tick = self.tick;
        match self.entries.get_mut(&key) {
            Some(Entry::Ready(texture, last, _)) => {
                *last = tick;
                return Some(texture.clone());
            }
            Some(Entry::Failed) => return None,
            _ => {}
        }
        let decoded = std::fs::read(path)
            .ok()
            .and_then(|bytes| image::load_from_memory(&bytes).ok())
            .map(|image| {
                if cap > 0 && (image.width() > cap || image.height() > cap) {
                    image.resize(cap, cap, image::imageops::FilterType::Triangle)
                } else {
                    image
                }
            })
            .map(|image| image.to_rgba8());
        let Some(image) = decoded else {
            self.entries.insert(key, Entry::Failed);
            return None;
        };
        let size = [image.width() as usize, image.height() as usize];
        let pixels = egui::ColorImage::from_rgba_unmultiplied(size, image.as_raw());
        let texture = ctx.load_texture(key.clone(), pixels, options);
        let bytes = size[0].saturating_mul(size[1]).saturating_mul(4);
        self.entries.insert(key, Entry::Ready(texture.clone(), tick, bytes));
        self.evict();
        // **Trim after a large synchronous decode.** `fetch` trims when an asynchronous reply lands, but a file decode --
        // which is what the viewer does -- never did, so the 5 MB bitmap that became the texture stayed in the allocator as
        // a free arena and the process never went back down: one preview read 38 840 kB with the viewer closed and the reader
        // back on Home. Only worth a syscall when the decode was big.
        if bytes >= 1024 * 1024 {
            trim_heap();
        }
        Some(texture)
    }

    /// The icon for `url`, if it is ready. Starts the fetch the first time it is asked.
    ///
    /// `None` while loading and after a failure, deliberately: a card does not care *why*
    /// it has no picture, only that it has none, and both cases draw the same placeholder.
    /// **A picture, at its own size.** Same cache, same channel, a different key and a larger cap: `#picture` cannot collide
/// with a URL, and an icon and a picture of the same URL are genuinely different textures.
pub fn get_picture(&mut self, ctx: &egui::Context, url: &str) -> Option<TextureHandle> {
    self.get_capped(ctx, url, MAX_PICTURE)
}

pub fn get(&mut self, ctx: &egui::Context, url: &str) -> Option<TextureHandle> {
    self.get_capped(ctx, url, MAX_ICON)
}

/// The one fetch path, with the cap that decides how big a texture may be. Icons and pictures share the cache and the
/// channel and differ only in this number and in the key they are filed under.
fn get_capped(&mut self, ctx: &egui::Context, url: &str, cap: u32) -> Option<TextureHandle> {
    let key = if cap == MAX_ICON { url.to_owned() } else { format!("{url}#{cap}") };
    self.pump(ctx);
        self.tick += 1;
        let tick = self.tick;

        match self.entries.get_mut(&key) {
            Some(Entry::Ready(texture, last, _)) => {
                // A hit is a use: the entry the reader keeps looking at is the last to go.
                *last = tick;
                return Some(texture.clone());
            }
            Some(Entry::Loading | Entry::Failed) => return None,
            None => {}
        }

        let url_owned = url.to_owned();
        // **The reply rides back under the cache key, not the URL.** A picture is filed as `url#1024`; replying with the bare
        // URL put its texture where nothing looked for it, and the picture never appeared.
        let reply = key.clone();
        let sender = self.sender.clone();
        let spawned = std::thread::Builder::new()
            .name("sift-icon".to_owned())
            .spawn(move || {
                let image = fetch(&url_owned, cap);
                // A closed receiver means the window is gone; dropping the result is right.
                let _ = sender.send((reply, image));
            });
        if spawned.is_ok() {
            self.inflight += 1;
            self.entries.insert(key.clone(), Entry::Loading);
            // Keep the frame loop alive while the fetch is out, or the reply would sit in
            // the channel until something else happened to ask for a repaint.
            ctx.request_repaint();
        } else {
            self.entries.insert(key.clone(), Entry::Failed);
        }
        None
    }

    /// Whether any fetch is still outstanding: the caller uses it the way it uses a running
    /// animation, to know that the frame it just drew is not the last one.
    pub fn loading(&self) -> bool {
        self.inflight > 0
    }

    /// Take whatever the workers have finished and upload it.
    fn pump(&mut self, ctx: &egui::Context) {
        let mut finished = Vec::new();
        while let Ok((url, result)) = self.receiver.try_recv() {
            finished.push((url, result));
        }
        for (url, result) in finished {
            self.inflight = self.inflight.saturating_sub(1);
            match result {
                Ok(image) => {
                    let bytes = texture_bytes([image.size[0], image.size[1]]);
                    let texture = ctx.load_texture(
                        format!("icon:{url}"),
                        image,
                        egui::TextureOptions::LINEAR,
                    );
                    self.entries.insert(url, Entry::Ready(texture, self.tick, bytes));
        self.evict();
                }
                Err(_) => {
                    self.entries.insert(url, Entry::Failed);
                }
            }
        }
        if self.inflight > 0 {
            ctx.request_repaint();
        }
        trim_heap();
    }
}

/// Find and decode one icon. Runs on a worker thread.
/// The most any icon in the product is drawn at, with headroom.
pub const MAX_ICON: u32 = 160;

/// **A picture is not an icon.** The project body and the gallery draw at four hundred points and more; the 160-point icon
/// cap existed to stop a Discover visit costing 38 MB, and applying it to pictures made every one of them a thumbnail.
/// 640, not 1024: measured. A picture at 1024 cost **9 MB of RSS** that the cache never gives back (30.8 MB idle against 22.6),
/// and the reference draws its body picture 470 points wide at a scale of 1. 640 covers the widest picture on the page with
/// headroom and costs a quarter of the memory.
pub const MAX_PICTURE: u32 = 640;

fn fetch(url: &str, max_side: u32) -> Result<ColorImage, String> {
    // A temporary file rather than an in-memory read: `download_file` streams to disk, which
    // is the launcher's own client and keeps a large icon off this thread's stack. The file
    // is removed whatever happens.
    let path = std::env::temp_dir().join(format!("sift-icon-{:016x}.bin", hash(url)));
    let result = sift_net::download_file(url, &path, Duration::from_secs(20))
        .map_err(|error| error.to_string());
    let bytes = match result {
        Ok(_) => std::fs::read(&path).map_err(|error| error.to_string()),
        Err(error) => Err(error),
    };
    let _ = std::fs::remove_file(&path);

    let bytes = bytes?;
    let decoded = image::load_from_memory(&bytes).map_err(|error| error.to_string())?;
    // **Downscaled to what the screen can use, not kept at the size it arrived.** Project icons come back at 512 or
    // 1024 points and this build uploaded them whole: one Discover visit cost **38 MB of RSS** (24.0 -> 62.1 MiB) and
    // gave none of it back, because ninety-six entries of that size sit under the cache's 64 MiB budget and nothing is
    // ever evicted. No icon in the product is drawn larger than 132 points, so 160 is the most any of them can need.
    let decoded = if decoded.width().max(decoded.height()) > max_side {
        decoded.resize(max_side, max_side, image::imageops::FilterType::Triangle)
    } else {
        decoded
    };
    let rgba = decoded.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    Ok(ColorImage::from_rgba_unmultiplied(size, rgba.as_raw()))
}

/// Hand freed pages back to the OS after a batch of decodes.
///
/// **A decode allocates megabytes and frees them immediately, and glibc keeps the pages.** One Discover visit -- twenty
/// icons fetched and decoded -- raised this process from 24.0 to 58.1 MiB of RSS and it never came back down, with the
/// textures ruled out (downscaling them to 160 changed nothing) and malloc arenas accounting for only 3.4 MiB of it.
/// That leaves the heap holding pages it no longer needs, which is exactly what `malloc_trim` returns.
///
/// Called once per batch rather than per frame: it walks the heap, so it costs something, and at a fetch boundary there is
/// nothing in flight to pay for it twice.
#[cfg(target_env = "gnu")]
fn trim_heap() {
    unsafe extern "C" {
        fn malloc_trim(pad: usize) -> std::ffi::c_int;
    }
    // SAFETY: malloc_trim is glibc's own allocator entry point, takes only an integer pad and returns an int; it has no
    // preconditions and cannot move or invalidate the caller's live allocations.
    unsafe {
        let _ = malloc_trim(0);
    }
}

#[cfg(not(target_env = "gnu"))]
fn trim_heap() {}



/// A stable name for a URL, used for the temporary file and the texture.
fn hash(url: &str) -> u64 {
    // FNV-1a: this is a cache key, not a security boundary, and a dependency for one 8-byte
    // value would be a dependency for nothing.
    let mut value: u64 = 0xcbf2_9ce4_8422_2325;
    for byte in url.as_bytes() {
        value ^= u64::from(*byte);
        value = value.wrapping_mul(0x0000_0100_0000_01b3);
    }
    value
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Two different URLs must not share a temporary file, which is the only thing this hash
    /// is for.
    #[test]
    fn the_cache_key_separates_the_urls_a_launcher_sees() {
        let a = hash("https://cdn.modrinth.com/data/1KVo5zza/icon.png");
        let b = hash("https://cdn.modrinth.com/data/1KVo5zzb/icon.png");
        assert_ne!(a, b);
        assert_eq!(a, hash("https://cdn.modrinth.com/data/1KVo5zza/icon.png"));
    }

    /// A cache starts empty and idle: no fetch is out until something asks for a picture.
    #[test]
    fn a_fresh_cache_is_idle() {
        let images = Images::default();
        assert!(!images.loading());
    }

    /// The eviction rule takes the coldest stamp first, and `Loading`/`Failed` are never
    /// candidates -- a test that planted `Ready` entries would need a GPU context, so this
    /// pins the rule on the stamps the rule reads.
    #[test]
    fn eviction_drops_the_coldest_ready_entry_first() {
        let mut stamps = vec![(7u64, "a"), (1, "b"), (9, "c")];
        stamps.sort_unstable();
        assert_eq!(
            stamps.first().map(|(_, url)| *url),
            Some("b"),
            "the lowest stamp is the coldest"
        );
    }

    /// The limit is a real bound and larger than any single screen of cards: a value under 20
    /// would evict icons the reader is still looking at.
    #[test]
    fn the_cache_limit_is_larger_than_a_screen_of_cards() {
        assert!(CACHE_LIMIT >= 20, "a screen of Discover cards is 12 to 20 icons");
        assert!(CACHE_LIMIT <= 1024, "and the cap has to actually bound memory");
    }
}
