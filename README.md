
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

## DOCS

https://github.com/sminez/my-penrose-config

https://sminez.github.io/penrose/rustdoc/penrose

https://sminez.github.io/penrose