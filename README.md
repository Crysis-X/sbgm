# Simplish Background Manager (SBGM)

**SBGM** is a simple wallpaper manager for Linux that uses [`mpvpaper`](https://github.com/GhostNaN/mpvpaper)

## 📦 Dependencies

- [`mpvpaper`](https://github.com/GhostNaN/mpvpaper) – required to render background
- Linux with X11 or Wayland
- [Rust](https://www.rust-lang.org/tools/install) (only if building from source)

Make sure `mpvpaper` is installed and available in your `$PATH`.

## 🔧 Installation

Download the `sbgm` binary and move it to a directory in your `$PATH`:

```bash
chmod +x sbgm
sudo mv sbgm /usr/local/bin/
```

# ⚙️ Configuration

After the first run, you must configure SBGM using a JSON config file: ~/.config/sbgm/config.json

# Usage

Basic commands:

## Help
```bash
sbgm -h
```

## Draw background
```bash
sbgm
```
Or
```bash
sbgm --path "~/Videos/bg.mp4"
```

## Next Folder 
```bash
sbgm next-folder
```

## Next Background
```bash
sbgm next
```

## Prev Folder
```bash
sbgm prev
```

## Prev Background
```bash
sbgm prev
```
