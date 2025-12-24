# 🦀 Xandeum pNode Monitor (CLI)

A high-performance, resource-efficient terminal dashboard for monitoring Xandeum pNodes. Built in Rust for zero-overhead observability.

## 📸 Demo
> *[Insert your screenshot or video link here]*

## 🚀 Features
- **Real-time Latency Tracking:** Visualizes pNode network jitter.
- **Resource Efficient:** Consumes <5MB RAM (unlike Electron/React apps).
- **Headless Ready:** Designed for node operators running on bare metal servers via SSH.
- **Simulation Mode:** Includes a built-in mock driver to test UI responsiveness.

## 🛠️ Usage
1. Clone the repo
2. Run with Cargo:cargo run --release


## 🔮 Roadmap
- [ ] Integration with live pnRPC (Post-Mainnet)
- [ ] Alerting (Discord/Slack webhooks on downtime)
- [ ] Multi-node config via TOML
