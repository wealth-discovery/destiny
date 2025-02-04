from typing import List, Literal, Optional, Callable, Tuple
from datetime import datetime
from decimal import Decimal
from enum import Enum, auto

class TradeType(Enum):
    """
    交易类型
    """

    Limit = auto()
    """限价"""
    Market = auto()
    """市价"""

class TradeSide(Enum):
    """
    交易方向
    """

    Long = auto()
    """做多"""
    Short = auto()
    """做空"""

class OrderStatus(Enum):
    """
    订单状态
    """

    Created = auto()
    """已创建"""
    Submitted = auto()
    """已提交"""
    PartialFilled = auto()
    """部分成交"""
    Filled = auto()
    """已成交"""
    Canceled = auto()
    """已取消"""
    Rejected = auto()
    """已拒绝"""

def init_log(
    show_std: bool = False,
    save_file: bool = False,
    targets: List[str] = [],
    level: Literal["trace", "debug", "info", "warn", "error"] = "info",
) -> int:
    """
    初始化日志, 返回一个日志收集器
    [`show_std`] : 是否显示标准输出
    [`save_file`] : 是否保存到文件
    [`targets`] : 日志目标
    [`level`] : 日志级别
    """

def free_log(log_collector: int):
    """
    释放日志收集器
    [`log_collector`] : 日志收集器
    """

def trace(*args):
    """
    打印trace级别的日志
    """

def debug(*args):
    """
    打印`debug`级别的日志
    """

def info(*args):
    """
    打印`info`级别的日志
    """

def warn(*args):
    """
    打印`warn`级别的日志
    """

def error(*args):
    """
    打印`error`级别的日志
    """

def print(*args):
    """
    打印日志, 默认`debug`级别
    """

class Kline:
    """
    K线
    """

    symbol: str
    """交易对"""
    open_time: datetime
    """开盘时间"""
    open: Decimal
    """开盘价"""
    high: Decimal
    """最高价"""
    low: Decimal
    """最低价"""
    close: Decimal
    """收盘价"""
    size: Decimal
    """成交量"""
    cash: Decimal
    """成交额"""
    buy_size: Decimal
    """买方成交量"""
    buy_cash: Decimal
    """买方成交额"""
    trades: int
    """交易笔数"""

class Order:
    """
    订单
    """

    id: str
    """ID"""
    symbol: str
    """交易对"""
    type: TradeType
    """交易类型"""
    side: TradeSide
    """交易方向"""
    reduce_only: bool
    """是否只减仓"""
    status: OrderStatus
    """订单状态"""
    price: Decimal
    """价格"""
    size: Decimal
    """数量"""
    deal_price: Decimal
    """成交价格"""
    deal_size: Decimal
    """成交数量"""
    deal_fee: Decimal
    """成交手续费"""
    create_time: datetime
    """创建时间"""

class Position:
    """
    持仓
    """

    side: TradeSide
    """方向"""
    price: Decimal
    """持仓均价"""
    size: Decimal
    """持仓"""

class API:
    def time(self) -> datetime: ...
    def stop(self): ...
    def init_symbol(self, symbol: str): ...
    def order(self, symbol: str, id: str) -> Optional[Order]: ...
    def orders(self, symbol: str) -> List[Order]: ...
    def orders_long(self, symbol: str) -> List[Order]: ...
    def orders_long_open(self, symbol: str) -> List[Order]: ...
    def orders_long_close(self, symbol: str) -> List[Order]: ...
    def orders_short(self, symbol: str) -> List[Order]: ...
    def orders_short_open(self, symbol: str) -> List[Order]: ...
    def orders_short_close(self, symbol: str) -> List[Order]: ...
    def leverage(self, symbol: str) -> int: ...
    def cash(self) -> Decimal: ...
    def cash_available(self) -> Decimal: ...
    def cash_frozen(self) -> Decimal: ...
    def margin(self) -> Decimal: ...
    def pnl(self) -> Decimal: ...
    def long_price(self, symbol: str) -> Decimal: ...
    def long_size(self, symbol: str) -> Decimal: ...
    def long_size_available(self, symbol: str) -> Decimal: ...
    def long_size_frozen(self, symbol: str) -> Decimal: ...
    def long_margin(self, symbol: str) -> Decimal: ...
    def long_pnl(self, symbol: str) -> Decimal: ...
    def short_price(self, symbol: str) -> Decimal: ...
    def short_size(self, symbol: str) -> Decimal: ...
    def short_size_available(self, symbol: str) -> Decimal: ...
    def short_size_frozen(self, symbol: str) -> Decimal: ...
    def short_margin(self, symbol: str) -> Decimal: ...
    def short_pnl(self, symbol: str) -> Decimal: ...
    def symbol_pnl(self, symbol: str) -> Decimal: ...
    def symbol_margin(self, symbol: str) -> Decimal: ...
    def price_mark(self, symbol: str) -> Decimal: ...
    def price_last(self, symbol: str) -> Decimal: ...
    def price_index(self, symbol: str) -> Decimal: ...
    def price_settlement(self, symbol: str) -> Decimal: ...
    def time_settlement(self, symbol: str) -> datetime: ...
    def rule_price_min(self, symbol: str) -> Decimal: ...
    def rule_price_max(self, symbol: str) -> Decimal: ...
    def rule_price_tick(self, symbol: str) -> Decimal: ...
    def rule_size_min(self, symbol: str) -> Decimal: ...
    def rule_size_max(self, symbol: str) -> Decimal: ...
    def rule_size_tick(self, symbol: str) -> Decimal: ...
    def rule_amount_min(self, symbol: str) -> Decimal: ...
    def rule_order_max(self, symbol: str) -> int: ...
    def long_market_open(self, symbol: str, size: Decimal) -> str: ...
    def long_limit_open(self, symbol: str, size: Decimal, price: Decimal) -> str: ...
    def long_market_close(self, symbol: str, size: Decimal) -> str: ...
    def long_limit_close(self, symbol: str, size: Decimal, price: Decimal) -> str: ...
    def short_market_open(self, symbol: str, size: Decimal) -> str: ...
    def short_limit_open(self, symbol: str, size: Decimal, price: Decimal) -> str: ...
    def short_market_close(self, symbol: str, size: Decimal) -> str: ...
    def short_limit_close(self, symbol: str, size: Decimal, price: Decimal) -> str: ...
    def order_close(self, symbol: str, id: str): ...
    def order_cancel_many(self, symbol: str, ids: List[str]): ...
    def leverage_set(self, symbol: str, leverage: int): ...

def download_history_data(metas: List[Tuple[str, str, str]]):
    """
    下载历史数据
    [`metas`] : 历史数据元组列表. Tuple(交易对, 开始时间, 结束时间)
    """

BasicCallback = Callable[[API], None]
KlineCallback = Callable[[API, Kline], None]
OrderCallback = Callable[[API, Order], None]
PositionCallback = Callable[[API, Position], None]

def run_backtest__(
    begin: str,
    end: str,
    cash: Decimal = Decimal("1000"),
    fee_rate_taker: Decimal = Decimal("0.0005"),
    fee_rate_maker: Decimal = Decimal("0.0005"),
    slippage_rate: Decimal = Decimal("0.01"),
    on_init: Optional[BasicCallback] = None,
    on_start: Optional[BasicCallback] = None,
    on_stop: Optional[BasicCallback] = None,
    on_daily: Optional[BasicCallback] = None,
    on_hourly: Optional[BasicCallback] = None,
    on_minutely: Optional[BasicCallback] = None,
    on_kline: Optional[KlineCallback] = None,
    on_order: Optional[OrderCallback] = None,
    on_position: Optional[PositionCallback] = None,
):
    """
    运行回测
    [`begin`] : 开始时间
    [`end`] : 结束时间
    [`cash`] : 初始资金
    [`fee_rate_taker`] : 吃单手续费
    [`fee_rate_maker`] : 挂单手续费
    [`slippage_rate`] : 滑点
    [`on_init`] : 初始化事件
    [`on_start`] : 开始事件
    [`on_stop`] : 停止事件
    [`on_daily`] : 每日事件
    [`on_hourly`] : 每小时事件
    [`on_minutely`] : 每分钟事件
    [`on_kline`] : K线事件
    [`on_order`] : 订单事件
    [`on_position`] : 持仓事件
    """
