from destiny import *

init_log(show_std=True, save_file=False)

download_history_data(
    [
        ("ETHUSDT", "202001", "202412"),
        ("BTCUSDT", "202001", "202412"),
        ("SOLUSDT", "202001", "202412"),
        ("DOGEUSDT", "202001", "202412"),
    ]
)
