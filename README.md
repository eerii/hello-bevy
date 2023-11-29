# hello bevy 🦆

an opinionated [bevy](https://github.com/bevyengine/bevy) template for my projects.

### features 🌿

- uses bevy 0.12 and has curated plugin support
- ci that checks errors and lint
- creates binaries for web, linux, mac and windows when adding a tag
- deploy to itch
- debug features integrated (inspector, schedule graphs)

### how to use it ✨

- clone this template into a new project
- search for '[CHANGE]' and make the necessary adjustments
- done c:

### runing locally 🌺

this project is configured to use dynamic linking and fast recompiling by default.
in order to have the fastest compile, you need to install [mold](https://github.com/rui314/mold) and use rust nightly (`rustup default nightly`).
however, if you want to change this settings go to [.cargo/config](.cargo/config) and follow the instructions, or remove it to disable optimizations all together.

to run a debug build use:

```sh
cargo run
```

and to start a local web build, use trunk (_this won't work until bevy#10157 is released on 0.12.1_):

```sh
trunk serve
```

### release 🌻

trigger manually on the actions page or add a tag like '[anything]0.1' with the version you want.

```sh
git tag -a "hello-bevy-0.1" -m "test release"
git push --tags
```

if you want to deploy to itch, go to the repository settings > secrets > actions and add:

```
ITCH_API_KEY = [your api key]
```

### other projects 💖

this is heavily based on [NiklasEi/bevy_game_template](https://github.com/NiklasEi/bevy_game_template) and [bevyengine/bevy_github_ci_template](https://github.com/bevyengine/bevy_github_ci_template). please use those for more general templates with community support.

### license 📝

this project is dual licensed under MIT and Apache 2.0, do what you want with it!
