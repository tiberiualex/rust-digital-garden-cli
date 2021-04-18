# Digital Garden

A CLI tool for the creation and maintenance of Digital Gardens. Code from [Egghead course](https://egghead.io/courses/creating-a-digital-garden-cli-with-rust-34b8).

## Commands

### write

Open a new file to write in our digital garden. Since we don't necessarily know what we want to title what we're writing, we'll leave the title as optional and ask the user for it later if they don't provide it up-front.

```shell
garden write
garden write -t "Some Title"
```
