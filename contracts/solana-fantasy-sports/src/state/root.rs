//! State transition types
use arrayref::{array_mut_ref, array_ref, array_refs, mut_array_refs};
use solana_sdk::{
    program_error::ProgramError,
    program_pack::{IsInitialized, Pack, Sealed},
    pubkey::Pubkey,
};
use super::{
    lists::{PlayerList, LeagueList},
    consts::PUB_KEY_LEN,
    helpers::*
};
use std::cell::{RefCell};

#[repr(C)]
pub struct Root<'a> {
    pub data: &'a RefCell<&'a mut [u8]>,
    pub offset: usize,
}
impl<'a> Root<'a> {
    pub const LEN: usize = PUB_KEY_LEN
                         + PlayerList::LEN
                         + LeagueList::LEN
                         + 1;
    fn slice<'b>(&self, data: &'b mut [u8]) -> (
        &'b mut [u8;PUB_KEY_LEN],
        &'b mut [u8;PlayerList::LEN],
        &'b mut [u8;LeagueList::LEN],
        &'b mut [u8;1]) {
        mut_array_refs![
            array_mut_ref![data, self.offset, Root::LEN],
            PUB_KEY_LEN,
            PlayerList::LEN,
            LeagueList::LEN,
            1
        ]
    }

    pub fn get_oracle_authority(&self) -> Pubkey {
        Pubkey::new_from_array(*self.slice(&mut self.data.borrow_mut()).0)
    }
    pub fn set_oracle_authority(&self, value: Pubkey) {
        self.slice(&mut self.data.borrow_mut()).0.copy_from_slice(value.as_ref());
    }

    pub fn get_players(&self) -> PlayerList<'a> {
        PlayerList { data: self.data, offset: self.offset + PUB_KEY_LEN }
    }

    pub fn get_leagues(&self) -> LeagueList<'a> {
        LeagueList { data: self.data, offset: self.offset + PUB_KEY_LEN + PlayerList::LEN }
    }

    pub fn get_is_initialized(&self) -> bool {
        unpack_is_initialized(self.slice(&mut self.data.borrow_mut()).3).unwrap()
    }
    pub fn set_is_initialized(&self, value: bool) {
        self.slice(&mut self.data.borrow_mut()).3[0] = value as u8;
    }

    pub fn copy_to(&self, to: &Self) {
        let mut dst = to.data.borrow_mut();
        let mut src = self.data.borrow_mut();
        array_mut_ref![dst, self.offset, Root::LEN]
            .copy_from_slice(array_mut_ref![src, self.offset, Root::LEN]);
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
    //     let check = Root {
    //         oracle_authority: Pubkey::new(&[1; PUB_KEY_LEN]),
    //         players: PlayerList::default(),
    //         leagues: LeagueList::default(),
    //         is_initialized: true,
    //     };
    //     let mut packed = vec![0; Root::get_packed_len() + 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         Root::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; Root::get_packed_len() - 1];
    //     assert_eq!(
    //         Err(ProgramError::InvalidAccountData),
    //         Root::pack(check.clone(), &mut packed)
    //     );
    //     let mut packed = vec![0; Root::get_packed_len()];
    //     Root::pack(check.clone(), &mut packed).unwrap();
    //     let mut expect = vec![0u8; 0];
    //     expect.extend_from_slice(&[1u8; 32]);
    //     expect.extend_from_slice(&[0u8; PlayerList::LEN]);
    //     expect.extend_from_slice(&[0u8; LeagueList::LEN]);
    //     expect.extend_from_slice(&[1u8]);
    //     assert_eq!(packed, expect);
    //     let unpacked = Root::unpack_unchecked(&packed).unwrap();
    //     assert_eq!(unpacked, check);

    //     let size = Root::get_packed_len();
    //     assert!(size < 100, "too large size, {} bytes", size);
    //     let size = std::mem::size_of::<Root>();
    //     assert!(size < 100, "too large size, {} bytes", size);
    // }
}
