# hello bevy 🦆

an opinionated [bevy](https://github.com/bevyengine/bevy) template for my projects.

<p float="left">
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/5f736632-75ac-40ef-bd8e-3a66dc57a68a" height="150px" />
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/f20dd7f6-3d91-4f22-8291-af287a12eaa3" height="150px" />
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/ed8f0c5c-1ca9-41ee-aa48-ab97c69b3887" height="150px" />
  <img src="https://github.com/eerii/hello-bevy/assets/22449369/d4207538-73cb-4bb9-b463-9b345887a118" height="150px" />
</p>

### features 🌿

- uses bevy main (0.13) and has curated plugin support
- ci that checks errors and lint
- creates binaries for web, linux, mac and windows when releasing a tag
- deploy to itch automatically
- remapable input manager with gamepad support
- easy debuggins features (inspector, fps counter, tracy script, hot reloading)
- asset loading with progress, audio, saving, menu...

### how to use it ✨

- use this template in a new project (on github, a green button on the top right)
- search for '[CHANGE]' and make the necessary adjustments
- done c:

### runing locally 🌺

this project is configured to use dynamic linking and fast recompiling by default.
in order to have the fastest compile, you may install [mold](https://github.com/rui314/mold) and use rust nightly (`rustup default nightly`).
if you don't want some of these features, go to [.cargo/config](.cargo/config) and follow the instructions, or remove it to disable optimizations all together.

to run a debug build use cargo:

```sh
cargo run
```

and to start a local web build, use trunk:

```sh
trunk serve
```

you can also play around with some of the included examples with `cargo run --example <name>`.

### release 🌻

in order to create a release build with binaries for all platforms you have two options: either you trigger it manually on the actions page or you add a tag like '[anything]0.1' with the version you want.

```sh
git tag -a "hello-bevy-0.1" -m "test release"
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

### tracing 📈

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

this is heavily based on [NiklasEi/bevy_game_template](https://github.com/NiklasEi/bevy_game_template) and [bevyengine/bevy_github_ci_template](https://github.com/bevyengine/bevy_github_ci_template). please use those for more general templates that are more robust and have community support. hello bevy is hardly tested and very tailored to my preferences.

### plugins 🪴

i tried to be very intentional with all the plugins in this template. there is an amazing community that creates tons of useful tools, and some of them have become essential when making games with bevy:

- [bevy_asset_loader](https://github.com/NiklasEi/bevy_asset_loader): easier asset handling with collections
- [bevy_embedded_assets](https://github.com/vleue/bevy_embedded_assets): puts assets inside the binary (only on release)
- [iyes_progress](https://github.com/IyesGames/iyes_progress): tracks progress (used for an accurate loading screen)
- [bevy_kira_audio](https://github.com/NiklasEi/bevy_kira_audio): improved audio library with more features
- [bevy-persistent](https://github.com/umut-sahin/bevy-persistent): save and load any resource on disk

there are also nice tools for debugging:

- [bevy-inspector-egui](https://github.com/jakobhellermann/bevy-inspector-egui): imgui-like interface where you can see entities and components in real time (press I)

### license 📝

this project is dual licensed under MIT and Apache 2.0, do what you want with it!
