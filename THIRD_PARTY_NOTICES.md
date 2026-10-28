# Third-Party Notices

This notice applies to both variants of the set: `svg/` (with tile) and `svg-bare/` (transparent, no tile).

This set combines and modifies vector glyphs from several open-source icon projects. The table below records the source, license, and the modifications made for this set. All trademarks remain the property of their respective owners; the icons are used only to identify file types, languages, tools, and technologies.

Full license texts are included in [`LICENSES/`](LICENSES/).

## Material Symbols Rounded

- **Project:** Material Symbols, Google Fonts — <https://fonts.google.com/icons>
- **License:** Apache License 2.0
- **License copy:** [`LICENSES/Apache-2.0.txt`](LICENSES/Apache-2.0.txt)
- **Used for:** `folder`, `image`, `audio`, `video`, `model-3d`, `document`, `json`, `terminal`, `database`, `package`, `game-engine`, `config`, `code`
- **Modifications:** extracted the rounded filled SVG glyph; recolored it; scaled and centered it by its actual bounding box; placed it on a new rounded gradient tile.

## Simple Icons

- **Project:** Simple Icons — <https://simpleicons.org/> · <https://github.com/simple-icons/simple-icons>
- **License:** CC0 1.0 Universal (public domain dedication)
- **License copy:** [`LICENSES/CC0-1.0.txt`](LICENSES/CC0-1.0.txt)
- **Used for:** `rust`, `javascript` (JS letterforms), `typescript` (TS letterforms), `c`, `cpp`, `git`, `markdown`, `html`, `css` (CSS letterforms), `react`, `docker`, `yaml`, `toml`, and the `JS` counterform inside `node`
- **Modifications:** recolored; extracted letterforms from tile-shaped marks where needed; scaled, centered, and placed on new rounded gradient tiles. CC0 does not require attribution, but the source is credited here as good practice.

## Devicon

- **Project:** Devicon — <https://devicon.dev/> · <https://github.com/devicons/devicon>
- **License:** MIT — Copyright (c) 2015 konpa
- **License copy:** [`LICENSES/MIT-Devicon.txt`](LICENSES/MIT-Devicon.txt)
- **Used for:** `python` (two-tone mark), `csharp` (C# counterform), `node` (solid hexagon base)
- **Modifications:** recolored the Python paths to white/yellow; extracted the white C# path; simplified the Node hexagon to a single solid shape and masked the JS letters out of it; scaled and centered on new rounded gradient tiles.

## Material Icon Theme

- **Project:** Material Icon Theme — <https://github.com/material-extensions/vscode-material-icon-theme>
- **License:** MIT — Copyright (c) 2025 Material Extensions
- **License copy:** [`LICENSES/MIT-Material-Icon-Theme.txt`](LICENSES/MIT-Material-Icon-Theme.txt)
- **Used for:** `shader`
- **Modifications:** recolored; scaled and centered; placed on a new rounded gradient tile.

## Original additions

The following are original to this set and released under the MIT license in [`LICENSE`](LICENSE):

- the home and interior glyphs: `cabinet`, `chair`, `table`, `decoration`, `gear`, `room`, `floor`, `wall`, `window`, `glass`, `air-conditioner`, `bed`, `clock`, `cat`, `dog`, `plant`, `potted-plant`, and `pillow`, in both `svg-bare/` and `svg-mono/`;
- the bathroom, household, food, nature, and arrow glyphs: `shower`, `toilet`, `mirror`, `rug`, `book`, `computer`, `lamp`, `light-bulb`, `desk-lamp`, `sofa`, `gas-stove`, `oven`, `microwave`, `faucet`, `knife`, `bread`, `plate`, `bowl`, `fork`, `sun`, `moon`, `star`, `mountain`, `river`, `water`, `fire`, `ice`, `lightning`, `tree`, `flower`, `rock`, `arrow-up`, `arrow-down`, `arrow-left`, and `arrow-right`, in both `svg-bare/` and `svg-mono/`;
- the 64 × 64 rounded-tile system, gradient color selection, and layout;
- the transparent, tile-free color system and consistent optical sizing across both variants;
- glyph centering, scaling, and composition;
- the solid Node.js hexagon + JS cut-out treatment;
- `preview.html`, `preview.png`, `manifest.json`, and documentation.

## Trademarks

Rust, Python, JavaScript, TypeScript, C, C++, C#, .NET, JSON, YAML, TOML, Markdown, HTML, CSS, React, Node.js, Docker, Git, and other names and logos are trademarks or registered trademarks of their respective owners. Their use here is nominative and does not imply endorsement by or affiliation with those owners.

If you redistribute these icons, keep this notice and the applicable license texts with the files.
