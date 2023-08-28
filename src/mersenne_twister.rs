// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

const N         : usize =  624      ;
const M         : usize =  397      ;
const MATRIX_A  : usize = 0x9908b0df;
const UPPER_MASK: usize = 0x80000000;
const LOWER_MASK: usize = 0x7fffffff;

pub struct MersenneTwister {
    pub _index        : usize,
    
    pub _btw_start_pos: usize,
    pub _btw_end_pos  : usize,
    pub _seed         : usize,

    pub _state        : Vec<usize>
}

impl Default for MersenneTwister {
    fn default() -> Self {
        MersenneTwister { 
            _index        : 0,
            _btw_start_pos: 0, 
            _btw_end_pos  : 0, 
            _seed         : 0, 
            _state        : vec![]
        }
    }
}

impl MersenneTwister {
    pub fn init_by_pos(&mut self, start: usize, end: usize) {
        self.init();
        self._btw_start_pos = start;
        self._btw_end_pos   = end;
        // you need to call init_state() when seed is applied.
    }

    pub fn init_by_seed(&mut self, seed: usize) {
        self.init();
        self._seed = seed;
        self.init_state();
    }

    pub fn init(&mut self) {
        self._state = vec![0; 625];
        // you need to call init_state() when seed is applied.
    }

    pub fn take(&mut self) -> usize {
        if self._index >= N {
            self._twist();
        }

        let mut y: usize = self._state[self._index];
        self._index += 1;

        y ^= (y >> 11) & 0xffffffff;
        y ^= (y << 7) & 0x9d2c5680;
        y ^= (y << 15) & 0xefc60000;
        y ^= y >> 18;

        y % (self._btw_end_pos - self._btw_start_pos + 1) + self._btw_start_pos
    } 

    pub fn init_state(&mut self) {
        self._state[0] = self._seed;

        for i in 1..N {
            self._state[i] = 1812433253usize
                                .wrapping_mul(self._state[i - 1] ^ (self._state[i - 1] >> 30))
                                .try_into()
                                .unwrap();
        }

        self._index = N;
    }

    fn _twist(&mut self) {
        for i in 0..N {
            let x     : usize = (self._state[i] & UPPER_MASK) + (self._state[(i + 1) % N] & LOWER_MASK);
            let mut xa: usize = x >> 1;
            
            if x % 2 != 0 {
                xa ^= MATRIX_A;
            }

            self._state[i] = self._state[(i + M) % N] ^ xa;
        }

        self._index = 0;
    } 
}