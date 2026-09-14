# GPUI Calculator

一个使用 GPUI Kit 构建的桌面计算器，支持四则运算、百分比、正负号和小数。

## 本地开发

本机只编辑代码。编译与测试使用 4090 上的 Docker 环境；Dockerfile 已包含 Rust nightly 和 Linux 图形依赖。

## macOS 下载

GitHub Actions 工作流 `macos-dmg.yml` 支持手动触发和 `v*` 标签触发。运行完成后，在 Actions 的 Artifacts 下载 `gpui-calculator-macos`。
