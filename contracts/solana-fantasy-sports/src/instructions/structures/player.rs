//! State transition types
use crate::instructions::*;
use crate::state::structures::Position;
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use byteorder::{ByteOrder, LittleEndian};
use num_enum::TryFromPrimitive;
use solana_sdk::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
};
use std::cell::RefCell;

#[repr(C)]
pub struct Player<'a> {
    pub data: &'a RefCell<&'a [u8]>,
    pub offset: usize,
}
impl<'a> Player<'a> {
    pub const LEN: usize = 2 + 1;
    fn slice<'b>(
        &self,
        data: &'b [u8],
    ) -> (
        &'b [u8; 2],
        &'b [u8; 1]
    ) {
        array_refs![
            array_ref![data, self.offset, Player::LEN],
            2,
            1
        ]
    }

    pub fn get_id(&self) -> u16 {
        LittleEndian::read_u16(self.slice(&mut self.data.borrow()).0)
    }

    pub fn get_position(&self) -> Position {
        Position::try_from_primitive(self.slice(&mut self.data.borrow()).1[0])
            .or(Err(ProgramError::InvalidAccountData))
            .unwrap()
    }

    pub fn copy_to(&self, to: &Self) {
        let mut dst = to.data.borrow_mut();
        let mut src = self.data.borrow();
        array_mut_ref![dst, self.offset, Player::LEN].copy_from_slice(array_ref![
            src,
            self.offset,
            Player::LEN
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
    //     let check = Player {
    //         id: 12,
    //         position: Position::LB,
    //         scores: ScoreList::default(),
    //         is_initialized: true,
    //     };
    //     let mut packed = vec![0; Player::get_packed_len() + 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         Player::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; Player::get_packed_len() - 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         Player::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; Player::get_packed_len()];
    //     Player::pack(check.clone(), &mut packed).unwrap();
    //     let mut expect = vec![12u8, 0u8];
    //     expect.extend_from_slice(&[Position::LB as u8; 1]);
    //     expect.extend_from_slice(&[0u8; ScoreList::LEN]);
    //     expect.extend_from_slice(&[1u8]);
    //     assert_eq!(packed, expect);
    //     let unpacked = Player::unpack_unchecked(&packed).unwrap();
    //     assert_eq!(unpacked, check);

    //     let size = Player::get_packed_len();
    //     assert!(size < 100, "too large size, {} bytes", size);
    //     let size = std::mem::size_of::<Player>();
    //     assert!(size < 100, "too large size, {} bytes", size);
    // }
}
