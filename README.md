# haikei

a tiny wallpaper manager for Wayland/X11.

haikei is meant to be used as an interface for wallpaper utilities like [swww](https://github.com/LGFae/swww), [feh](#) or [hsetroot](#).

## :rocket: usage

- `haikei current` - get path to current wallpaper
- `haikei set <path>` - set wallpaper
- `haikei r [dir]` - set random wallpaper from current wallpaper's parent directory
- `haikei daemon [-f <config_file>]` - set wallpaper based on time of day

## installation

haikei can be installed like this:

```bash
git clone --depth 1 https://github.com/comfysage/haikei.git ~/haikei && ~/haikei
make install
```

or using [saku](https://github.com/comfysage/saku):

```bash
saku install haikei
```

## :gear: configuration

the daemon can be configured using a configuration file at `~/.config/haikei/daemon.json`.

```json
{
  "interval": 5, // interval (to check for time) in minutes
  "wallpapers": {
    "09.30": [
      "/home/comfy/media/pictures/wallpapers/anime/64565A6D61574A7856454D306445510A.png",
      "/home/kitchen/pictures/wallpapers/anime/4E6A493256334A584C305649536D4A0A.jpeg"
    ],
    "11.30": [
      "/home/kitchen/pictures/wallpapers/anime/4D325A444C323575544545785A46520A.png"
    ],
    "16.45": [
      "/home/kitchen/pictures/wallpapers/anime/656D4E534D476B7A5254685462454A0A.png",
      "/home/kitchen/pictures/wallpapers/anime/4D3142704D6E4A33627A30784D6A550A.png"
    ],
    "19.30": [
      "/home/kitchen/pictures/wallpapers/anime/52304A46554778496433686A4E6B520A.png",
      "/home/kitchen/pictures/wallpapers/anime/645752506343395762454E6B636E670A.png",
      "/home/kitchen/pictures/wallpapers/anime/4B324A70536C685A5253394B5379380A.png"
    ],
    "23.30": [
      "/home/kitchen/pictures/wallpapers/anime/56335A4E554868505A6A68345446590A.png"
    ]
  }
}
```
