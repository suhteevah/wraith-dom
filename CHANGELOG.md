# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-04-02

### Added

- HTML parser with flat node arena, handling start/end/self-closing tags, comments, entities, and raw-text elements (script/style)
- CSS selector engine supporting tag, id, class, attribute presence/value, descendant combinators, and comma-separated alternatives
- Form detection and extraction with login/OAuth form heuristics
- Text extraction (visible text, title, links) with script/style filtering
- Cloudflare IUAM challenge detection and solving (behind `cloudflare` feature flag)
- Full `#![no_std]` support with `alloc` only
- 32 unit tests across all modules
