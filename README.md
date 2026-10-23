# Codecraft Icons

58 枚用于编程 / IDE / 编辑器 / 游戏引擎 / 代码文件场景的图标：

- `svg-bare/`：彩色版本，透明背景，适合文件树、工具栏和编辑器 UI；
- `svg-mono/`：无色版本，与彩色版本一一对应，适合单色主题或需要自行着色的场景；
- `tags.json`：每个图标的搜索 tags；
- `preview.html`：简单预览，只显示图标、名字和 tags，可在彩色／无色之间切换。

> 交互预览：[`preview.html`](preview.html)。旧 `svg/` 带底板版本保留用于兼容，但预览不再显示。

## 快速使用

```html
<img src="svg-bare/rust.svg" alt="Rust" width="32" height="32">
<img src="svg-mono/rust.svg" alt="Rust" width="32" height="32">
```

作为 CSS 背景：

```css
.icon-rust {
  width: 32px;
  height: 32px;
  background: url("svg-bare/rust.svg") center / contain no-repeat;
}
```

图标名称与 tags：

```json
{
  "name": "rust",
  "tags": ["rust", "languages"]
}
```

## 文件结构

```text
.
├── svg-bare/                  # 彩色透明 SVG（当前图标库）
├── svg-mono/                  # 无色 SVG（与 svg-bare 一一对应）
├── svg/                       # 旧带底板 SVG（兼容保留）
├── tags.json                  # 每个图标的 tags
├── preview.html               # 简单预览：图标 / 名字 / tags，彩色／无色切换
├── manifest.json              # 旧带底板清单（兼容保留）
├── rust/                      # uuhonk_icons crate + build.rs + rust-toolchain.toml
├── README.md
├── THIRD_PARTY_NOTICES.md
├── LICENSE
└── LICENSES/                  # 第三方许可证原文
```


## Rust crate

`rust/` 是图标库自带的 `uuhonk_icons` crate；图标库根目录不需要 `Cargo.toml`。`rust/build.rs` 读取 `../svg-bare/` 并生成 `OUT_DIR/icons.rs`，构建时不访问网络；可用 `UUHONK_ICONS_SVG_DIR` 覆盖 SVG 目录。

```toml
uuhonk_icons = { git = "https://github.com/uuhonk/icons", package = "uuhonk_icons", rev = "..." }
```

crate 只提供名字和 SVG 字符串（`Icon`、`ICONS`、`get`、`names`）；tags、彩色／无色预览仍由本目录维护。

## 设计规格

- **画布**：统一 64 × 64，透明外边缘。
- **带底板**：56 × 56 底板，`rx="18"`，对角双色渐变；字形为白色或高对比深色。
- **无底板**：无底板矩形，透明背景；字形为纯色实心填充，Python 保留蓝/黄双色，Node.js 为绿色六边形 + 透明 JS 镂空。
- **尺寸统一**：方形 / 近方形图标的无底板包围盒约 42 × 42；宽幅图形按 44–48 px 宽归一化，高度随比例变化，避免大小忽大忽小。
- **全实心**：没有依赖描边的线性图形；所有图标均为填充形状。
- **可访问性**：每个文件内置 `<title>` 与 `aria-labelledby`。
- **可内联**：gradient / mask / title ID 均带图标名，批量内联不会有 ID 冲突。
- **可缩放**：建议展示尺寸 20–64 px；无底板版本在 24 px 以上最清晰。
- **来源**：字形来自 Material Symbols、Simple Icons、Devicon、Material Icon Theme，并按各自开源许可重新着色与组合；详见下方许可部分。

## 修改颜色

彩色版本直接修改 `svg-bare/<name>.svg` 中的 `fill` / `stroke`。无色版本是 `svg-bare/` 的副本，所有颜色统一为 `#eeeeee`；修改彩色图标后需要同步更新 `svg-mono/`。

`preview.html` 会直接读取 `svg-bare/` 和 `svg-mono/`，不需要额外的颜色映射文件。

## 来源与许可

图标字形来自以下开源项目，并在本套图标中进行了重新着色、缩放、居中、透明化与圆角底板组合等修改：

- [Material Symbols Rounded](https://fonts.google.com/icons) — Apache License 2.0
- [Simple Icons](https://simpleicons.org/) — CC0 1.0
- [Devicon](https://devicon.dev/) — MIT
- [Material Icon Theme](https://github.com/material-extensions/vscode-material-icon-theme) — MIT（Shader 图标）

本套组合、配色、双版本系统、Node.js 实心镂空处理、预览与文档以 MIT 发布。第三方字形仍遵循各自许可证；完整说明与许可证原文见 [`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md) 和 [`LICENSES/`](LICENSES/)。

所有品牌名称与商标归各自所有者所有，此处仅用于表示文件类型或技术栈。
