mod args;
mod hotkey;

use core::time;
use std::{fs, thread};

use anyhow::{Context, Result};
use clap::Parser;
use rdev::{display_size, listen, simulate, Event, EventType, Key};

use crate::hotkey::Outer;

fn main() -> Result<()> {
    let args = args::Args::parse();

    if args.verbose {
        let display_size = display_size().unwrap_or_default();
        println!("Display size is {}x{}", display_size.0, display_size.1)
    }

    let config_dir = dirs::config_dir()
        .context("Could not resolve config directory")?
        .join("keeb");
    let hotkey_file = config_dir.join(format!("{}.toml", args.preset.unwrap()));

    if !config_dir.exists() {
        if args.verbose {
            println!("Creating config directory")
        }
        fs::create_dir_all(&config_dir).with_context(|| format!("Could not create directories"))?;
    }

    if !hotkey_file.exists() {
        if args.verbose {
            println!("Creating hotkey file")
        }
        fs::write(&hotkey_file, DEFAULT_HOTKEY_FILE)
            .with_context(|| format!("Could not write hotkey file"))?;
    }

    let hotkeys_outer: Outer = hotkey::Outer::from_path(&hotkey_file.to_str().unwrap())?;

    if args.verbose {
        println!("Loaded hotkey file: {:?}", hotkey_file);
        if &hotkeys_outer.hotkeys.len() <= &0 {
            println!("No defined hotkeys");
        } else {
            println!("Defined hotkeys:");
            for hotkey in &hotkeys_outer.hotkeys {
                if let Some(hotkey) = hotkey {
                    println!("{:?}", hotkey)
                }
            }
        }
    }

    let mut mouse_position = (0.0, 0.0);
    let mut paused = args.paused;

    let callback = move |event: Event| match event.event_type {
        EventType::MouseMove { x, y } => {
            if args.watch && !paused {
                println!("{:?}", event);
            }
            mouse_position = (x, y)
        }
        EventType::KeyPress(key) => {
            if key == Key::Pause {
                paused = {
                    if paused {
                        if args.verbose {
                            println!("Unpaused listening")
                        }
                        false
                    } else {
                        if args.verbose {
                            println!("Paused listening")
                        }
                        true
                    }
                };
            }

            if paused {
                return;
            }

            if args.watch {
                println!("{:?}", event);
            }

            let hotkeys = &hotkeys_outer.hotkeys;
            for hotkey in hotkeys {
                if let Some(hotkey) = hotkey {
                    if hotkey.key == key {
                        let hk = hotkey;

                        if args.verbose {
                            println!(
                                "Hit {:?}, sending {:?} at {:?}...",
                                &hk.key, &hk.button, &hk.position
                            )
                        }

                        send(&EventType::MouseMove {
                            x: hk.position.x,
                            y: hk.position.y,
                        });

                        send(&EventType::ButtonPress(hk.button));
                        send(&EventType::ButtonRelease(hk.button));

                        send(&EventType::MouseMove {
                            x: mouse_position.0,
                            y: mouse_position.1,
                        });
                    }
                }
            }
        }
        _ => {
            if args.watch && !paused {
                println!("{:?}", event);
            }
        }
    };

    if let Err(e) = listen(callback) {
        eprintln!("{:?}", e)
    }

    Ok(())
}

fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    if let Err(e) = simulate(event_type) {
        eprintln!("{}", e);
    }

    thread::sleep(delay);
}

const DEFAULT_HOTKEY_FILE: &str = r#"# Default generated hotkey file
#
# [[hotkey]]
# key = "KeyQ"
# button = "Left"
# position = { x = 50, y = 10 }
"#;
