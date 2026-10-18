//! The Nova Foods Apps: the thirteen apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`novaphoto`).
    pub id: &'static str,
    /// Display name (`Nova Photo`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    /// Brand colour (sRGB), for surfaces: workflowind.agcunae.com/apps's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older machine names a Nova Foods release may carry.
    pub former_ids: &'static [&'static str],
    /// Older display names registered by Nova Foods installers.
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `br.com.novafoods.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("br.com.novafoods.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's page on workflowind.agcunae.com/apps.
    pub fn website(&self) -> String {
        format!("https://workflowind.agcunae.com/apps/{}", self.id)
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// Os apps Nova Foods, na ordem de exibição: os sete clássicos e depois os seis novos.
pub const SUITE: [CraftApp; 13] = [
    app("novaphoto", "Nova Photo", "Image editing", [[0x00, 0x6d, 0xff], [0x00, 0x52, 0xc2], [0x5f, 0xa8, 0xff]], "EuLucaDev/novaphoto"),
    app("novavector", "Nova Vector", "Vector illustration", [[0xf2, 0x48, 0x2b], [0xb8, 0x32, 0x1a], [0xff, 0x8a, 0x4c]], "EuLucaDev/novavector"),
    app(
        "novafilm",
        "Nova Film",
        "Video editing, color and sound",
        [[0x5b, 0x3b, 0xf0], [0x44, 0x26, 0xc4], [0x9b, 0x7b, 0xff]],
        "EuLucaDev/novafilm",
    ),
    app(
        "novalight",
        "Nova Light",
        "Photo library and raw development",
        [[0xee, 0x8a, 0x03], [0x8a, 0x50, 0x00], [0xff, 0xc8, 0x45]],
        "EuLucaDev/novalight",
    ),
    app(
        "novapdf",
        "Nova PDF",
        "Reading, organizing and protecting PDFs",
        [[0xd6, 0x28, 0x39], [0xa3, 0x18, 0x28], [0xff, 0x6b, 0x6b]],
        "EuLucaDev/novapdf",
    ),
    app(
        "novaeffect",
        "Nova Effect",
        "Motion graphics and visual effects",
        [[0xb5, 0x26, 0x7e], [0x8c, 0x1a, 0x61], [0xf0, 0x62, 0xc0]],
        "EuLucaDev/novaeffect",
    ),
    app(
        "novadesign",
        "Nova Design",
        "Page layout and publishing",
        [[0x3e, 0x9a, 0x3a], [0x2c, 0x6f, 0x29], [0x8b, 0xd1, 0x50]],
        "EuLucaDev/novadesign",
    ),
    app("novafig", "Nova Fig", "Interface design and prototyping", [[0x00, 0xa3, 0x89], [0x00, 0x75, 0x63], [0x3d, 0xe0, 0xc4]], "EuLucaDev/novafig"),
    app("novaword", "Nova Word", "Writing and document design", [[0x1e, 0x25, 0xab], [0x1e, 0x25, 0xab], [0x4c, 0x6f, 0xff]], "EuLucaDev/novaword"),
    app("novadeck", "Nova Deck", "Presentations and slide shows", [[0xee, 0x62, 0x16], [0xa8, 0x44, 0x08], [0xff, 0x9a, 0x3d]], "EuLucaDev/novadeck"),
    app("novagrid", "Nova Grid", "Spreadsheets and data", [[0x0b, 0x8a, 0x4a], [0x0b, 0x6b, 0x3a], [0x34, 0xc7, 0x7b]], "EuLucaDev/novagrid"),
    app(
        "novacad",
        "Nova CAD",
        "Computer-aided design and drafting",
        [[0x2b, 0x36, 0x47], [0x2b, 0x36, 0x47], [0x8a, 0x9a, 0xb0]],
        "EuLucaDev/novacad",
    ),
    app(
        "novasound",
        "Nova Sound",
        "Recording, editing and mixing audio",
        [[0x1a, 0x98, 0xc4], [0x0e, 0x6f, 0x91], [0x5f, 0xd3, 0xf0]],
        "EuLucaDev/novasound",
    ),
];

/// Nova Foods's website.
pub const WEBSITE_URL: &str = "https://workflowind.agcunae.com/apps";
/// The GitHub account every Nova Foods App's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
pub const GITHUB_OWNER: &str = "EuLucaDev";
/// Nova Foods on GitHub: every Nova Foods App's source.
pub const GITHUB_URL: &str = "https://github.com/EuLucaDev";
/// Nova Foods's social accounts: (service, URL). Shown as small icons, less prominent than Suporte.
pub const SOCIAL: [(&str, &str); 0] = [];

/// The Nova Foods community on Suporte.
pub const DISCORD_URL: &str = "https://workflowind.agcunae.com/inbox";
/// O suporte é a caixa de entrada do WorkFlow.ind, não um servidor do Discord: sem link para o app.
pub const DISCORD_APP_URL: &str = DISCORD_URL;

/// Nova Foods Launcher itself, described like a Nova Foods App so it can update itself with the same
/// machinery (asset names `novafoods-launcher-<ver>-…`, bundle id `br.com.novafoods.novafoods-launcher`,
/// `Nova Foods Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "novafoods-launcher",
    "Nova Foods Launcher",
    "Installs and updates the Nova Foods Apps",
    [[0x0b, 0x0b, 0x0c], [0x0b, 0x0b, 0x0c], [0xf2, 0xf1, 0xee]],
    "EuLucaDev/novafoods-launcher",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "EuLucaDev/novafoods-launcher";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "novafoods-launcher";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_thirteen_apps_with_unique_ids() {
        assert_eq!(SUITE.len(), 13);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("EuLucaDev/{}", a.id));
            assert_eq!(a.name.to_lowercase().replace(' ', ""), a.id);
        }
    }

    #[test]
    fn novapdf_has_its_site_page() {
        let pdf = CraftApp::by_id("novapdf").unwrap();
        assert_eq!(pdf.website(), "https://workflowind.agcunae.com/apps/novapdf");
        assert_eq!(CraftApp::by_id("novaword").unwrap().website(), "https://workflowind.agcunae.com/apps/novaword");
    }
}
