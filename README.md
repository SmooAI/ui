# @smooai/ui

SmooAI's **cross-language design system** — design tokens, base CSS, and the smoo monogram, shared across every SmooAI app: TypeScript (web), Rust (desktop), and eventually .NET, Python, Go.

## Why a multi-language repo

SmooAI is already a multi-runtime company:

- `apps/web` — Next.js / React / Tailwind (TypeScript)
- `smooblue` — Dioxus desktop client (Rust)
- `observability-studio` — Dioxus desktop viewer (Rust)
- future agents and CLIs in Python / .NET / Go

Without a single source of truth, "smoo green" drifts silently across runtimes. This repo is that source of truth. The canonical files live under [`shared/`](shared/); per-language bindings wrap them.

```
~/dev/smooai/ui/
├── shared/                # language-agnostic source of truth
│   ├── styles.css         # canonical OKLCH tokens + base component CSS
│   ├── monogram.svg       # smoo monogram
│   └── tokens.json        # tokens as plain JSON, for any language to import
│
├── rust/                  # smooai-ui (crates.io)
│   ├── Cargo.toml
│   ├── src/lib.rs         # pub const STYLES / MONOGRAM_SVG / tokens::*
│   └── tests              # validates Rust consts match shared/styles.css
│
├── src/                   # @smooai/ui (npm) — TS, future
├── dotnet/                # SmooAI.Ui (NuGet) — future
├── python/                # smooai-ui (PyPI) — future
└── go/                    # github.com/SmooAI/ui/go — future
```

Every language binding embeds `shared/styles.css` + `shared/monogram.svg` at build time. The Rust crate does this via `include_str!`; the TS package does it via a `?inline` Vite import or a build-step copy; etc.

## Status

| Language | Package | Status |
| --- | --- | --- |
| **Rust** | `smooai-ui` (crate) | ✅ Shipped — consumed by `observability-studio` |
| **TypeScript** | `@smooai/ui` (npm) | 🚧 Lives today inside the `SmooAI/smooai` monorepo at `packages/ui`; will graduate here |
| **.NET** | `SmooAI.Ui` (NuGet) | 📦 Planned |
| **Python** | `smooai-ui` (PyPI) | 📦 Planned |
| **Go** | `github.com/SmooAI/ui/go` | 📦 Planned |

## Rust quickstart

```toml
# Cargo.toml
[dependencies]
smooai-ui = "0.1"
```

```rust
use dioxus::prelude::*;

fn App() -> Element {
    rsx! {
        // Inject the canonical brand stylesheet once at the root component.
        style { "{smooai_ui::STYLES}" }

        // Use the BEM classes everywhere.
        div { class: "card",
            button { class: "btn btn--primary", "Save" }
        }

        // Drop the monogram into your brand badge.
        div {
            class: "brand-badge",
            style: "width:32px;height:32px;",
            dangerous_inner_html: "{smooai_ui::MONOGRAM_SVG}",
        }
    }
}
```

For non-DOM frameworks (egui, iced, native menus), reference token values directly:

```rust
let accent = smooai_ui::tokens::SMOOAI_GREEN; // "oklch(0.657 0.112 194.8)"
```

## What's in `shared/styles.css`

- **OKLCH brand palette** — `--color-smooai-orange`, `--color-smooai-red`, `--color-smooai-green`, the blue scale, the dark-blue scale
- **Semantic tokens** — `--background`, `--foreground`, `--card`, `--muted`, `--border`, `--ring`, `--sidebar`
- **Brand gradient** — `--gradient-brand` (the signature orange→red 135° gradient)
- **Geometry** — `--radius`, spacing scale, type stack
- **Base components** — `.btn` family, `.card`, `.fab`, `.modal__sheet`, `.rail`, `.brand-badge`, action-icon hover colors
- **Reset + base + scrollbars**

Dark mode is the only mode. Always reference a token, never hardcode hex.

## Versioning

Per-language packages share the same semver line so consumers can correlate versions across runtimes.

| Bump | Triggers |
| --- | --- |
| **Patch** | Token value tweaks, CSS rule additions, bug fixes |
| **Minor** | New tokens, new component classes, new monogram variants — additive only |
| **Major** | Token renames, removed classes, breaking layout assumptions |

## Contributing

PRs welcome. Keep this surface narrow — only add a token / class when at least two apps need it. Run `cargo test -p smooai-ui` to validate Rust constants match `shared/styles.css`. Future language bindings should add an equivalent drift-detector test.
