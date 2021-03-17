# hbs-wasm

WASM handlebars template engine bindings, to be used with NodeJS

```
@param {string} handlebars template string
@param {any} context to render template
@param {boolean | undefined} minify (true by default)
@returns {string}
```

Usage:

```
  const { render } = require('hbs-wasm')

  const rendered = render('<h1>{{example}}<h1>', { example: 'this is my template' })
```
