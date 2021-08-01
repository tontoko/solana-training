// 参考: https://note.com/cml_2010/n/n079101f19fc8

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    program_error::ProgramError,
    pubkey::Pubkey,
};

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Account {
    /// 「hello」が送信された回数
    pub counter: u32,
}

entrypoint!(process_instruction);

pub fn process_instruction(
    program_id: &Pubkey,
    accounts: &[AccountInfo],
    _instruction_data: &[u8],
) -> ProgramResult {
    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    // if account.owner != program_id {
    //     msg!("Greeted account does not have the correct program id");
    //     return Err(ProgramError::IncorrectProgramId);
    // }

    let mut greeting_account = Account::try_from_slice(&account.data.borrow())?;
    greeting_account.counter += 1;
    greeting_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_program::clock::Epoch;
    use std::mem;

    #[test]
    fn test_sanity() {
        let program_id = Pubkey::default();
        // Pubkey::default()は、アカウントのアドレスの初期値を返す
        let key = Pubkey::default();
        let mut lamports = 0;
        // dataは、プログラムによって任意に保存可能なデータ
        // ここに「hello」が送信された回数を保存します。
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();
        // AccountInfo::newでインスタンスを生成。
        // solana_program::account_infoの公式ドキュメント通り。
        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );

        let instruction_data: Vec<u8> = Vec::new();

        let accounts = vec![account];

        assert_eq!(
            Account::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        assert_eq!(
            Account::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        process_instruction(&program_id, &accounts, &instruction_data).unwrap();

        assert_eq!(
            Account::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );
    }
}
