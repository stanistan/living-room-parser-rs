import test from 'ava';
import parse from '../index';

test('parsing', t => {

  t.deepEqual([], parse(''), 'empty input');
  t.deepEqual([ word('hi')], parse('hi'), 'simple word');
  t.deepEqual([ variable('foo') ], parse('$foo'), 'simple var');

  t.deepEqual(
    ts()
    .w_('oh')
    .w_('hi')
    .v_('thing is quoted here')
    .h_()
    .v_(1.0)
    .variable('var').done(),
    parse('oh hi "thing is quoted here" _ 1.0 $var')
  );

});

test('whitespace', t => {
  t.deepEqual(
    ts().w_('hi').w('you').done(),
    parse('hi    you'),
    'white space gets collapsed'
  );
});

test('words', t => {
  // this exists to document *how* things are being parsed
  // for documentation purposes
  t.deepEqual(ts().w(',,,,').done(), parse(',,,,'));
  t.deepEqual(ts().w('a,y').done(), parse('a,y'));
  t.deepEqual(ts().w('a').h().done(), parse('a_'));
  t.deepEqual(
    ts().w('(').variable('a').w_(',').variable('b').w(')').done(),
    parse('($a, $b)')
  );
  t.deepEqual(ts().w_('hi.').w('you').done(), parse('hi. you'));
  t.deepEqual(ts().w('hi').v(0.1).done(), parse('hi.1'));
  t.deepEqual(ts().w('w').v(1).done(), parse('w1'));
  t.deepEqual(ts().w('w').v('').done(), parse('w""'));
  t.deepEqual(ts().id('').done(), parse('#'));
  t.deepEqual([ { wildcard: true } ], parse('$'));
});

test('FIXME quotes', t => {
  // these are weird/ambiguous grammar rules that
  // are currently being forgiven, but should error out
  // because they dont make sense and make parsing the grammar
  // more difficult in other languages
  t.deepEqual(ts().w('w"').done(), parse('w"'));
  t.deepEqual(ts().w('"').done(), parse('"'));
  t.deepEqual(ts().w('w"a').done(), parse('w"a'));
});

test('issue#2', t => {
  const half = [ value(0.5) ];
  t.deepEqual(half, parse('0.5'));
  t.deepEqual(half, parse('.5'));
  t.deepEqual([ value(10) ], parse('10'));

  const gorog = ts()
    .w_('gorog')
    .w_('is')
    .w_('at')
    .v(0.1)
    .w_(',')
    .v(5)
    .done();

  t.deepEqual(gorog, parse('gorog is at 0.1, 5'));
  t.deepEqual(gorog, parse('gorog is at .1, 5'));
});

///////////////////////////////////
// ... Helpers defined below ... //
///////////////////////////////////

const token = (k, v) => {
  var o = { };
  o[k] = v;
  return o;
};

const word = (v) => token('word', v);
const value = (v) => token('value', v);
const variable = (v) => token('variable', v);
const id = (v) => token('id', v);

const hole = () => token('hole', true);
const space = () => word(' ');

class T {
  constructor() {
    this.tokens = [];
  }

  w(v) {
    this.tokens.push(word(v));
    return this;
  }

  w_(v) {
    return this.w(v)._();
  }

  v(v) {
    this.tokens.push(value(v));
    return this;
  }

  v_(v) {
    return this.v(v)._();
  }

  variable(v) {
    this.tokens.push(variable(v));
    return this;
  }

  variable_(v) {
    return this.variable(v)._();
  }

  h() {
    this.tokens.push(hole());
    return this;
  }

  h_() {
    return this.h()._();
  }

  _() {
    this.tokens.push(space());
    return this;
  }

  id(v) {
    this.tokens.push(id(v));
    return this;
  }

  id_(v) {
    return this.id(v)._();
  }

  done() {
    return this.tokens;
  }
}

function ts() {
  return new T();
}
