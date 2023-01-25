#!/bin/sh

rm -rf ../_build/
cd ..
meson setup -Dprofile=development _build .
meson dist -C _build/
cd _build/meson-dist/ || exit
nautilus .