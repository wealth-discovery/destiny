from destiny import *

init_log(show_std=True, save_file=False)

download_history_data(
    [
        ("BTCUSDT", "202001", "202501"),
        ("ETHUSDT", "202001", "202501"),
        ("SOLUSDT", "202009", "202501"),
        ("DOGEUSDT", "202007", "202501"),
        ("TRUMPUSDT", "202501", "202501"),
    ]
)
