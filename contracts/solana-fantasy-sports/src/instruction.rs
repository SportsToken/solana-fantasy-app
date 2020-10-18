//! Instruction types

use crate::error::SfsError;
use solana_sdk::{
    instruction::{AccountMeta, Instruction},
    program_error::ProgramError,
    program_option::COption,
    pubkey::Pubkey,
    sysvar,
};
use std::convert::TryInto;
use std::mem::size_of;

/// Instructions supported by the token program.
#[repr(C)]
#[derive(Clone, Debug, PartialEq)]
pub enum SfsInstruction {
    /// Initializes a new root.
    ///
    /// The `InitializeRoot` instruction requires no signers and MUST be included within
    /// the same Transaction as the system program's `CreateInstruction` that creates the account
    /// being initialized.  Otherwise another party can acquire ownership of the uninitialized account.
    ///
    /// Accounts expected by this instruction:
    ///
    ///   0. `[writable]` The root to initialize.
    ///   1. `[]` Rent sysvar
    ///
    InitializeRoot {
        /// The authority/multisignature to supply game scores.
        oracle_authority: COption<Pubkey>,
    },
    /// Test
    TestMutate,
}
impl SfsInstruction {
    /// Unpacks a byte buffer into a [SfsInstruction](enum.SfsInstruction.html).
    pub fn unpack(input: &[u8]) -> Result<Self, ProgramError> {
        use SfsError::InvalidInstruction;

        let (&tag, rest) = input.split_first().ok_or(InvalidInstruction)?;
        Ok(match tag {
            0 => {
                let (oracle_authority, _rest) = Self::unpack_pubkey_option(rest)?;
                Self::InitializeRoot {
                    oracle_authority,
                }
            },
            1 => Self::TestMutate,

            _ => return Err(SfsError::InvalidInstruction.into()),
        })
    }

    /// Packs a [SfsInstruction](enum.SfsInstruction.html) into a byte buffer.
    pub fn pack(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(size_of::<Self>());
        match self {
            &Self::InitializeRoot {
                ref oracle_authority,
            } => {
                buf.push(0);
                Self::pack_pubkey_option(oracle_authority, &mut buf);
            }
            Self::TestMutate => buf.push(1),
        };
        buf
    }

    fn unpack_pubkey(input: &[u8]) -> Result<(Pubkey, &[u8]), ProgramError> {
        if input.len() >= 32 {
            let (key, rest) = input.split_at(32);
            let pk = Pubkey::new(key);
            Ok((pk, rest))
        } else {
            Err(SfsError::InvalidInstruction.into())
        }
    }

    fn unpack_pubkey_option(input: &[u8]) -> Result<(COption<Pubkey>, &[u8]), ProgramError> {
        match input.split_first() {
            Option::Some((&0, rest)) => Ok((COption::None, rest)),
            Option::Some((&1, rest)) if rest.len() >= 32 => {
                let (key, rest) = rest.split_at(32);
                let pk = Pubkey::new(key);
                Ok((COption::Some(pk), rest))
            }
            _ => Err(SfsError::InvalidInstruction.into()),
        }
    }

    fn pack_pubkey_option(value: &COption<Pubkey>, buf: &mut Vec<u8>) {
        match *value {
            COption::Some(ref key) => {
                buf.push(1);
                buf.extend_from_slice(&key.to_bytes());
            }
            COption::None => buf.push(0),
        }
    }
}

/// Creates a `InitializeRoot` instruction.
pub fn initialize_root(
    sfs_program_id: &Pubkey,
    root_pubkey: &Pubkey,
    oracle_authority_pubkey: Option<&Pubkey>,
) -> Result<Instruction, ProgramError> {
    let oracle_authority = oracle_authority_pubkey.cloned().into();
    let data = SfsInstruction::InitializeRoot {
        oracle_authority,
    }
    .pack();

    let accounts = vec![
        AccountMeta::new(*root_pubkey, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Ok(Instruction {
        program_id: *sfs_program_id,
        accounts,
        data,
    })
}

/// Creates a `TestMutate` instruction.
pub fn test_mutate(
    sfs_program_id: &Pubkey,
    account_pubkey: &Pubkey,
    mint_pubkey: &Pubkey,
    owner_pubkey: &Pubkey,
) -> Result<Instruction, ProgramError> {
    let data = SfsInstruction::TestMutate.pack(); // TODO do we need to return result?

    let accounts = vec![
        AccountMeta::new(*account_pubkey, false),
        AccountMeta::new_readonly(*mint_pubkey, false),
        AccountMeta::new_readonly(*owner_pubkey, false),
        AccountMeta::new_readonly(sysvar::rent::id(), false),
    ];

    Ok(Instruction {
        program_id: *sfs_program_id,
        accounts,
        data,
    })
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_instruction_packing() {
        let check = SfsInstruction::InitializeRoot {
            oracle_authority: COption::Some(Pubkey::new(&[3u8; 32])),
        };
        let packed = check.pack();
        let mut expect = vec![0u8, 1];
        expect.extend_from_slice(&[3u8; 32]);
        assert_eq!(packed, expect);
        let unpacked = SfsInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);

        let check = SfsInstruction::TestMutate;
        let packed = check.pack();
        let expect = Vec::from([1u8]);
        assert_eq!(packed, expect);
        let unpacked = SfsInstruction::unpack(&expect).unwrap();
        assert_eq!(unpacked, check);
    }
}