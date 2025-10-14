use std::collections::BTreeMap;
use num::traits::{Zero, One,CheckedAdd,CheckedSub};



pub trait Config{
    type AccountId: Ord + Clone;
    type BlockNumber: Zero + One + Copy + CheckedAdd + CheckedSub;
    type Nonce: Zero + One +Copy;

}

#[derive(Debug)]
pub struct Pallet<T:Config>{
    block_number:T::BlockNumber,
    nonce:BTreeMap<T::AccountId,T::Nonce>,
}

impl<T:Config>Pallet<T>{
    pub fn new()->Self{
        Self{
            block_number:T::BlockNumber::zero(),
            nonce:BTreeMap::new()
        }
    }
    pub fn inc_block_number(&mut self){
        self.block_number = self.block_number.checked_add(&T::BlockNumber::one()).unwrap();
    }
    pub fn inc_nonce(&mut self,who: &T::AccountId){
        let nonce = *self.nonce.get(who).unwrap_or(&T::Nonce::zero());
        let new_nonce = nonce+T::Nonce::one();
        self.nonce.insert(who.clone(),new_nonce);
    }
    pub fn get_nonce(&self,who: &T::AccountId)->T::Nonce{
        *self.nonce.get(who).unwrap_or(&T::Nonce::zero())
    }
    pub fn block_number(&self) -> T::BlockNumber {
        self.block_number
    }
}

#[cfg(test)]
mod test{
    struct TestConfig;
    
    impl super::Config for TestConfig {
        type AccountId = String;
        type BlockNumber = u32;
        type Nonce = u32;
    }

    #[test]
    fn init_system(){
        let system:super::Pallet<TestConfig> = super::Pallet::new();
        assert_eq!(system.get_block_number(), 0);
    }

    #[test]
    fn inc_block_number(){
        let mut system = super::Pallet::<TestConfig>::new();
        let alice = String::from("alice");
        system.inc_nonce(&alice);
        system.inc_block_number();
        assert_eq!(system.get_nonce(&alice), 1);
        assert_eq!(system.get_block_number(), 1);
    }
}