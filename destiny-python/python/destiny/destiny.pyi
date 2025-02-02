from typing import List, Literal

def init_log(
    show_std: bool = False,
    save_file: bool = False,
    targets: List[str] = [],
    level: Literal["trace", "debug", "info", "warn", "error"] = "info",
) -> any:
    """
    初始化日志, 返回一个日志收集器
    [`show_std`] : 是否显示标准输出
    [`save_file`] : 是否保存到文件
    [`targets`] : 日志目标
    [`level`] : 日志级别
    """

def free_log(log_collector: any):
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
    打印debug级别的日志
    """

def info(*args):
    """
    打印info级别的日志
    """

def warn(*args):
    """
    打印warn级别的日志
    """

def error(*args):
    """
    打印error级别的日志
    """
