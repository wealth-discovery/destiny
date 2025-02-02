from destiny import *


class Config:
    symbol: str
    isbuy: bool

    def __init__(self):
        self.symbol = "ETHUSDT"
        self.isbuy = False


config = Config()


def on_init(api: API):
    api.init_symbol(config.symbol)


def on_daily(api: API):
    if not config.isbuy:
        api.long_limit_open("ETHUSDT", Decimal(0.1), Decimal(3000))
        config.isbuy = True
    time = api.time()
    price_mark = api.price_mark(config.symbol)
    cash_available = api.cash_available()
    margin = api.margin()
    long_size = api.long_size(config.symbol)
    info(f"{time} 标记价({price_mark:.2f}),可用资金({cash_available:.4f}),保证金({margin:.4f}),多仓({long_size:.4f})")


def on_kline(api: API, kline: Kline):
    info(api.time(), kline.close)


init_log(show_std=True, save_file=True)
run_backtest(
    begin="2023",
    end="2024",
    on_init=on_init,
    on_daily=on_daily,
    # on_kline=on_kline,
)
