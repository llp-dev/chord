**anstyle_query > windows**

# Module: windows

## Contents

**Functions**

- [`enable_ansi_colors`](#enable_ansi_colors) - Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences))

---

## anstyle_query::windows::enable_ansi_colors

*Function*

Enable ANSI escape codes ([`ENABLE_VIRTUAL_TERMINAL_PROCESSING`](https://learn.microsoft.com/en-us/windows/console/console-virtual-terminal-sequences#output-sequences))

For non-windows systems, returns `None`

```rust
fn enable_ansi_colors() -> Option<bool>
```



