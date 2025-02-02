from destiny import *


log_collector = init_log(show_std=True, level="trace")
debug("debug", "123")
free_log(log_collector)
debug("debug")
