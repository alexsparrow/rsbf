#![feature(str_char)]

use std::io::prelude::*;

pub struct State {
    pc: usize,
    cell: usize,
    mem: [u8; 1024],
    stack: Vec<usize>,
}

pub fn initial_state() -> State {
    State {
        pc: 0, cell: 0, mem: [0; 1024], stack: Vec::new(),
    }
}

fn plus(state: &mut State) {
    state.mem[state.cell] += 1;
    state.pc += 1;
}

fn minus(state: &mut State) {
    if state.mem[state.cell] > 0 {
        state.mem[state.cell] -= 1;
    }
    state.pc += 1;
}

fn left(state: &mut State) {
    state.cell -= 1;
    state.pc += 1;
}

fn right(state: &mut State) {
    state.cell += 1;
    state.pc += 1;
}

fn output(state: &mut State, write: &mut Write) {
    write.write(&[state.mem[state.cell]]).unwrap();
    state.pc += 1;
}

fn loop_start(state: &mut State, code: &str) {
    state.stack.push(state.pc);

    if state.mem[state.cell] == 0 {
        while state.pc < code.len() {
            if code.char_at(state.pc) == ']' {
                return;
            }
        }
    }
    else {
        state.pc += 1;
    }
}

fn loop_end(state: &mut State) {
    let pos = state.stack.pop();

    if state.mem[state.cell] != 0 {
        state.pc = pos.unwrap();
    }
    else {
        state.pc += 1;
    }

}

pub fn interpret(s:&str, state: &mut State, write: &mut Write) {
    while state.pc < s.len() {
        let c = s.char_at(state.pc);

        match c {
            '+' => plus(state),
            '-' => minus(state),
            '<' => left(state),
            '>' => right(state),
            '.' => output(state, write),
            '[' => loop_start(state, s),
            ']' => loop_end(state),
            ',' => panic!("Not implemented"),
            _ => state.pc += 1
        }
    }

}
