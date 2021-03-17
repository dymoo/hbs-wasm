const { render } = require('./pkg/hbs_wasm');

const rendered = render('<h1>{{example}}<h1>', {
  example: 'this is my template',
});

console.log(rendered);
