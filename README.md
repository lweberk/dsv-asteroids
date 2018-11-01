Environment Setup
=================
This game depends on the csfml library binaries and headers to compile and run.

On Debian and derivates you just have to install libcsfml-dev (version 2.4.x, currently
available on buster/testing) which will in turn pull the libcsfml libraries as well as
the original C++ variant libsfml-*.

```bash
sudo apt-get install libcsfml-dev
```

The rust bindings of sfml are shipped as a submodule within this repository for sake of
pinning on specific commits of upstream which have yet to be published as crates. This
means that after cloning this repository you have to also checkout the submodule source
tree.

```bash
git submodule update --init --recursive
```

At this point the only thing that is left is to just build and run! Have fun tinkering!