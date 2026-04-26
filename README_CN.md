# Portico

一个基于 Rust 的 SSH 端口转发管理工具，支持实时监控和终端用户界面。

[English Version](./README.md)

## 功能特性

- 通过 YAML 配置文件管理多个 SSH 服务器和端口转发
- 支持密码和私钥认证
- 实时监控端口转发状态
- 终端用户界面 (TUI) 用于可视化连接和流量
- 退出时自动清理

## 安装

```bash
git clone https://github.com/yourusername/portico.git
cd portico
cargo build --release
```

## 使用方法

1. 创建配置文件（参考 [examples/config.yaml](./examples/config.yaml)）
2. 运行 Portico：

```bash
./target/release/portico -f your-config.yaml
```

## 配置

Portico 使用 YAML 配置文件。以下是一个基本示例：

```yaml
timeout:
  connect_timeout_secs: 5
  read_timeout_secs: 10
  write_timeout_secs: 10

servers:
  - name: "server1"
    host: "192.168.1.100"
    port: 22
    username: "user"
    password: "password"
    forwardings:
      - local_port: 8080
        remote_host: "localhost"
        remote_port: 80
```

更多示例请查看 [examples](./examples) 目录。

## 许可证

本项目采用 MIT 许可证。
