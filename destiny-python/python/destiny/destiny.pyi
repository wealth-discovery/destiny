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
    def time(self) -> datetime:
        """
        获取当前时间
        """

    def stop(self):
        """
        停止运行
        """

    def init_symbol(self, symbol: str):
        """
        初始化交易对
        [`symbol`] : 交易对
        """

    def order(self, symbol: str, id: str) -> Optional[Order]:
        """
        获取订单
        [`symbol`] : 交易对
        [`id`] : 订单ID
        """

    def orders(self, symbol: str) -> List[Order]:
        """
        获取订单
        [`symbol`] : 交易对
        """

    def orders_long(self, symbol: str) -> List[Order]:
        """
        获取做多订单
        [`symbol`] : 交易对
        """

    def orders_long_open(self, symbol: str) -> List[Order]:
        """
        获取做多开仓订单
        [`symbol`] : 交易对
        """

    def orders_long_close(self, symbol: str) -> List[Order]:
        """
        获取做多平仓订单
        [`symbol`] : 交易对
        """

    def orders_short(self, symbol: str) -> List[Order]:
        """
        获取做空订单
        [`symbol`] : 交易对
        """

    def orders_short_open(self, symbol: str) -> List[Order]:
        """
        获取做空开仓订单
        [`symbol`] : 交易对
        """

    def orders_short_close(self, symbol: str) -> List[Order]:
        """
        获取做空平仓订单
        [`symbol`] : 交易对
        """

    def leverage(self, symbol: str) -> int:
        """
        获取杠杆倍率
        [`symbol`] : 交易对
        """

    def cash(self) -> Decimal:
        """
        获取资金
        """

    def cash_available(self) -> Decimal:
        """
        获取可用资金
        """

    def cash_frozen(self) -> Decimal:
        """
        获取冻结资金
        """

    def margin(self) -> Decimal:
        """
        获取保证金
        """

    def pnl(self) -> Decimal:
        """
        获取持仓盈亏
        """

    def long_price(self, symbol: str) -> Decimal:
        """
        获取做多均价
        [`symbol`] : 交易对
        """

    def long_size(self, symbol: str) -> Decimal:
        """
        获取做多持仓
        [`symbol`] : 交易对
        """

    def long_size_available(self, symbol: str) -> Decimal:
        """
        获取做多可用持仓
        [`symbol`] : 交易对
        """

    def long_size_frozen(self, symbol: str) -> Decimal:
        """
        获取做多冻结持仓
        [`symbol`] : 交易对
        """

    def long_margin(self, symbol: str) -> Decimal:
        """
        获取做多保证金
        [`symbol`] : 交易对
        """

    def long_pnl(self, symbol: str) -> Decimal:
        """
        获取做多盈亏
        [`symbol`] : 交易对
        """

    def short_price(self, symbol: str) -> Decimal:
        """
        获取做空均价
        [`symbol`] : 交易对
        """

    def short_size(self, symbol: str) -> Decimal:
        """
        获取做空持仓
        [`symbol`] : 交易对
        """

    def short_size_available(self, symbol: str) -> Decimal:
        """
        获取做空可用持仓
        [`symbol`] : 交易对
        """

    def short_size_frozen(self, symbol: str) -> Decimal:
        """
        获取做空冻结持仓
        [`symbol`] : 交易对
        """

    def short_margin(self, symbol: str) -> Decimal:
        """
        获取做空保证金
        [`symbol`] : 交易对
        """

    def short_pnl(self, symbol: str) -> Decimal:
        """
        获取做空盈亏
        [`symbol`] : 交易对
        """

    def symbols(self) -> List[str]:
        """
        获取交易对列表
        """

    def symbol_pnl(self, symbol: str) -> Decimal:
        """
        获取交易对盈亏
        [`symbol`] : 交易对
        """

    def symbol_margin(self, symbol: str) -> Decimal:
        """
        获取交易对保证金
        [`symbol`] : 交易对
        """

    def price_mark(self, symbol: str) -> Decimal:
        """
        获取标记价格
        [`symbol`] : 交易对
        """

    def price_last(self, symbol: str) -> Decimal:
        """
        获取最新价格
        [`symbol`] : 交易对
        """

    def price_index(self, symbol: str) -> Decimal:
        """
        获取指数价格
        [`symbol`] : 交易对
        """

    def price_settlement(self, symbol: str) -> Decimal:
        """
        获取结算价格
        [`symbol`] : 交易对
        """

    def time_settlement(self, symbol: str) -> datetime:
        """
        获取下一次结算时间
        [`symbol`] : 交易对
        """

    def rule_price_min(self, symbol: str) -> Decimal:
        """
        获取最小交易价格
        [`symbol`] : 交易对
        """

    def rule_price_max(self, symbol: str) -> Decimal:
        """
        获取最大交易价格
        [`symbol`] : 交易对
        """

    def rule_price_tick(self, symbol: str) -> Decimal:
        """
        获取交易价格步长
        [`symbol`] : 交易对
        """

    def rule_size_min(self, symbol: str) -> Decimal:
        """
        获取最小交易数量
        [`symbol`] : 交易对
        """

    def rule_size_max(self, symbol: str) -> Decimal:
        """
        获取最大交易数量
        [`symbol`] : 交易对
        """

    def rule_size_tick(self, symbol: str) -> Decimal:
        """
        获取交易数量步长
        [`symbol`] : 交易对
        """

    def rule_amount_min(self, symbol: str) -> Decimal:
        """
        获取最小交易金额
        [`symbol`] : 交易对
        """

    def rule_order_max(self, symbol: str) -> int:
        """
        获取最大订单数量
        [`symbol`] : 交易对
        """

    def long_market_open(self, symbol: str, size: Decimal) -> str:
        """
        做多市价开仓
        [`symbol`] : 交易对
        [`size`] : 数量
        """

    def long_limit_open(self, symbol: str, size: Decimal, price: Decimal) -> str:
        """
        做多限价开仓
        [`symbol`] : 交易对
        [`size`] : 数量
        [`price`] : 价格
        """

    def long_market_close(self, symbol: str, size: Decimal) -> str:
        """
        做多市价平仓
        [`symbol`] : 交易对
        [`size`] : 数量
        """

    def long_limit_close(self, symbol: str, size: Decimal, price: Decimal) -> str:
        """
        做多限价平仓
        [`symbol`] : 交易对
        [`size`] : 数量
        [`price`] : 价格
        """

    def short_market_open(self, symbol: str, size: Decimal) -> str:
        """
        做空市价开仓
        [`symbol`] : 交易对
        [`size`] : 数量
        """

    def short_limit_open(self, symbol: str, size: Decimal, price: Decimal) -> str:
        """
        做空限价开仓
        [`symbol`] : 交易对
        [`size`] : 数量
        [`price`] : 价格
        """

    def short_market_close(self, symbol: str, size: Decimal) -> str:
        """
        做空市价平仓
        [`symbol`] : 交易对
        [`size`] : 数量
        """

    def short_limit_close(self, symbol: str, size: Decimal, price: Decimal) -> str:
        """
        做空限价平仓
        [`symbol`] : 交易对
        [`size`] : 数量
        [`price`] : 价格
        """

    def order_close(self, symbol: str, id: str):
        """
        平仓订单
        [`symbol`] : 交易对
        [`id`] : 订单ID
        """

    def order_cancel_many(self, symbol: str, ids: List[str]):
        """
        取消多个订单
        [`symbol`] : 交易对
        [`ids`] : 订单ID列表
        """

    def leverage_set(self, symbol: str, leverage: int):
        """
        设置杠杆倍率
        [`symbol`] : 交易对
        [`leverage`] : 杠杆倍率
        """

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
