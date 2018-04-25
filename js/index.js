const parser = require('./parser');

module.exports = function parse(input) {
  return JSON.parse(parser.parse(input));
};
