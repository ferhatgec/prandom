// MIT License
//
// Copyright (c) 2023 Ferhat Geçdoğan All Rights Reserved.
// Distributed under the terms of the MIT License.
//
//

pub mod linear_congruential;
pub mod mersenne_twister;
pub mod multiply_with_carry;

#[cfg(test)]
mod tests {
    use crate::{ 
        linear_congruential::LinearCongruential, 
        mersenne_twister::MersenneTwister, 
        multiply_with_carry::MultiplyWithCarry
    };

    use std::time::{ 
        SystemTime, 
        UNIX_EPOCH 
    };

    #[test]
    fn linear_congruential_gen() {
        let mut x: LinearCongruential = LinearCongruential::default();
        x.init_by_pos(1, 15);
        
        let since_the_epoch = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("").as_millis();

        x.init_by_seed(since_the_epoch as usize);
        println!("linear_congruential_gen() = {}", x.take());
    }

    #[test]
    fn mersenne_twister_gen() {
        let mut x: MersenneTwister = MersenneTwister::default();
        x.init_by_pos(1, 199);

        let since_the_epoch = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("").as_millis();

        x.init_by_seed(since_the_epoch as usize);
        println!("mersenne_twister_gen() = {}", x.take());
    }

    #[test]
    fn multiply_with_carry_gen() {
        let mut x: MultiplyWithCarry = MultiplyWithCarry::default();
        x.init_by_pos(1, 199);

        let since_the_epoch = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .expect("").as_millis();

        x.init_by_seed(since_the_epoch as usize);
        println!("multiply_with_carry_gen() = {}", x.take());
    }
}
