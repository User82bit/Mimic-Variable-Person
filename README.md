# Mimic-Variable-Person (M.V.P.)

[![License](https://img.shields.io/badge/license-Apache--2.0-blue.svg)](LICENSE.txt)
[![Version](https://img.shields.io/github/v/release/User82bit/Mimic-Variable-Person?display_name=tag)](../../releases/latest)
[![Language](https://img.shields.io/badge/language-Rust-000000?logo=rust&logoColor=white)](https://www.rust-lang.org/)

O **Mimic-Variable-Person (MVP)** é um projeto experimental de agente modular, planejado para organizar capacidades cognitivas e comportamentais em módulos independentes. O projeto está em fase inicial de migração e estruturação em **Rust**.

> **Estado atual:** a estrutura de inicialização e os módulos-base estão configurados. Os módulos cognitivos ainda são esqueletos; os arquivos JSON são modelos de dados e ainda não são carregados pelo programa.

---

## Estrutura atual

```text
Mimic-Variable-Person/
├── data/
│   └── brain/
│       ├── feelings-data.json
│       └── thought-data.json
├── src/
│   ├── brain/
│   │   ├── feelings.rs
│   │   ├── memory.rs
│   │   ├── mod.rs
│   │   └── thought.rs
│   └── main.rs
├── .github/
│   └── workflows/
│       ├── ci.yml
│       ├── release-production.yml
│       └── stale.yml
├── Cargo.toml
└── Cargo.lock
```

### Organização

- **Brain:** ponto de entrada para os módulos de sentimentos, memória e pensamento.
- **Data:** modelos JSON separados do código-fonte. A leitura, validação e persistência desses dados ainda serão implementadas.
- **GitHub Actions:** valida formatação, executa Clippy, testes e compilação em Linux e Windows.

A antiga estrutura C#/.NET foi removida durante a migração. Módulos de atuação física/lógica ainda não foram recriados em Rust.

---

## Requisitos

- Rust **1.85.0 ou superior** (edição 2024).
- Cargo, instalado junto com o toolchain Rust.

Instale o toolchain pelo site oficial: https://www.rust-lang.org/tools/install

## Executar localmente

Na raiz do repositório:

```bash
cargo run
```

## Verificações de desenvolvimento

```bash
cargo fmt --all -- --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all-targets
cargo build --release
```

---

## Licença

Este projeto está licenciado sob a **[Apache License 2.0](LICENSE.txt)**.
