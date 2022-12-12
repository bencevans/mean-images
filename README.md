# mean-images

Calculate Mean &amp; Standard Deviation of an Image Dataset

## Install & Run

```
$ cargo install --git https://github.com/bencevans/mean-images
$ time mean-images ~/Datasets/ENA24/ena24 # 8,790 images on M1 MacBook Air (2020)
mean: 90.43575, 95.387474, 78.83858 std: 56.035336, 56.852406, 54.175068 # out of 255 (RGB)
mean: 0.35465002, 0.37406853, 0.3091709 std: 0.21974641, 0.22295061, 0.21245125 # normalised to out of 1.0 (RGB)
mean-images ~/Datasets/ENA24/ena24  695.28s user 27.52s system 748% cpu 1:36.57 total # = 0.079 seconds per image
```

## License

MIT
