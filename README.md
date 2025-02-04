# Destiny 天命量化库

[![Rust](https://github.com/wealth-discovery/destiny/actions/workflows/rust-test-check.yml/badge.svg)](https://github.com/wealth-discovery/destiny/actions/workflows/rust-test-check.yml)
[![Clippy](https://github.com/wealth-discovery/destiny/actions/workflows/rust-clippy-check.yml/badge.svg)](https://github.com/wealth-discovery/destiny/actions/workflows/rust-clippy-check.yml)

Destiny 是一个用 Rust 编写的高性能量化交易库，旨在提供快速、可靠的量化交易功能。

## ✨ 特性

- 🚀 高性能回测引擎
- 📊 数据处理与分析
- 🔧 实用的交易工具集
- 💡 灵活的策略开发框架

## 🚀 快速开始

### 环境要求

- Python 3.8+
- Rust 1.75+
- Cargo

### 安装

#### Python 包安装

```bash
pip install wealth-discovery-destiny
```

#### 从源码安装

```bash
git clone https://github.com/wealth-discovery/destiny.git
cd destiny
cargo build --release
```

### 运行测试

```bash
cargo test --all
```

## 📖 使用示例

### 下载历史数据

```python
from destiny import *

init_log(show_std=True, save_file=False)

download_history_data([
    ("BTCUSDT", "202001", "202501"),
    ("ETHUSDT", "202001", "202501"),
    ("SOLUSDT", "202009", "202501"),
    ("DOGEUSDT", "202007", "202501"),
])
```

### 回测策略示例

```python
from destiny import *

class BacktestStrategy(Strategy):
    symbol: str
    isbuy: bool

    def __init__(self):
        self.symbol = "ETHUSDT"
        self.isbuy = False

    def on_init(self, api: API):
        api.init_symbol(self.symbol)

    def on_daily(self, api: API):
        if not self.isbuy:
            api.long_limit_open(self.symbol, Decimal(0.1), Decimal(3000))
            self.isbuy = True
        
        time = api.time()
        price_mark = api.price_mark(self.symbol)
        cash_available = api.cash_available()
        margin = api.margin()
        long_size = api.long_size(self.symbol)
        info(
            f"{time} 标记价({price_mark:.2f}),可用资金({cash_available:.4f}),保证金({margin:.4f}),多仓({long_size:.4f})"
        )

# 初始化日志并运行回测
init_log(show_std=True, save_file=True)
run_backtest(BacktestStrategy(), "2023", "2024")
```

## 🤝 贡献指南

欢迎提交 Pull Request 和 Issue！

## 📄 开源协议

本项目采用 [LICENSE](LICENSE) 协议开源。

## 👥 作者

- Husky (<husky.robot.dog@gmail.com>)
