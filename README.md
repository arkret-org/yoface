# yoface

Cokret 共享前端组件库。所有 Cokret 的 Dioxus 前端(yougen、sodmin 等)统一依赖
本 crate:它**透传**官方 `dioxus-primitives` / `dioxus-icons`,并收纳一组以
`#[css_module]` 封装、统一走设计令牌着色的自定义控件。

- Dioxus 版本:**0.7.9**
- 基准实现:yougen 的 `src/ui/`(`#[css_module]` + `dioxus_primitives` 封装)

---

## 1. 锁定的 fork rev(单一真相源)

yoface 与所有下游必须钉死同一对 fork revision。**升级时只在本节手工 bump**,
然后同步到所有下游的 `[patch]` 段(见 §2)。

| 用途 | git 源 | rev |
| --- | --- | --- |
| dioxus core(及 generational-box / subsecond) | `https://github.com/cokret/dioxus` | `e59f9f24a5c27c9303cd61edd0452b44161374ab` |
| dioxus components(dioxus-primitives) | `https://github.com/cokret/dioxus-components` | `3510aeee2d14f0ca2c11682c9b826650cb557d2f` |

为什么必须 fork:

- **core fork**:upstream 0.7.9 的 `Callback::__point_to` 对 `point_to` 调
  `.unwrap()`,当 `Option<EventHandler>` prop 的 owner scope 在同帧被回收时整个
  wasm 实例 abort(白屏)。fork 把该 unwrap 降级为 trace-log no-op。
- **components fork**:`focus-trap.js` 加了幂等守卫,避免 `DialogRoot` 跨 mount
  重复注入脚本时顶层 `class FocusTrap` 重声明抛 `SyntaxError`,进而腐蚀
  focus-trap eval 闭包导致 wasm 堆损坏。
- **同源约束**:git `dioxus-core` 会经 workspace deps 拉入 git
  `generational-box` / `subsecond`。若 registry 副本仍在,同一 crate 会存在于两个
  source 下,`GenerationalBox<T>` 在 registry `dioxus-signals` / `dioxus-web` 与
  patched core 之间 **mismatch**。所以下面 5 个 crate 必须一并 patch 到同一 rev。

---

## 2. 下游必须复制的 `[patch]` 清单 ⚠️

**`[patch]` 段不被依赖继承** —— 它只对 workspace 根的 Cargo.toml 生效。yoface 自带
这些 patch 是为了自身能独立编译;**下游每个前端的 Cargo.toml(workspace 根)必须
原样复制以下两段**,否则 `GenerationalBox<T>` 跨源 mismatch、编译/运行报错。

```toml
[patch."https://github.com/DioxusLabs/components"]
dioxus-primitives = { git = "https://github.com/cokret/dioxus-components", rev = "3510aeee2d14f0ca2c11682c9b826650cb557d2f" }

[patch.crates-io]
dioxus-core       = { git = "https://github.com/cokret/dioxus", rev = "e59f9f24a5c27c9303cd61edd0452b44161374ab" }
dioxus-core-types = { git = "https://github.com/cokret/dioxus", rev = "e59f9f24a5c27c9303cd61edd0452b44161374ab" }
generational-box  = { git = "https://github.com/cokret/dioxus", rev = "e59f9f24a5c27c9303cd61edd0452b44161374ab" }
subsecond         = { git = "https://github.com/cokret/dioxus", rev = "e59f9f24a5c27c9303cd61edd0452b44161374ab" }
subsecond-types   = { git = "https://github.com/cokret/dioxus", rev = "e59f9f24a5c27c9303cd61edd0452b44161374ab" }
```

依赖声明示例(下游 Cargo.toml):

```toml
[dependencies]
yoface = { path = "../yoface" }            # 或 git
dioxus = "0.7.9"
# 下游按 target 启用 dioxus 的 desktop / web feature,与 yougen 现状一致
```

---

## 3. 已收纳组件清单

### 3.1 官方原语透传(直接用,无封装)

```rust
use yoface::dioxus_primitives;   // accordion / dropdown / tooltip / popover ... 全部官方原语
use yoface::dioxus_icons;        // lucide 等图标:yoface::dioxus_icons::lucide::Inbox
```

> 优先用官方原语 + `dioxus_icons` 图标,不要手写 SVG / 图标注册表。

### 3.2 yoface 自带封装(`yoface::ui::*`)

dioxus-primitives 之上的 Cokret `#[css_module]` 封装(自 yougen `src/ui` 原样迁入):

| 模块 | 组件 |
| --- | --- |
| `ui::badge` | `Badge` / `BadgeVariant` / `VerifiedIcon` |
| `ui::button` | `Button` / `ButtonVariant` / `ButtonSize` |
| `ui::card` | `Card` / `CardHeader` / `CardTitle` / `CardDescription` / `CardAction` / `CardContent` / `CardFooter` |
| `ui::checkbox` | `Checkbox` |
| `ui::dialog` | `Dialog` / `DialogTitle` / `DialogDescription` |
| `ui::input` | `Input` |
| `ui::label` | `Label` |
| `ui::select` | `Select` / `SelectMulti` / `SelectOption` / `SelectGroup` / `SelectGroupLabel` |
| `ui::separator` | `Separator` |
| `ui::slider` | `Slider` / `RangeSlider` |
| `ui::switch` | `Switch` |
| `ui::tabs` | `Tabs` / `TabList` / `TabTrigger` / `TabContent` / `TabsVariant` |
| `ui::textarea` | `Textarea` / `TextareaVariant` |

后台实用控件(官方原语没有,自 sodmin 收纳并改写为 `#[css_module]`):

| 模块 | 组件 |
| --- | --- |
| `ui::table` | `Table` / `TableHeader` / `TableBody` / `TableRow` / `TableHead` / `TableCell` / `EmptyRow` |
| `ui::pagination` | `Pagination` / `CursorPagination` |
| `ui::toast` | `Toaster` / `show_toast` / `show_toast_with_action` / `Toast` / `ToastVariant` / `ToastAction` |
| `ui::loading` | `LoadingSkeleton` / `StatsSkeleton` / `Spinner` |
| `ui::page_header` | `PageHeader` / `Breadcrumbs` / `BreadcrumbItem` |
| `ui::info_row` | `InfoRow` |
| `ui::error_banner` | `ErrorBanner` |
| `ui::empty_state` | `EmptyState` |

> 收纳时去除的业务耦合(下游接入注意):
> - `error_banner`:原依赖 i18n `t` / telemetry / `HttpError`。改为纯展示组件,
>   `error_label` / `retry_label` / `detail` 等文案由调用方传入;埋点在调用处自理。
> - `empty_state`:`icon` 改为可选 `Element`(传 `yoface::dioxus_icons` 图标),
>   动作改为普通 `<a href>`。
> - `page_header::Breadcrumbs`:`BreadcrumbItem.href: Option<String>` 取代强类型
>   `Route`。下游用 dioxus-router 时在外层把 `Route` 渲染成 `href` 字符串。
> - `pagination`:复用 `yoface::ui::button`。文案默认英文,可经 props 覆盖。
> - `dialog`:去除 yougen 的 `crate::api::sleep_for` 业务调用,改为即时清除
>   backdrop 抑制标志(`mousedown`/`click` 的 `stop_propagation` 已足够防误关)。

### 3.3 用法示例

```rust
use dioxus::prelude::*;
use yoface::ui::button::{Button, ButtonVariant};
use yoface::ui::table::{Table, TableHeader, TableBody, TableRow, TableHead, TableCell};
use yoface::ui::toast::{Toaster, show_toast, ToastVariant};
use yoface::dioxus_icons::lucide::Inbox;
use yoface::ui::empty_state::EmptyState;

fn App() -> Element {
    rsx! {
        // 令牌一次性注入(见 §4)
        document::Style { {yoface::TOKENS_CSS} }
        // 全局 toast 容器,挂一次即可
        Toaster {}

        Button {
            variant: ButtonVariant::Primary,
            onclick: move |_| show_toast("已保存", ToastVariant::Success),
            "保存"
        }

        Table {
            TableHeader { TableRow { TableHead { "名称" } TableHead { "状态" } } }
            TableBody { TableRow { TableCell { "示例" } TableCell { "在线" } } }
        }

        EmptyState {
            icon: rsx! { Inbox {} },
            title: "暂无数据".to_string(),
            description: "这里还没有任何条目。".to_string(),
        }
    }
}
```

---

## 4. 设计令牌与下游换肤

所有组件颜色 / 半径 / 阴影只引用 `src/tokens.css` 的 CSS 变量,不写死颜色。

- **第一层(shadcn 命名,下游覆盖入口)**:`--background` / `--foreground` /
  `--primary` / `--primary-foreground` / `--primary-2` / `--primary-strong` /
  `--secondary` / `--muted` / `--muted-foreground` / `--accent` / `--destructive` /
  `--success` / `--border` / `--input` / `--ring` / `--radius` / `--shadow-*` …
- **第二层(兼容别名)**:`--primary-color-N` / `--secondary-color-N` /
  `--primary-error-color` / `--focused-border-color` 等,全部指向第一层,
  不持有独立色值。这是 yougen 封装迁入的桥接层,改主题只需动第一层。
- **明 / 暗**:沿用 dioxus-components 的 `--light` / `--dark` 双值开关。
  默认跟随 `prefers-color-scheme`;也可在根元素显式设 `data-theme="light"` /
  `data-theme="dark"` 强制。

### 注入令牌

`tokens.css` 定义的是全局 `:root` 变量,**不走 `#[css_module]`**。下游二选一:

1. 注入 yoface 默认令牌:
   ```rust
   rsx! { document::Style { {yoface::TOKENS_CSS} } }
   ```
2. 提供等价的同名令牌(下游已有自己的 design.css 时,直接定义第一层语义令牌
   即可,组件会自动取用)。

### 覆盖主题

只覆盖第一层语义令牌即可全局换肤,例如把品牌主色改成蓝:

```css
:root {
  --primary:            var(--dark, #6aa9ff) var(--light, #2563eb);
  --primary-strong:     var(--dark, #93c1ff) var(--light, #1d4ed8);
  --primary-2:          var(--dark, #4f8ff0) var(--light, #3b82f6);
  --primary-foreground: var(--dark, #0b1220) var(--light, #ffffff);
}
```

---

## 5. 编译验证

```bash
cargo check                                   # host
cargo check  --target wasm32-unknown-unknown  # wasm
cargo build  --target wasm32-unknown-unknown  # wasm
```

本仓三者均已验证通过(dioxus 0.7.9,上述 fork rev)。
