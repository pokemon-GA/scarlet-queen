# Documents

> [!WARNING]
> When editing docs for the first time, let it add `mdbook`

install command is under the command

```sh
cargo install mdbook
```

- watch docs

```sh
mdbook watch -o ./docs
```

- build docs

```sh
mdbook build ./docs
```

## added plugins

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
