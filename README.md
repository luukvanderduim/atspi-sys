# atspi-sys

Gir generated raw bindings for the atspi accessibility library for Rust

The accessibility library libatspi, part of AT-SPI2-CORE, is a C library for making client programs to aid people in using their computers more conveniently.

AT-SPI2-CORE, and thus libatspi, can be found here: <https://gitlab.gnome.org/GNOME/at-spi2-core>
The Atspi API reference guide can be found here: <https://developer.gnome.org/libatspi/stable/ch01.html>

The bindings provided here are generated against XUbuntu 19.10.x which uses AT-SPI2-CORE 2.34.0-2 / these bindings may work on earlier iterations of the library but this is not tested.

This crate was automatically generated by gir. <https://github.com/gtk-rs/gir>
If you find yourself with a C library which is glib dependent and want to write rust, turn to gir and follow the tutorial.

Aside from the bindings you will also find the gir file used to generate the bindings.

gir-files-18.04: corresponds with the Ubuntu 18.04 release. The Atspi-2.0.gir is modified to use libdbus and has a potion removed.

gir-files-19.10: corresponds with the Ubuntu 18.04 release. The Atspi-2.0.gir file was modified to use Gio.DBusConnection and Gio.DBusServer for their corresponding GDBusConnection* and GDBusServer* respectively.

timeval is used but the g_ir file does not say where to find it(?)
timevalue.rs takes timeval from glib-sys.

The _AtspiAccessible is used but never defined.
accessible.rs creates an alias.