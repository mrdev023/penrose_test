
# Configure

```bash
sudo pacman -S xorg-server-xephyr
```

```bash
Xephyr -br -ac -noreset -screen 800x600 :2&
```

```bash
DISPLAY=:2 target/release/my_penrose_config
```