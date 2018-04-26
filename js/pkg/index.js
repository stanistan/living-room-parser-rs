const parser = require('./src/parser');

module.exports = function parse(input) {
  let output = parser.parse(input);
  if (!output) {
    throw new Error("Could not parse input:" + input);
  } else {
    return JSON.parse(output);
  }
};
