//! State transition types

use crate::state::*;
use arrayref::{array_mut_ref, array_ref, mut_array_refs};
use solana_sdk::{
    program_error::ProgramError,
    program_pack::{Pack, Sealed},
};
use std::cell::RefCell;
use std::ops::{Index, IndexMut};

const ITEM_SIZE: usize = SwapProposal::LEN;
const ITEM_COUNT: usize = consts::SWAP_PROPOSALS_CAPACITY;

#[repr(C)]
pub struct SwapProposalsList<'a> {
    pub data: &'a RefCell<&'a mut [u8]>,
    pub offset: usize,
}
impl<'a> SwapProposalsList<'a> {
    pub const ITEM_SIZE: usize = SwapProposal::LEN;
    pub const ITEM_CAPACITY: usize = consts::SWAP_PROPOSALS_CAPACITY;
    pub const LEN: usize = 1 + SwapProposalsList::ITEM_SIZE * SwapProposalsList::ITEM_CAPACITY;
    fn slice<'b>(
        &self,
        data: &'b mut [u8],
    ) -> (
        &'b mut [u8; 1],
        &'b mut [u8; SwapProposalsList::ITEM_SIZE * SwapProposalsList::ITEM_CAPACITY],
    ) {
        mut_array_refs![
            array_mut_ref![data, self.offset, SwapProposalsList::LEN],
            1,
            SwapProposalsList::ITEM_SIZE * SwapProposalsList::ITEM_CAPACITY
        ]
    }

    pub fn get_count(&self) -> u8 {
        self.slice(&mut self.data.borrow_mut()).0[0]
    }
    fn set_count(&self, value: u8) {
        self.slice(&mut self.data.borrow_mut()).0[0] = value;
    }

    pub fn get(&self, i: u8) -> SwapProposal<'a> {
        if i >= self.get_count() {
            panic!("Attempt to access player out of bound");
        }
        SwapProposal {
            data: self.data,
            offset: self.offset + 1 + i as usize * SwapProposalsList::ITEM_SIZE,
        }
    }

    pub fn add(&self, give_player_id: u16, want_player_id: u16) {
        if usize::from(self.get_count()) >= SwapProposalsList::ITEM_CAPACITY {
            panic!("No more proposal can be added");
        }
        self.set_count(self.get_count() + 1);
        let proposal = self.get(self.get_count() - 1);
        proposal.set_give_player_id(give_player_id);
        proposal.set_want_player_id(want_player_id);
        proposal.set_is_initialized(true);
    }

    pub fn remove(&self, i: u8) {
        if i >= self.get_count() {
            panic!("Index out of bounds");
        }
        for i2 in i..self.get_count() - 1 {
            let proposal = self.get(i2);
            let next = self.get(i2 + 1);
            next.copy_to(&proposal);
        }
        let last = self.get(self.get_count() - 1);
        last.set_give_player_id(0);
        last.set_want_player_id(0);
        last.set_is_initialized(false);
        self.set_count(self.get_count() - 1);
    }

    pub fn copy_to(&self, to: &Self) {
        let mut dst = to.data.borrow_mut();
        let mut src = self.data.borrow_mut();
        array_mut_ref![dst, self.offset, SwapProposalsList::LEN].copy_from_slice(array_mut_ref![
            src,
            self.offset,
            SwapProposalsList::LEN
        ]);
    }
}

// Pull in syscall stubs when building for non-BPF targets
#[cfg(not(target_arch = "bpf"))]
solana_sdk::program_stubs!();

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_pack_unpack() {
    //     let check = SwapProposalsList {
    //         list: vec![SwapProposal::default(); ITEM_COUNT],
    //     };
    //     let mut packed = vec![0; SwapProposalsList::get_packed_len() + 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         SwapProposalsList::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; SwapProposalsList::get_packed_len() - 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         SwapProposalsList::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; SwapProposalsList::get_packed_len()];
    //     SwapProposalsList::pack(check.clone(), &mut packed).unwrap();
    //     let expect = vec![0u8; ITEM_SIZE * ITEM_COUNT];
    //     assert_eq!(packed, expect);
    //     let unpacked = SwapProposalsList::unpack_unchecked(&packed).unwrap();
    //     assert_eq!(unpacked, check);
    // }
}
