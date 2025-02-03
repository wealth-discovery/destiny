from abc import ABC, abstractmethod
from datetime import datetime
from .destiny import *
import numpy as np


class Strategy(ABC):
    @abstractmethod
    def on_init(self, api: API):
        pass

    @abstractmethod
    def on_start(self, api: API):
        pass

    @abstractmethod
    def on_stop(self, api: API):
        pass

    @abstractmethod
    def on_daily(self, api: API):
        pass

    @abstractmethod
    def on_hourly(self, api: API):
        pass

    @abstractmethod
    def on_minutely(self, api: API):
        pass

    @abstractmethod
    def on_kline(self, api: API, kline: Kline):
        pass

    @abstractmethod
    def on_order(self, api: API, order: Order):
        pass

    @abstractmethod
    def on_position(self, api: API, position: Position):
        pass


class StrategyWrapper(Strategy):
    callback: Strategy
    _daily_cash: np.ndarray
    _time: datetime

    def __init__(self, callback: Strategy):
        self.callback = callback

    def on_init(self, api: API):
        self.callback.on_init(api)

    def on_start(self, api: API):
        self._daily_cash = np.array([api.cash_available()])
        self._time = api.time()
        self.callback.on_start(api)

    def on_stop(self, api: API):
        self.callback.on_stop(api)

    def on_daily(self, api: API):
        time = api.time()
        if time != self._time:
            cash_available = api.cash_available()
            self._daily_cash = np.append(self._daily_cash, cash_available)
            self._time = time
            daily_cash_mean = self._daily_cash.mean()
            info(f"{time.strftime('%Y%m%d')} 资金均值:({daily_cash_mean:.2f}),当前资产({cash_available:.2f})")
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
