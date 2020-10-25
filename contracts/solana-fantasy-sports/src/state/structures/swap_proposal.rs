//! State transition types

use crate::state::*;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use byteorder::{ByteOrder, LittleEndian};
use solana_sdk::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};
use std::cell::RefCell;

#[repr(C)]
pub struct SwapProposal<'a> {
    pub data: &'a RefCell<&'a mut [u8]>,
    pub offset: usize,
}
impl<'a> SwapProposal<'a> {
    pub const LEN: usize = 2 + 2 + 1;
    fn slice<'b>(&self, data: &'b mut [u8]) -> (&'b mut [u8; 2], &'b mut [u8; 2], &'b mut [u8; 1]) {
        mut_array_refs![
            array_mut_ref![data, self.offset, SwapProposal::LEN],
            2,
            2,
            1
        ]
    }

    pub fn get_give_player_id(&self) -> u16 {
        LittleEndian::read_u16(self.slice(&mut self.data.borrow_mut()).0)
    }
    pub fn set_give_player_id(&self, value: u16) {
        LittleEndian::write_u16(self.slice(&mut self.data.borrow_mut()).0, value);
    }

    pub fn get_want_player_id(&self) -> u16 {
        LittleEndian::read_u16(self.slice(&mut self.data.borrow_mut()).1)
    }
    pub fn set_want_player_id(&self, value: u16) {
        LittleEndian::write_u16(self.slice(&mut self.data.borrow_mut()).1, value);
    }

    pub fn get_is_initialized(&self) -> bool {
        unpack_is_initialized(self.slice(&mut self.data.borrow_mut()).2).unwrap()
    }
    pub fn set_is_initialized(&self, value: bool) {
        self.slice(&mut self.data.borrow_mut()).2[0] = value as u8;
    }

    pub fn copy_to(&self, to: &Self) {
        let mut dst = to.data.borrow_mut();
        let mut src = self.data.borrow_mut();
        array_mut_ref![dst, self.offset, SwapProposal::LEN].copy_from_slice(array_mut_ref![
            src,
            self.offset,
            SwapProposal::LEN
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
    //     let check = SwapProposal {
    //         give_player_id: 4,
    //         want_player_id: 4,
    //         is_initialized: true,
    //     };
    //     let mut packed = vec![0; SwapProposal::get_packed_len() + 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         SwapProposal::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; SwapProposal::get_packed_len() - 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         SwapProposal::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; SwapProposal::get_packed_len()];
    //     SwapProposal::pack(check.clone(), &mut packed).unwrap();
    //     let mut expect = vec![35u8; 1];
    //     expect.extend_from_slice(&[1u8]);
    //     assert_eq!(packed, expect);
    //     let unpacked = SwapProposal::unpack_unchecked(&packed).unwrap();
    //     assert_eq!(unpacked, check);
    // }
}
