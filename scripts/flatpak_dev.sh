#!/bin/sh

rm -rf ../_build/
cd ..
meson setup -Dprofile=development _build .
meson dist -C _build/ --allow-dirty
cd _build/meson-dist/ || exit
nautilus .