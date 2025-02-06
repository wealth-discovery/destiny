from .destiny import *
from .strategy import Strategy
from .backtest_strategy import BacktestStrategy
from decimal import Decimal
import setproctitle


def run_backtest(
    strategy: Strategy,
    begin: str,
    end: str,
    cash: Decimal = Decimal("1000"),
    fee_rate_taker: Decimal = Decimal("0.0005"),
    fee_rate_maker: Decimal = Decimal("0.0005"),
    slippage_rate: Decimal = Decimal("0.01"),
) -> Strategy:
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

    setproctitle.setproctitle(f"wealth-discovery-destiny-backtest")
    backtest_strategy = BacktestStrategy(strategy)
    run_backtest__(
        begin,
        end,
        cash,
        fee_rate_taker,
        fee_rate_maker,
        slippage_rate,
        backtest_strategy.on_init,
        backtest_strategy.on_start,
        backtest_strategy.on_stop,
        backtest_strategy.on_daily,
        backtest_strategy.on_hourly,
        backtest_strategy.on_minutely,
        backtest_strategy.on_kline,
        backtest_strategy.on_order,
        backtest_strategy.on_position,
    )
    return backtest_strategy
