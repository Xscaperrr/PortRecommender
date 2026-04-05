# Port Recommender

`port-recommender` 是一个使用 Rust 编写的跨平台 CLI 工具，可以把一个服务名称映射成一个稳定且可用的端口号。

它适合本地开发、脚本调用和轻量级服务约定场景，帮助你避免手动挑选端口。

## 功能特性

- 使用 `SHA-256` 对输入名称做哈希映射。
- 仅在安全的用户端口范围 `1024-65535` 内推荐端口。
- 内置排除常见端口，例如 `22`、`80`、`443`、`3306`、`5432`、`6379`、`8080`、`8443`、`27017` 等。
- 自动排除当前机器已经被占用的端口。
- 支持 `tcp`、`udp` 和 `both` 三种占用检查模式。
- 支持 Windows、Linux 和 macOS。
- 在排除集合不变的前提下，相同名称会得到稳定结果。

## 工作原理

1. 对输入名称做 `SHA-256` 哈希。
2. 取哈希结果前 8 个字节，映射到 `1024-65535` 范围内的一个起始端口。
3. 工具会排除：
   - 内置的常见端口；
   - 当前机器上、在所选协议范围内已经被占用的端口。
4. 如果起始端口不可用，就按顺序向后探测；到达上限后会回绕到范围起点继续查找。
5. 找到第一个可用端口后，只向标准输出打印该端口号。

由于本机当前监听端口会参与排除，因此同一个名称在不同机器上，或者同一台机器的不同时间点，得到的推荐结果可能不同。

## 使用方法

### 通过 Cargo 运行

```bash
cargo run -- <名称>
```

示例：

```bash
cargo run -- example-service
```

### 使用编译后的二进制

```bash
port-recommender <名称>
```

### 协议参数

默认会同时检查 TCP 和 UDP。

```bash
port-recommender <名称> --protocol both
port-recommender <名称> --protocol tcp
port-recommender <名称> --protocol udp
```

示例：

```bash
port-recommender api-gateway
port-recommender redis-shadow --protocol tcp
port-recommender telemetry-agent --protocol udp
```

### 输出格式

成功时，CLI 只输出端口号：

```text
15251
```

失败时，会向 stderr 输出错误信息，并返回非零退出码。

## 打包与构建

### 开发构建

```bash
cargo build
```

### Release 构建

```bash
cargo build --release
```

生成的可执行文件路径：

- Windows: `target/release/port-recommender.exe`
- Linux: `target/release/port-recommender`
- macOS: `target/release/port-recommender`

### 本地安装

如果希望在终端里直接使用命令，可以执行：

```bash
cargo install --path .
```

### 打包源码

如果需要生成 Cargo 源码包，可执行：

```bash
cargo package
```

### 针对指定平台构建

示例：

```bash
cargo build --release --target x86_64-pc-windows-msvc
cargo build --release --target x86_64-unknown-linux-gnu
cargo build --release --target aarch64-apple-darwin
```

对应产物会输出到：

```text
target/<target-triple>/release/
```

## 测试

运行全部测试：

```bash
cargo test
```

仓库中的 CI 已配置在 Windows、Linux 和 macOS 上执行测试。

## 项目结构

- `src/main.rs`：CLI 入口
- `src/cli.rs`：命令行参数解析
- `src/recommender.rs`：哈希映射与端口探测逻辑
- `src/ports.rs`：本机已占用端口探测
- `src/common_ports.rs`：内置常见端口排除列表
- `tests/cli.rs`：CLI 集成测试

## 许可证

本项目采用 MIT 许可证。

详见 [LICENSE](LICENSE)。
