# Biiwadi

Barebones Idle Inhibitor With A DBus Interface (for Wayland compositors).

Originally based on [Vigiland](https://github.com/Jappie3/vigiland).

## Dependencies

- Rust
- A compositor which supports the `idle-inhibit-unstable-v1` protocol.

## Building

```bash
cargo build --release
```

## Usage

Once launched, the application will serve the following DBus interface on the session bus at `/st/contraptioni/IdleInhibitor`:

```xml
<interface name="st.contraptioni.IdleInhibitor1">
    <method name="EnableInhibitor">
    <arg type="b" direction="out"/>
    </method>
    <method name="DisableInhibitor">
    <arg type="b" direction="out"/>
    </method>
    <method name="ToggleInhibitor">
    <arg type="b" direction="out"/>
    </method>
    <property name="IsInhibitorActive" type="b" access="read"/>
</interface>
```

### Note
- The `EnableInhibitor` and `DisableInhibitor` methods will return `false` if the inhibitor was already in the requested state, `true` otherwise
- The `ToggleInhibitor` method will return `true` if the inhibitor was enabled, `false` if it was disabled
- The `IsInhibitorActive` method will return `true` if the inhibitor is currently enabled, `false` otherwise
