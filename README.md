# CC-GEN v2.0 — Credit Card Generator for Testing

> **⚠️ DISCLAIMER:** Generated card numbers are for **testing and development purposes only**. They are mathematically valid (pass Luhn check) but are NOT real credit cards. Using generated numbers for fraud is **illegal**.

High-performance credit card number generator powered by **Rust** and **WebAssembly**. Generates ISO/IEC 7812 compliant test card numbers with full Luhn validation, BIN database lookup, and a modern glassmorphism UI.

## Features

- **14 Card Brands**: Visa, MasterCard, Amex, Discover, Diners Club, JCB, UnionPay, Maestro, Mir, RuPay, Verve, UATP, Dankort, InterPayment
- **ISO 7812 Compliance**: Full MII + IIN/BIN validation, not just Luhn
- **Triple Verification**: Forward Luhn + Reverse consistency + Structural validation
- **CSPRNG**: OS-level cryptographically secure random numbers with rejection sampling (zero modulo bias)
- **12 Output Formats**: Pipe, CSV, TSV, JSON, XML, YAML, SQL, Stripe, PayPal, and more
- **BIN Database**: In-memory lookup for card brand, issuer, type, and country
- **Card Validator**: Paste any card number for full validation with confidence score
- **100% Valid**: Every generated card passes Luhn — guaranteed by construction
- **Zero Server Storage**: All generation happens client-side in WASM
- **Modern UI**: Glassmorphism design, 3D card preview, dark/light theme, responsive

## Architecture

```
Rust Core Engine (ccgen-core)
  ├── Luhn + ISO 7812 validator
  ├── BIN pattern engine (x, [0-9], ?, {a,b} syntax)
  ├── CSPRNG with rejection sampling
  ├── BIN database with 200+ entries
  └── 12 output formatters

WebAssembly Bridge (ccgen-wasm)
  └── wasm-bindgen exports for browser

Frontend (HTML/CSS/JS)
  ├── Glassmorphism + 3D card preview
  ├── Dark/Light theme with system detection
  ├── Keyboard shortcuts
  └── BIN lookup + Card validator tools
```

## Quick Start

### Prerequisites

- [Rust](https://rustup.rs/) (stable 2024+)
- [wasm-pack](https://rustwasm.github.io/wasm-pack/installer/)

### Build

```bash
# Install dependencies
rustup target add wasm32-unknown-unknown
cargo install wasm-pack

# Build WASM module
wasm-pack build crates/wasm --target web --out-dir ../../frontend/pkg --out-name ccgen

# Run tests
cargo test
```

### Serve

```bash
# Any static file server works
cd frontend
python3 -m http.server 8080
# Open http://localhost:8080
```

### Run Benchmarks

```bash
cargo bench -p ccgen-core
```

## Performance

| Metric | Result |
|--------|--------|
| 10,000 cards (native) | < 50ms |
| 1,000 cards (WASM) | < 100ms |
| Luhn pass rate | 100% |
| WASM bundle (gzipped) | ~60KB |

## Keyboard Shortcuts

| Shortcut | Action |
|----------|--------|
| `Ctrl+Enter` | Generate |
| `Ctrl+C` | Copy output |
| `Ctrl+S` | Download |
| `Ctrl+K` | Focus BIN input |
| `Ctrl+D` | Toggle dark mode |
| `Ctrl+L` | Clear output |

## Project Structure

```
CC-GEN/
├── Cargo.toml              # Workspace root
├── crates/
│   ├── core/               # Core generation engine (Rust)
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── generator.rs
│   │       ├── validator.rs
│   │       ├── bin_database.rs
│   │       ├── pattern.rs
│   │       ├── card.rs
│   │       ├── crypto.rs
│   │       ├── formatter.rs
│   │       └── error.rs
│   └── wasm/               # WASM bindings
│       └── src/lib.rs
├── frontend/
│   ├── index.html
│   ├── css/                # Modular CSS (themes, animations, responsive)
│   ├── js/                 # Modular JS (app, wasm-bridge, ui, theme, shortcuts, export)
│   └── pkg/                # Built WASM artifacts
├── benches/                # Criterion benchmarks
└── tests/
```

## License

MIT
