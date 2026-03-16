# Code Style

## Style Guide

- Functions should be kept under 100 lines. If they exceed 100 lines, split them.
- Public-facing doc comments should follow rustdoc style as shown below.

````rust
    /// Returns the contained [`Ok`] value or a provided default.
    ///
    /// Arguments passed to `unwrap_or` are eagerly evaluated; if you are passing
    /// the result of a function call, it is recommended to use [`unwrap_or_else`],
    /// which is lazily evaluated.
    ///
    /// [`unwrap_or_else`]: Result::unwrap_or_else
    ///
    /// # Examples
    ///
    /// ```
    /// let default = 2;
    /// let x: Result<u32, &str> = Ok(9);
    /// assert_eq!(x.unwrap_or(default), 9);
    ///
    /// let x: Result<u32, &str> = Err("error");
    /// assert_eq!(x.unwrap_or(default), default);
    /// ```
````

## Library Guide

- Use `anyhow` and `thiserror` for error handling.
- Use `tracing` for logging.
- If you want to simulate inheritance, use `enum_dispatch`.
