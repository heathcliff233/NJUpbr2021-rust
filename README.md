# NJU pbr 2021[![Build Status](https://travis-ci.com/heathcliff233/NJUpbr2021-rust.svg?branch=main)](https://travis-ci.com/heathcliff233/NJUpbr2021-rust)
This is a final project for NJU physically based rendering 2021 Sping.
It is implemented in Rust and is by no means a mature renderer.

This project derives from [@gkmngrgn](https://github.com/gkmngrgn) 's work 
of implementing Peter Shirley's Ray Tracing trilogy in multiple
languages. The previous project stopped at book 1 [Raytracing in a 
weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) .

Core Features:
* Parallelism enabled by the fantastic Rust crate `rayon`.
* Convert source pixels of a logo image to a rendered one in 3D space.

Usage:
* `cargo build --release`
* save logo to assets/base.png, run `python3 proc.py`. note that opencv-python 
and numpy packages are desired.
* `mkdir result && time ./target/release/one_weekend > result/one_weekend.ppm`
  for book one weekend, and `time ./target/release/custom_img > result/nju.ppm`
  for customized logo. 
  