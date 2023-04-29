const path = require('path');

module.exports = {
  entry: './static/js/app.js',
  output: {
    path: path.resolve(__dirname, 'public/static'),
    filename: 'bundle.js',
  },
};