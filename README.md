# hello bevy 🦆

an opinionated [bevy](https://github.com/bevyengine/bevy) template for my projects.

<p float="left">
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/c5b7cdcd-20d7-44e4-8a56-3a4122cdb5d7" height="250px" />
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/a3c6d2bb-7648-45da-9cb0-1257ee081a46" height="250px" />
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/7fa2176f-8dbe-4867-a6b2-33f366af122f" height="250px" />
</p>

### features 🌿

- uses bevy 0.14
- minimal and curated plugin support
- fully featured accesible menu with keyboard, mouse and gamepad navigation
- ci that checks errors and lint
- creates binaries for web, linux, mac and windows when releasing a tag
- deploy to itch automatically
- has a nix flake development shell for easy building

### how to use it ✨

- use this template in a new project (on github, a green button on the top right)

### runing locally 🌺

this project is configured to use dynamic linking for debug builds and fast recompiling by default.
in order to have the fastest compile, you may install [mold](https://github.com/rui314/mold) and use rust nightly (`rustup default nightly`).
if you don't want some of these features, go to [.cargo/config](.cargo/config) and follow the instructions, or remove it to disable optimizations all together.

to run a debug build use cargo:

```sh
cargo run
```

you can also play around with some of the included examples with `cargo run --example <name>`. and if you want to get started quickly, copy any example to `src/main.rs`!

if you have nix installed, running `nix develop` you get a shell with all the dependencies already installed.

### release 🌻

in order to create a release build with binaries for all platforms you have two options: either you trigger it manually on the actions page or you add a tag like '[anything]0.1' with the version you want.

```sh
git tag -a "0.1" -m "test release"
git push --tags
```

if you want to also deploy this build to itch, go to the repository settings > secrets > actions and add:

```
ITCH_API_KEY = [your api key]
```

to run a release build locally:

```sh
cargo run --release --no-default-features --features release
```

### profiling 📈

bevy has built in support for the [tracy](https://github.com/wolfpld/tracy) profiler. you can profile your game easily:

```sh
tracy-capture -o capture.tracy &
cargo run --release --no-default-features --features trace
```

and then view the result with:

```sh
tracy capture.tracy
```

### other projects 💖

this is heavily based on [TheBevyFlock/bevy_quickstart](https://github.com/TheBevyFlock/bevy_quickstart) and [NiklasEi/bevy_game_template](https://github.com/NiklasEi/bevy_game_template). please use these more general templates that are robust and have community support. hello bevy is hardly tested and very tailored to my preferences.

### plugins 🪴

this template intends to use as little external dependencies as possible to facilitate version updates and avoid bloat. that said, there are a few awesome community plugins that make everything as easy as possible.

- [leafwing-input-manager](https://github.com/Leafwing-Studios/leafwing-input-manager): an awesome way of handling input from multiple sources and create simple bindings
- [bevy_mod_picking](https://github.com/aevyrie/bevy_mod_picking): used to select things on the screen. only the ui picker is enabled by default, used for mouse navigation
- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui): optional and only enabled when using the `inspector` feature. it provides a very useful world inspector

### license 📝

this project is dual licensed under MIT and Apache 2.0, do what you want with it!

the files under assets may come from other sources and have different licenses:

- `icons/bevy.png` and `icons/pixelbevy.png` from [cart](https://github.com/bevyengine/bevy_github_ci_template/issues/45#issue-2022210264), **not** open
- `sounds/boing.ogg`, sound effect from [bigsoundbank.com](https://bigsoundbank.com/high-pitched-tom-1-s2329.html), [CC0](https://creativecommons.org/publicdomain/zero/1.0/)
- `music/rain.ogg`, sound effect from [bigsoundbank.com](https://bigsoundbank.com/summer-rain-on-terrace-s1019.html), [CC0](https://creativecommons.org/publicdomain/zero/1.0/)
- `fonts/pixel.ttf`, public pixel font from [ggbot](https://ggbot.itch.io/public-pixel-font), [CC0](https://creativecommons.org/publicdomain/zero/1.0/)
- `fonts/sans.tff`, outfit font from [google](https://fonts.google.com/specimen/Outfit), [OFL](https://scripts.sil.org/cms/scripts/page.php?site_id=nrsi&id=OFL)
