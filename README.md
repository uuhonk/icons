# Codecraft Icons

110 枚用于编程 / IDE / 编辑器 / 游戏引擎 / 代码文件 / 家居 / 自然与通用场景的图标：

- `svg-bare/`：彩色版本，透明背景，适合文件树、工具栏和编辑器 UI；
- `svg-mono/`：无色版本，与彩色版本一一对应，适合单色主题或需要自行着色的场景；
- `tags.json`：每个图标的搜索 tags，使用 `编程语言` tag 标记语言和开发工具；
- `preview.html`：显示图标、名字和 tags，支持彩色／无色切换及全部／编程语言／其他筛选。每个图标都有统一 56 × 56 的低透明度虚线框，方便比较大小。

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
  "tags": ["rust", "languages", "编程语言"]
}
```

筛选直接依据 `tags`，无需额外的 `category` 字段。`编程语言` tag 仅用于语言、框架及开发工具图标（19 枚）；通用控件、普通文件、地形、家居、自然等图标归入“其他”（91 枚）。编辑 `tags.json` 后需同步 `preview.html` 中的内嵌清单，以支持直接打开 HTML 预览。

## 场景与通用图标

以下名称均可用于彩色、单色版本及 Rust `get(name)`；`tags.json` 和预览中包含对应中文 tags，归入“其他”。

原 `settings` 图标已删除，设置用途统一使用 `gear`；其 tags 包含 `settings`、`options`、`preferences` 和“设置”。

| 图标 | 名称 | 图标 | 名称 |
| --- | --- | --- | --- |
| 柜子 | `cabinet` | 椅子 | `chair` |
| 桌子 | `table` | 装饰 | `decoration` |
| 齿轮 | `gear` | 房间 | `room` |
| 地板 | `floor` | 墙 | `wall` |
| 窗 | `window` | 玻璃 | `glass` |
| 空调 | `air-conditioner` | 床 | `bed` |
| 时钟 | `clock` | 猫咪 | `cat` |
| 狗狗 | `dog` | 植物 | `plant` |
| 盆栽 | `potted-plant` | 枕头 | `pillow` |
| 花洒（tags：花洒、卫浴） | `shower` | 马桶 | `toilet` |
| 镜子 | `mirror` | 地毯 | `rug` |
| 书本 | `book` | 电脑 | `computer` |
| 灯（落地灯） | `lamp` | 灯泡 | `light-bulb` |
| 台灯 | `desk-lamp` | 沙发 | `sofa` |
| 煤气灶 | `gas-stove` | 烤箱 | `oven` |
| 微波炉 | `microwave` | 水龙头 | `faucet` |
| 刀具 | `knife` | 面包 | `bread` |
| 盘子 | `plate` | 碗 | `bowl` |
| 餐叉 | `fork` | 太阳 | `sun` |
| 月亮 | `moon` | 星星 | `star` |
| 山 | `mountain` | 河 | `river` |
| 水 | `water` | 火 | `fire` |
| 冰 | `ice` | 雷 | `lightning` |
| 树木 | `tree` | 花朵 | `flower` |
| 石头 | `rock` | 上箭头 | `arrow-up` |
| 下箭头 | `arrow-down` | 左箭头 | `arrow-left` |
| 右箭头 | `arrow-right` | | |

## 文件结构

```text
.
├── svg-bare/                  # 彩色透明 SVG（当前图标库）
├── svg-mono/                  # 无色 SVG（与 svg-bare 一一对应）
├── svg/                       # 旧带底板 SVG（兼容保留）
├── tags.json                  # 每个图标的 tags（含编程语言 tag）
├── preview.html               # 图标 / 名字 / tags、配色切换与分类筛选
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
- **尺寸统一**：彩色及单色版本按实际可见内容的 alpha 包围盒归一化，包含描边。64 × 64 输出中，可见最长边统一为约 44 px（高分辨率测量误差不超过 0.125 px），包围盒居中；保持长宽比，较短边随原始形状变化。通过调整 SVG `viewBox` 实现，直接引用 SVG 或通过 Rust 使用均生效。旧 `svg/` 带底板版本保留原尺寸。
- **字形**：原有技术与文件类型图标采用实心填充；编辑器工具及新增场景图标采用圆角线条，场景图标沿用 24 单位设计网格、1.6 单位描边，输出尺寸仍为 64 × 64。
- **可访问性**：每个文件内置 `<title>` 与 `aria-labelledby`。
- **可内联**：gradient / mask / title ID 均带图标名，批量内联不会有 ID 冲突。
- **可缩放**：建议展示尺寸 20–64 px；无底板版本在 24 px 以上最清晰。
- **来源**：原有字形来自 Material Symbols、Simple Icons、Devicon、Material Icon Theme，并按各自开源许可重新着色与组合；新增场景与通用字形为原创，采用本项目 MIT 许可。详见下方许可部分。

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
