# Attempts to generate animated pictures


## GIF
Generating animated GIF with ImageMagick is easy but fails when using more than ~20 images at high resolution:


## WebP
ImageMagick supports webp through cwebp delegation.
It requires installing cwebp tool from webp package:

```
$ sudo apt-get install webp
```

A WebP file can then be created using ImageMagick:

```
$ convert -delay 1 -loop 0 screenshots/000* replay.webp
```

But it will not be animated: cwebp doesn't support it


## APNG
ImageMagick accepts the following but just generates a PNG with the first image:

```
$ convert -delay 1 -loop 0 screenshots/000* replay.apng
$ file replay.apng
replay.apng: PNG image data, 2000 x 1400, 4-bit colormap, non-interlaced
```

## MNG
It seems to work:

```
$ convert -delay 1 -loop 0 screenshots/0000* replay.mng
```

But neither chrome nor firefox display it. Won't play in mplayer or vlc either.
Not sure if the file is valid or not ...


## FLIF
Here again, ImageMagick accepts the command but just generates a PNG:

```
$ convert -delay 1 -loop 0 screenshots/0000* replay.flif
$ file  replay.flif
replay.flif: PNG image data, 2000 x 1400, 4-bit colormap, non-interlaced
```


## MP4

```
$ avconv -i screenshots/%06d.png -r 100 -y -an replay.mp4
```

It requires files to be named consecutively (no gap).
With `-r` providing the delay between framse, `-y` overwriting if output file exists and `-an` deactivating audio.
It's also possible to start with an offset with `-start_number`

Resulting file weight 3GiB for 2000 pictures in 2000x1400 (cell width at 1).
