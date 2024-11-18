# `quikselect`: A quick and effective way of HTML parsing

If writing loads of repetitive selectors, methods and dealing with large number of unwraps and iterations are not for you, `quikselect` is the way to go.

## Getting started

Install via `Cargo.toml` by using the following line:

```toml
quikselect = {git = "https://github.com/The-HackeLite/quikselect}
```

and you're good to go.

## Usage

There are 2 versions of each function: With and without `try`. As the name implies, the version with `try` returns a `Result` or `Option` which will have to be accessed by pattern matching or by using `unwrap()`.

### Functions

1. `selector`: Returns a selector
2. `inner_html`: Returns the inner HTML content for the given document and selector.
3. `attr`: Returns the attribute for the given attribute, selector and document.
4. `text`: Returns the inner text upon provision of document and selector.
5. `element`: Provides a reference to the element upon provision of document and selector.
