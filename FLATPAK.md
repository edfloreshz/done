# Flatpak

## Dependencies
- org.gnome.Platform 43
- org.freedesktop.Sdk.Extension.rust-stable

Install the following dependencies:
```
flatpak install --runtime org.gnome.Platform org.freedesktop.Sdk.Extension.rust-stable
```

## Build

#### Development
To build the development version of the app for Flatpak:
```bash
flatpak-builder flatpak_build build-aux/dev.edfloreshz.Done.Devel.json
```

#### Release
To build the release version of the app for Flatpak:
```bash
flatpak-builder flatpak_build build-aux/dev.edfloreshz.Done.json
```

## Test the build
To verify that the build was successful, run the following:

#### Development
```bash
$ flatpak-builder --user --install --force-clean flatpak_build build-aux/dev.edfloreshz.Done.Devel.json
$ flatpak run dev.edfloreshz.Done.Devel
```

#### Release
```bash
$ flatpak-builder --user --install --force-clean flatpak_build build-aux/dev.edfloreshz.Done.json
$ flatpak run dev.edfloreshz.Done
```
