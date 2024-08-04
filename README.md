# Mina Plugin

[![fluentci pipeline](https://shield.fluentci.io/x/mina)](https://pkg.fluentci.io/mina)
[![ci](https://github.com/fluentci-io/mina-plugin/actions/workflows/ci.yml/badge.svg)](https://github.com/fluentci-io/mina-plugin/actions/workflows/ci.yml)

This plugin install and run [mina](https://github.com/mina-deploy/mina) on your CI/CD pipelines.

## 🚀 Usage

Add the following command to your CI configuration file:

```bash
fluentci run --wasm mina install
```

## Functions

| Name      | Description                               |
| --------- | ----------------------------------------- |
| install   | Install mina                              |
| setup     | Setup mina                                |
| deploy    | Deploy your project using mina            |

## Code Usage

Add `fluentci-pdk` crate to your `Cargo.toml`:

```toml
[dependencies]
fluentci-pdk = "0.2.1"
```

Use the following code to call the plugin:

```rust
use fluentci_pdk::dag;

// ...

dag().call("https://pkg.fluentci.io/mina@v0.1.1?wasm=1", "install", vec![])?;
```

## 📚 Examples

Github Actions:

```yaml
- name: Setup Fluent CI CLI
  uses: fluentci-io/setup-fluentci@v5
  with:
    wasm: true
    plugin: mina
    args: |
      install
- name: Show mina version
  run: |
    flox activate -- type mina
    flox activate -- mina --version
```
