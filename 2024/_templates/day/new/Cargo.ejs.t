---
to: packages/day-<%= dayNumber %>/Cargo.toml
---
[package]
name = "day-<%= dayNumber %>"
edition.workspace = true
version.workspace = true
authors.workspace = true

[dependencies]
utils.workspace = true
