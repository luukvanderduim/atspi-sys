# atspi-sys

Gir generated raw bindings for the atspi accessibility library for Rust

The accessbility library Atspi, part of AT-SPI2-CORE, is a C library which you can use to create client programs to aid people in using their computers more conveniently.

AT-SPI2-CORE, and thus Atspi, can be found here: <https://gitlab.gnome.org/GNOME/at-spi2-core>
The Atspi API reference guide can be found here: <https://developer.gnome.org/libatspi/stable/ch01.html>

The bindings provided here are generated against Mint 19.x / Ubuntu 18.04.x which use AT-SPI2-CORE 2.28.0-1

This crate was automatically generated by gir. <https://github.com/gtk-rs/gir>
If you find yourself with a C library which is glib dependent and want to write rust, turn to gir and follow the tutorial.

Aside from the bindigns you will also find the modified gir file used to generate the bindings.

# post gir fixes

atspi-sys needs libdbus-sys:

extern crate libdbus_sys as dbus;
Added libdbus dependency in Gir.toml

There is likely a better way. If you know how, please don't hesitate to tell me or rather send in a pull request.

timeval is used but the g_ir file does not say where to find it(?)
timeval.rs takes timeval from the nix crate

The _AtspiAccessible is used but never defined.
accessible.rs creates an alias.
