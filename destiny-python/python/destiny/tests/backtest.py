from destiny import *


class BacktestStrategy(Strategy):
    symbol: str
    isbuy: bool

    def __init__(self):
        self.symbol = "ETHUSDT"
        self.isbuy = False

    def on_init(self, api: API):
        api.init_symbol(self.symbol)

    def on_start(self, api: API):
        pass

    def on_stop(self, api: API):
        pass

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

    def on_hourly(self, api: API):
        pass

    def on_minutely(self, api: API):
        pass

    def on_kline(self, api: API, kline: Kline):
        pass

    def on_order(self, api: API, order: Order):
        pass

    def on_position(self, api: API, position: Position):
        pass


init_log(show_std=True, save_file=True)
run_backtest(BacktestStrategy(), "2023", "2024")
