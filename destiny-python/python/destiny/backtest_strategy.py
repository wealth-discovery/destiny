from .destiny import *
from .strategy import Strategy
from typing import List
from decimal import Decimal
import numpy as np


class BacktestStrategy(Strategy):
    callback: Strategy
    symbols: List[str]
    start_cash: Decimal
    daily_cash: np.ndarray
    daily_pnl: np.ndarray
    daily_seq: int

    def __init__(self, callback: Strategy):
        self.callback = callback

    def on_init(self, api: API):
        self.callback.on_init(api)

    def on_start(self, api: API):
        self.symbols = api.symbols()
        self.start_cash = api.cash()
        self.daily_cash = np.zeros(3650)
        self.daily_pnl = np.zeros(3650)
        self.daily_seq = 0
        self.callback.on_start(api)

    def on_stop(self, api: API):
        self.callback.on_stop(api)

    def on_daily(self, api: API):
        self.daily_cash[self.daily_seq] = api.cash()
        self.daily_pnl[self.daily_seq] = api.pnl()
        self.daily_seq += 1
        self.callback.on_daily(api)

    def on_hourly(self, api: API):
        self.callback.on_hourly(api)

    def on_minutely(self, api: API):
        self.callback.on_minutely(api)

    def on_kline(self, api: API, kline: Kline):
        self.callback.on_kline(api, kline)

    def on_order(self, api: API, order: Order):
        self.callback.on_order(api, order)

    def on_position(self, api: API, position: Position):
        self.callback.on_position(api, position)
