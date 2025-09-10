# scarlet-queen Documentation (mdbook)

project directory: `docs/`

> [!IMPORTANT]
> When editing docs for the first time, let it add `mdbook`

install command is under the command

```sh
cargo install mdbook
```

- start dev server

> [!TIP]
> This commands cannot deal with hot reloading. You have to reload manually, when you change the docs.

```sh
mdbook watch -o ./docs
```

- build docs

```sh
mdbook build ./docs
```

**mdbook plugins**

- mdbook-mermaid

You can use mermaid diagrams in this docs.

example

~~~md
```mermaid
graph TD;
    A-->B;
    A-->C;
    B-->D;
    C-->D;
```
~~~
