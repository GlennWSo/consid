## requirments
 - nix with flakes enabled

 I have only tested on a x86 linux system


## install

oneshoot:
nix run nix run github:GlennWSo/consid#default -- --help

ephemiral shell:
nix shell github:GlennWSo/cosid#default



## usage

> finds largest sequence of numbers within a tolerance from a random source

Usage: consid [OPTIONS] --lb <LB> --width <WIDTH>

Options:
  -s, --seed <SEED>    
  -p, --print-rng      prints debug information
      --lb <LB>        low bound for rng numbers
  -w, --width <WIDTH>  rng span(high bound - low bound)
  -c, --count <COUNT>  number values to generate/scan for [default: 1000]
  -t <TOL>             tolerance of minmax differance in window scan [default: 5]
  -h, --help           Print help
  -V, --version        Print version


