<h1 align="center">&nbsp;<br><i>FFUU</i><br>&nbsp;</h1>

The static site generator for people who hate static site generators.

FFUU transforms Markdown and HTML pieces into a complete HTML website. It
pipes the contents of custom HTML elements in the Markdown through any
CLI tool.

Things FFUU doesn't have:

- **No configuration files.** But there are some command line arguments. And I hate them.
- **No dependencies.** Download the binary. Run it.
- **No plugins.** Just use any CLI tool in your Markdown.
- **No template language.** WIP still figuring out the alternative.
- **No taxonomies.** WIP still figuring out the alternative. 
- **No query languages.** WIP still figuring out the alternative.

## Example

```sh
$ ffuu ./posts ./dist
$ open ./dist/first-post.html
```

```markdown
<!-- ./posts/first-post.md -->

<head>
  <title>Blog * My First Post</title>
  <link href="./first-post.css" rel="stylesheet">
</head>

# My First Post

Hello, I'm the worst.

Here's a helpful diagram:

<svgbob args="--font-size 32">
  Me  -----> The Worst
</svgbob>

[Don't forget to checkout my second post :D](./second-post.md)
```

