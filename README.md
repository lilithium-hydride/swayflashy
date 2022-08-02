# swayflashy
Briefly "flash" the opacity of Sway windows on focus. All parameters of the animation are
fully configurable, including easing animations provided by
[`keyframe`](https://github.com/HannesMann/keyframe). There are not yet any command-line switches
or configuration files; all configuration is done by modifying the constants near the top of
`main.rs`.

In X11, this effect is implemented at the compositor level. Wayland does not presently feature any
way for "eye candy programs" to hook in and add effects such as blur and animated transparency, so
`swayflashy` spams Sway with `opacity` commands over IPC in order to simulate intentionally 
attention-grabbing focus animations. Despite its spammy nature, CPU usage is minimal.

### Building
```shell
git clone https://github.com/lilithium-hydride/swayflashy
cd swayflashy
cargo build --release
# The binary will be located at `target/release/swayflashy`
```