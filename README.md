
## Scratch

WIP A web crawler DSL in Rust.

## Examples

Writing a single field
```
navigateTo('http://www.example.com') {
  title = from(h1 having class(title)) getTextContent;
  write(title)
}
```

Iterating and writing content
```
navigateTo('http://www.example.com') {
  in(table having id('')) {
    question = from(tr having class='question');
    answer = from(tr having class='question');
    write(title)
  }
}
```

### Contributing

- Fork the repository
- Make changes and add appropriate tests
- Run the test with `cargo test`
- Raise a PR

### TODO - Roadmap
- [] Finalize the grammar
- [] Work up to MVP

### Inspiration
Heavily inspired by this project [crawler](https://github.com/bplawler/crawler)
