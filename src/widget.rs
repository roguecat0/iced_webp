pub mod webp;

pub use webp::Webp;

/// Creates a new [`Gif`] with the given [`gif::Frames`]
pub fn webp(frames: &webp::Frames) -> Webp {
    Webp::new(frames)
}
