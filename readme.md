## requirments
 - nix with flakes enabled

 I have only tested on a x86 linux system

## demo
[![asciicast](https://asciinema.org/a/wImKjqJXhhcMKxgtlKaiVLcoC.svg)](https://asciinema.org/a/wImKjqJXhhcMKxgtlKaiVLcoC)

## usage

oneshoot: 
```shell
nix run github:GlennWSo/consid -- --help
```

>finds largest sequence of numbers within a tolerance from a random source
>
>Usage: consid [OPTIONS] -t <TOL> --lb <LB> --width <WIDTH>
>
>Options:
  -t <TOL>             tolerance of minmax differance in window scan
  -l, --lb <LB>        low bound for rng numbers
  -w, --width <WIDTH>  rng span(high bound - low bound)
  -c, --count <COUNT>  number values to generate/scan for [default: 1000]
  -s, --seed <SEED>    
  -p, --print-rng      prints debug information
  -h, --help           Print help
  -V, --version        Print version
