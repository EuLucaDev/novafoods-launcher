//! Bundled images: each Nova Foods App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Nova Foods App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "novaphoto" => egui::include_image!("../../../assets/app-icons/novaphoto.png"),
        "novavector" => egui::include_image!("../../../assets/app-icons/novavector.png"),
        "novafilm" => egui::include_image!("../../../assets/app-icons/novafilm.png"),
        "novalight" => egui::include_image!("../../../assets/app-icons/novalight.png"),
        "novapdf" => egui::include_image!("../../../assets/app-icons/novapdf.png"),
        "novaeffect" => egui::include_image!("../../../assets/app-icons/novaeffect.png"),
        "novadesign" => egui::include_image!("../../../assets/app-icons/novadesign.png"),
        "novafig" => egui::include_image!("../../../assets/app-icons/novafig.png"),
        "novaword" => egui::include_image!("../../../assets/app-icons/novaword.png"),
        "novadeck" => egui::include_image!("../../../assets/app-icons/novadeck.png"),
        "novagrid" => egui::include_image!("../../../assets/app-icons/novagrid.png"),
        "novacad" => egui::include_image!("../../../assets/app-icons/novacad.png"),
        "novasound" => egui::include_image!("../../../assets/app-icons/novasound.png"),
        _ => return None,
    })
}

/// The Nova Foods wordmark, as supplied: dark ink for light backgrounds, white for dark ones.
/// An Nova Foods trademark, not covered by the MIT/Apache license (`docs/brand/LICENSE-brand.txt`).
pub fn novafoods_logo(dark_background: bool) -> ImageSource<'static> {
    if dark_background {
        egui::include_image!("../../../docs/brand/novafoods-logo-white.svg")
    } else {
        egui::include_image!("../../../docs/brand/novafoods-logo.svg")
    }
}

/// The Nova Foods logo's width / height (its SVG view box: 1712.05 × 735.57).
pub const NOVAFOODS_LOGO_ASPECT: f32 = 1712.05 / 735.57;

/// A social service's logo (Simple Icons, white, for tinting), by its name in `catalog::SOCIAL`.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    Some(match service {
        "X" => egui::include_image!("../../../assets/social/x.svg"),
        "YouTube" => egui::include_image!("../../../assets/social/youtube.svg"),
        "TikTok" => egui::include_image!("../../../assets/social/tiktok.svg"),
        "Instagram" => egui::include_image!("../../../assets/social/instagram.svg"),
        "LinkedIn" => egui::include_image!("../../../assets/social/linkedin.svg"),
        _ => return None,
    })
}

/// The Nova Foods "A" mark in blue, as supplied: the launcher's app icon and workflowind.agcunae.com/apps's favicon.
pub fn novafoods_mark() -> ImageSource<'static> {
    egui::include_image!("../../../docs/brand/novafoods-mark.svg")
}
