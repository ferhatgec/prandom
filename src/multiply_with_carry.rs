// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

const C: usize = 6364136223846793005usize;

pub struct MultiplyWithCarry {
    pub _btw_start_pos: usize,
    pub _btw_end_pos  : usize,
    pub _seed         : usize
}

impl Default for MultiplyWithCarry {
    fn default() -> Self {
        MultiplyWithCarry { 
            _btw_start_pos: 0, 
            _btw_end_pos  : 0, 
            _seed         : 0 
        }
    }
}

impl MultiplyWithCarry {
    pub fn init_by_pos(&mut self, start: usize, end: usize) {
        self._btw_start_pos = start;
        self._btw_end_pos   = end;
    }

    pub fn init_by_seed(&mut self, seed: usize) {
        self._seed = seed;
    }

    pub fn take(&mut self) -> usize {
        self._seed = (<usize as TryInto<usize>>::try_into(C.wrapping_mul(self._seed)).unwrap()) + 1usize;
        
        self._seed % (self._btw_end_pos - self._btw_start_pos + 1) + self._btw_start_pos
    }
}