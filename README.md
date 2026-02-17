# infrawatch

infrawatch is a lightweight Rust-based infrastructure monitoring and anomaly detection agent designed for cloud and hybrid environments. It collects host-level system metrics (memory, disk, processes) and cloud provider metrics (AWS CloudWatch), detects anomalies using configurable rules, and can alert via webhooks. The project is written with real-world engineering and platform requirements in mind and is suitable to demonstrate systems, observability, and systems programming skills on a resume.

Key highlights:
- Written in Rust for safety, performance, and minimal runtime overhead
- Integrates with AWS CloudWatch and EC2 for instance discovery and cloud metrics
- Collects rich host-level metrics locally using the `sysinfo` crate
- Pluggable analysis pipeline (threshold-based anomaly detection; Z-score planned)
- Designed to be extended with alerting (Discord webhook integration planned) and persistent storage (SQLite planned)

Table of Contents
- Features
- Architecture & Design
- Getting started
- Configuration
- Development
- Roadmap
- Contributing
- License

Features
- Host-level metrics: memory, disk, and top processes (via `sysinfo`).
- Cloud metrics: pulls CloudWatch metrics for EC2 instances.
- Unified analysis pipeline: normalized metrics flow through detectors (threshold-based now).
- CLI utility: `run`, `check`, `query`, `init` subcommands (see `src/main.rs`).
- Platform-friendly: small binary, no credentials stored (uses standard AWS credential chain).

Architecture & Design

Project layout (important files):

```
Cargo.toml
src/main.rs                    # CLI entrypoint
src/config/                    # config types and loaders
src/cloud/                     # cloud provider trait + AWS implementation
src/system/                    # system collector trait + sysinfo collector
src/analysis/ (planned)        # anomaly detectors and types (threshold module)
config/infrawatch.example.toml # example config used by `init`
```

Design notes:
- Separation of concerns: collectors (cloud/system) produce normalized `Metric` values for analysis; alerting and storage are separate layers.
- Config-driven: behavior is controlled by the TOML configuration (see `config/infrawatch.example.toml`).
- Safe AWS usage: authentication follows the standard credential chain (environment variables, `~/.aws/credentials`, or IAM role).
- Incremental implementation: threshold-based detection is implemented first; Z-score and persistent storage are planned to follow.

Getting started

Prerequisites
- Rust toolchain (stable or a recent nightly if you use the `edition = "2024"` entry in `Cargo.toml`).
- AWS credentials configured if you want CloudWatch integration (env vars, `~/.aws/credentials`, or instance role).

Build

```
cargo build --release
```

Run the CLI

- Generate a config template in the platform-appropriate config directory:

```
cargo run -- init
```

- Validate configuration and verify cloud connectivity:

```
cargo run -- check
```

- Start the daemon (placeholder; main loop planned):

```
cargo run -- run
```

Configuration

- The canonical example configuration is at `config/infrawatch.example.toml` and is copied to the host config directory when running `cargo run -- init`.
- Important sections:
  - `[aws]` — `region` and optional `profile_name` for AWS SDK behavior
  - `[metrics]` — which CloudWatch metrics to fetch
  - `[system]` — toggles for collecting memory, disk, and process metrics
  - `[analysis]` — threshold rules for anomaly detection (see roadmap if not present)

Development

Workflow suggestions:
- Run `cargo fmt` before committing formatting changes.
- Run `cargo clippy` to surface lints and potential improvements.
- Use `cargo test` to run unit and integration tests (none yet — tests are a planned improvement).

Key files to inspect when extending the project:
- `src/config/configs.rs` — config structs and serde usage
- `src/cloud/mod.rs` and `src/cloud/aws.rs` — `MetricsProvider` trait and AWS implementation
- `src/system/collector.rs` and `src/system/types.rs` — `SysinfoCollector` and system types
- `src/main.rs` — CLI entrypoint and available subcommands

Roadmap
- Phase 3: Analysis (implemented: threshold detector; planned: Z-score using historical storage)
- Phase 4: Alerting (Discord webhook integration; deduplication and cooldown policies)
- Phase 5: Daemon loop (polling, graceful shutdown, error handling)
- Phase 6: Storage (SQLite via `rusqlite` for historical baselines used by Z-score)
- Phase 7: Tests, CI, Docker image, and documentation polish

How this project demonstrates relevant skills
- Systems programming with Rust: memory-safe code, crate selection and idiomatic module layout
- Observability and monitoring: gathering and normalizing both host and cloud metrics
- Cloud integration: AWS SDK usage and secure credential handling patterns
- Design for extensibility: pluggable collectors, detectors, and alerting backends
- DevOps/platform engineering fit: building an agent/daemon, thinking about persistence, throttling, and alert fatigue

Contributing
- Contributions are welcome. Follow standard Git workflows: fork, branch, open PR with focused changes, run `cargo fmt` and `cargo clippy`, and include tests where appropriate.

License
- This project is unlicensed by default. Add an appropriate open-source license if you plan to publish or share the code publicly.

Contact
- For questions, reach the author via the project repository or attach contact info in your GitHub profile.

--

Minimal README written to be resume-friendly and highlight core engineering decisions. Extend with architecture diagrams, example alerts, screenshots, and telemetry outputs as you implement alerting and the daemon loop.
