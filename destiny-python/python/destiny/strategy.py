from abc import ABC, abstractmethod
from .destiny import *


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

    def __init__(self, callback: Strategy):
        self.callback = callback

    def on_init(self, api: API):
        self.callback.on_init(api)

    def on_start(self, api: API):
        self.callback.on_start(api)

    def on_stop(self, api: API):
        self.callback.on_stop(api)

    def on_daily(self, api: API):
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
