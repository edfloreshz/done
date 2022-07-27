#!/bin/sh

cd ..
meson setup -C _build .
meson dist -C _build/
cd _build/meson-dist/
nautilus .