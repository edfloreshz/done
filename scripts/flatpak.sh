#!/bin/sh

rm -rf ../_build/
cd ..
meson build
meson setup _build .
meson dist -C _build/
cd _build/meson-dist/ || exit
nautilus .