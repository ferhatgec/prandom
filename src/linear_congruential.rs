// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

const A: usize = 1664525   ;
const C: usize = 1013904223;
const M: usize = 1013904223;

pub struct LinearCongruential {
    pub _btw_start_pos: usize,
    pub _btw_end_pos  : usize,
    pub _seed         : usize
}

impl Default for LinearCongruential {
    fn default() -> Self {
        LinearCongruential {
            _btw_start_pos: 0,
            _btw_end_pos  : 0,
            _seed         : 0
        }
    }
}

impl LinearCongruential {
    pub fn init_by_pos(&mut self, start: usize, end: usize) {
        self._btw_start_pos = start;
        self._btw_end_pos   = end;
    }

    pub fn init_by_seed(&mut self, seed: usize) {
        self._seed = seed;
    }

    pub fn take(&mut self) -> usize {
        self._seed = (A * self._seed + C) % M;
        
        self._seed % (self._btw_end_pos - self._btw_start_pos + 1) + self._btw_start_pos
    }
}
