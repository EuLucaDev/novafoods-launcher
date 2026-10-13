# Nova Foods Launcher app icon

The app icon is the **Nova Foods "A" mark in blue** (workflowind.agcunae.com/apps's favicon), exactly as supplied in
[`docs/brand/novafoods-mark.svg`](../../docs/brand/novafoods-mark.svg). Every file in this
directory is rendered from it by `packaging/icons.sh`.

**These files are an Nova Foods trademark. They are not open source and not covered by Nova Foods
Launcher's MIT or Apache-2.0 license.** Their terms are in
[`docs/brand/LICENSE-brand.txt`](../../docs/brand/LICENSE-brand.txt): use them only as part of
Nova Foods Launcher, never redraw, recolor or crop them, and remove them from forks and modified
versions.

## Files

- `novafoods-launcher.svg`: a copy of `docs/brand/novafoods-mark.svg`; also the hicolor scalable icon.
- `novafoods-launcher-1024.png`: 1024 px render, with extra transparent margin for Apple's icon grid.
- `novafoods-launcher.icns`: macOS bundle icon (`CFBundleIconFile`).
- `novafoods-launcher.ico`: Windows icon, 16–256 px, embedded in the `.exe` by
  `apps/novafoods-launcher/build.rs` and used by the MSI.
- `hicolor/<size>/apps/br.com.novafoods.novafoods-launcher.png` (16–512) and
  `hicolor/scalable/apps/br.com.novafoods.novafoods-launcher.svg`: Linux icon theme.
- The tray / menu-bar icon is the same mark, rendered into `docs/brand/novafoods-mark-black-44.png`
  (macOS template, 22 pt @2x) and `docs/brand/novafoods-mark-64.png` (blue; Windows, Linux, FreeBSD).

## Regenerate

Run `packaging/icons.sh` on any OS. It needs `resvg`; the `.ico` and `.icns` are packed by
`cargo xtask ico`. Never edit these files by hand: change the source only if the Nova Foods Team
supplies a new mark in `docs/brand/`.
