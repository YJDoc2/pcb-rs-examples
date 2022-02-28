'use strict';
// instructions are :
// 0 : stop

// mov
// 1 : read next byte addr and store in reg1
// 2 : read next byte addr and store in reg2
// 3 : write reg1 to next addr
// 4 : write reg2 to next addr

// mvi
// 5 : store next byte in reg1
// 6 : store next byte in reg2
// 7 : read next byte as addr, and store next-to-next byte at addr
// 8 : xchg

// add, sub
// 9 : add reg1 and reg2 and store in reg1
// 10 : add next byte to reg1
// 11 : sub reg2 from reg1 and store in reg1
// 12 : sub next byte from reg1

// jmps
// 13 : jump to next byte if zero
// 14 : jump to next byte if not zero
// 15 : jump if reg1 less than reg2
// 16 : jump if reg1 greater than reg2

// 255 : nop

function get_num(s) {
  let v = parseInt(s);
  if (isNaN(v)) {
    throw 'invalid code ' + s;
  }
  return v;
}

export default function compile(code) {
  let _code = code.toLowerCase();
  let codelines = _code.split('\n');
  let compiled = [];

  for (let n in codelines) {
    if (codelines[n].trim() == '') {
      continue;
    }

    let three_ac = codelines[n].split(' ');

    if (three_ac.length > 3) {
      throw 'Invalid code';
    }

    let cmd = three_ac[0];

    switch (cmd) {
      case 'hlt': {
        compiled.push(0);
        continue;
      }

      case 'mov': {
        let op1 = three_ac[1];
        let op2 = three_ac[2];
        // this is register
        if (op1 == 'a') {
          let addr = get_num(op2);
          compiled.push(1);
          compiled.push(addr);
          continue;
        }
        // this is register
        if (op1 == 'b') {
          let addr = get_num(op2);
          compiled.push(2);
          compiled.push(addr);
          continue;
        }
        // op1 is an addr
        let addr = get_num(op1);
        if (op2 == 'a') {
          compiled.push(3);
          compiled.push(addr);
          continue;
        }
        if (op2 == 'b') {
          compiled.push(4);
          compiled.push(addr);
          continue;
        }
      }

      case 'mvi': {
        let op1 = three_ac[1];
        let op2 = three_ac[2];
        // this is register
        if (op1 == 'a') {
          let v = get_num(op2);
          compiled.push(5);
          compiled.push(v);
          continue;
        }
        // this is register
        if (op1 == 'b') {
          let v = get_num(op2);
          compiled.push(6);
          compiled.push(v);
          continue;
        }
        // op1 is an addr
        let addr = get_num(op1);
        let v = get_num(op2);
        compiled.push(7);
        compiled.push(addr);
        compiled.push(v);
        continue;
      }

      case 'xchg': {
        compiled.push(8);
        continue;
      }

      case 'add': {
        compiled.push(9);
        continue;
      }

      case 'adi': {
        let v = get_num(three_ac[1]);
        compiled.push(10);
        compiled.push(v);
        continue;
      }

      case 'sub': {
        compiled.push(11);
        continue;
      }

      case 'sbi': {
        let v = get_num(three_ac[1]);
        compiled.push(12);
        compiled.push(v);
        continue;
      }

      case 'jz': {
        let addr = get_num(three_ac[1]);
        compiled.push(13);
        compiled.push(addr);
        continue;
      }

      case 'jnz': {
        let addr = get_num(three_ac[1]);
        compiled.push(14);
        compiled.push(addr);
        continue;
      }

      case 'jl': {
        let addr = get_num(three_ac[1]);
        compiled.push(15);
        compiled.push(addr);
        continue;
      }

      case 'jg': {
        let addr = get_num(three_ac[1]);
        compiled.push(16);
        compiled.push(addr);
        continue;
      }

      case 'nop': {
        compiled.push(17);
        continue;
      }

      default: {
        throw 'invalid code ' + codelines[n];
      }
    }
  }
  return compiled;
}

let code = `
mvi a 0
mvi b 5
add
xchg
sbi 1
jz 13
xchg
jnz 4
xchg
mov 52 a
hlt
`;
// console.log(compile(code));
