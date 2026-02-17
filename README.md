# infrawatch

infrawatch is a lightweight Rust-based infrastructure monitoring and anomaly detection agent designed for cloud and hybrid environments. It collects host-level system metrics (memory, disk, processes) and cloud provider metrics (AWS CloudWatch), detects anomalies using configurable rules, and can alert via webhooks.

Key highlights:
- Written in Rust for safety, performance, and minimal runtime overhead
- Integrates with AWS CloudWatch and EC2 for instance discovery and cloud metrics
- Collects rich host-level metrics locally using the `sysinfo` crate
- Pluggable analysis pipeline
- Designed to be extended with alerting

Features
- Host-level metrics: memory, disk, and processes information.
- Cloud metrics: pulls CloudWatch metrics for EC2 instances.
- Unified analysis pipeline: normalized metrics flow through detectors.
- CLI utility: `run`, `check`, `query`, `init` subcommands.
- Platform-friendly: small binary, no credentials stored (uses standard AWS credential chain).

Getting started

Prerequisites
- Rust toolchain (stable or a recent nightly if you use the `edition = "2024"` entry in `Cargo.toml`).
- AWS credentials configured if you want CloudWatch integration.

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

Minimal README written to be resume-friendly and highlight core engineering decisions. Extend with architecture diagrams, example alerts, screenshots, and telemetry outputs as you implement alerting and the daemon loop.
