# keeb

Minimalist click simulation tool. Allows to map a key on the keyboard to a mouse click at specific coordinates in a human-readable TOML file. Works on Linux, Windows, and Mac.

## Configuration

By default, `keeb/hotkeys.toml` will be generated in your config directory (`~/.config` on Linux, `%USERPROFILE%/AppData/Roaming` on Windows, `~/Library/Application Support` on Mac). Here you can define default hotkeys that will be read if no other preset is specified (via `--preset` option). All presets must be located in the app config directory, e.g. `~/.config/keeb`.

A hotkey example:

```toml
[[hotkey]]
key = "KeyQ"
button = "Left"
position = { x = 50, y = 10 }
```

You can define as many hotkeys as you need by copy-pasting the provided example. In order to figure out values for the keys and cursor position, pass `--watch` flag to watch any keyboard or mouse events.

Example watch output:

```rust
Event { time: SystemTime { tv_sec: 1669819061, tv_nsec: 455732070 }, name: None, event_type: ButtonPress(Left) }
Event { time: SystemTime { tv_sec: 1669819061, tv_nsec: 562716614 }, name: None, event_type: ButtonRelease(Left) }
Event { time: SystemTime { tv_sec: 1669819062, tv_nsec: 418840847 }, name: None, event_type: MouseMove { x: 1088.0, y: 573.0 } }
Event { time: SystemTime { tv_sec: 1669819062, tv_nsec: 548842265 }, name: None, event_type: MouseMove { x: 1088.0, y: 572.0 } }
Event { time: SystemTime { tv_sec: 1669819063, tv_nsec: 49122706 }, name: Some("w"), event_type: KeyPress(KeyW) }
Event { time: SystemTime { tv_sec: 1669819063, tv_nsec: 171917399 }, name: None, event_type: KeyRelease(KeyW) }
```

Listening/watching can be paused or unpaused by hitting Pause key. Pass `--pause` flag to start paused.

## Compatability

| Tested  | Platform | Note                                                                                                                                    |
| ------- | -------- | --------------------------------------------------------------------------------------------------------------------------------------- |
| &#9745; | Linux    |                                                                                                                                         |
| &#9745; | Windows  | Might hang if relentlessly hitting hotkeys                                                                                              |
| &#9744; | Mac      | You might want to set accessibility permissions for keeb and use Latin input method ([issue](https://github.com/Narsil/rdev/issues/86)) |

## Use cases

This project might be useful when it comes to playing Android games in a virtual machine/emulator that doesn't provide keymapping capabilities, which inspired keeb to be made.

## License

keeb is licensed under the MIT License. Please consult the attached `LICENSE.md` file for further details.
