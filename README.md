# wraith-dom

[![no_std](https://img.shields.io/badge/no__std-compatible-brightgreen)](https://doc.rust-lang.org/reference/names/preludes.html#the-no_std-attribute)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue)](LICENSE-MIT)

A minimal `#![no_std]` HTML parser with CSS selectors and Cloudflare challenge solver.

Designed for environments without the standard library: bare-metal systems, WebAssembly, embedded devices, and anywhere you need lightweight HTML processing without pulling in a full browser engine.

## Features

- **HTML parsing** -- tokenizer and tree builder producing a flat node arena with parent/child indices
- **CSS selectors** -- tag, id, class, attribute presence/value, descendant combinators, comma-separated alternatives
- **Form detection** -- extract all forms and inputs; heuristic login/OAuth form finder
- **Text extraction** -- visible text (skipping script/style), page title, and link extraction
- **Cloudflare IUAM bypass** (optional, behind `cloudflare` feature) -- detect and solve Cloudflare "Under Attack Mode" challenge pages via [js-lite](https://github.com/suhteevah/js-lite)

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
wraith-dom = "0.1"
```

To enable Cloudflare challenge solving:

```toml
[dependencies]
wraith-dom = { version = "0.1", features = ["cloudflare"] }
```

### Parse HTML and query elements

```rust
use wraith_dom::{parse, Selector, select};

let doc = parse("<html><body><p class=\"intro\">Hello</p><p>World</p></body></html>");

// Select by tag
let sel = Selector::parse("p").unwrap();
let matches = select(&doc, &sel);
assert_eq!(matches.len(), 2);

// Select by class
let sel = Selector::parse(".intro").unwrap();
let matches = select(&doc, &sel);
assert_eq!(matches.len(), 1);

// Get text content
let text = doc.inner_text(matches[0]);
assert_eq!(text, "Hello");
```

### Extract forms

```rust
use wraith_dom::{parse, find_forms, find_login_form};

let doc = parse(r#"
    <form action="/login" method="post">
        <input type="email" name="user">
        <input type="password" name="pass">
        <button type="submit">Sign In</button>
    </form>
"#);

let forms = find_forms(&doc);
assert_eq!(forms.len(), 1);
assert_eq!(forms[0].action, "/login");
assert_eq!(forms[0].method, "POST");

// Heuristic login form detection
let login = find_login_form(&doc).unwrap();
assert!(login.inputs.iter().any(|i| i.input_type == "password"));
```

### Extract text and links

```rust
use wraith_dom::{parse, extract_text, extract_title, extract_links};

let doc = parse(r#"
    <html>
    <head><title>My Page</title></head>
    <body>
        <h1>Welcome</h1>
        <p>Visit <a href="https://example.com">Example</a></p>
        <script>var x = 1;</script>
    </body>
    </html>
"#);

let title = extract_title(&doc);
assert_eq!(title, Some("My Page".into()));

let text = extract_text(&doc);
assert!(text.contains("Welcome"));
assert!(!text.contains("var x")); // scripts are excluded

let links = extract_links(&doc);
assert_eq!(links[0].0, "https://example.com");
assert_eq!(links[0].1, "Example");
```

### CSS selector syntax

| Pattern | Matches |
|---------|---------|
| `p` | All `<p>` elements |
| `#main` | Element with `id="main"` |
| `.active` | Elements with class `active` |
| `[type]` | Elements with a `type` attribute |
| `[type="email"]` | Elements where `type="email"` |
| `form input` | `<input>` elements inside a `<form>` (descendant) |
| `input, select` | `<input>` or `<select>` elements |
| `button.submit` | `<button>` elements with class `submit` |
| `input[type="email"]` | `<input>` elements where `type="email"` |

## no_std

This crate is `#![no_std]` and requires only `alloc`. It has no dependencies beyond `log` (for optional debug logging). The `cloudflare` feature adds a dependency on `js-lite` for JavaScript evaluation.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or <http://www.apache.org/licenses/LICENSE-2.0>)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or <http://opensource.org/licenses/MIT>)

at your option.

## Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted for inclusion in the work by you, as defined in the Apache-2.0 license, shall be dual licensed as above, without any additional terms or conditions.

---

---

---

---

---

---

---

---

---

---

---

---

## Support This Project

If you find this project useful, consider buying me a coffee! Your support helps me keep building and sharing open-source tools.

[![Donate via PayPal](https://img.shields.io/badge/Donate-PayPal-blue.svg?logo=paypal)](https://www.paypal.me/baal_hosting)

**PayPal:** [baal_hosting@live.com](https://paypal.me/baal_hosting)

Every donation, no matter how small, is greatly appreciated and motivates continued development. Thank you!
