# Release

## Flatpak
To test the build for `Flatpak` and release to `Flathub`, head to [FLATPAK.md](FLATPAK.md).

## Arch Linux
Update the [PKGBUILD](PKGBUILD) to include the new release number in `pkgrel` and `pkgver`.

## Windows
To compile for Windows, we need to configure Docker:

### Setup
To install Docker in Ubuntu:
```bash
sudo apt install docker.io
sudo systemctl enable docker
sudo systemctl start docker
```

We'll make use of an image in DockerHub.
> We need this image to be updated to Fedora 38 before deploying to Windows again.
```bash
docker pull mglolenstine/gtk4-cross:rust-gtk-4.6
```

Once it downloads, we need to create a container inside the project:
```bash
docker run -ti -v $(pwd):/mnt mglolenstine/gtk4-cross:rust-gtk-4.6
```

Then, we need to install some dependencies:
```bash
sudo dnf -y install libadwaita-devel gtk4-devel
```

After that, run `build` to build the project and `package` to package it into a zip file.

A file called `package.zip` will be generated in the root directory, this will contain the `.exe`.

## macOS
macOS support is still on the way.
