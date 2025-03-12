//! This code was AUTOGENERATED using the codama library.
//! Please DO NOT EDIT THIS FILE, instead use visitors
//! to add features, then rerun codama to update it.
//!
//! <https://github.com/codama-idl/codama>
//!

use crate::generated::types::ED25519RecoverInfo;
use borsh::BorshDeserialize;
use borsh::BorshSerialize;

/// Accounts.
pub struct Lock {
    pub global_account: solana_program::pubkey::Pubkey,

    pub user_account: solana_program::pubkey::Pubkey,

    pub user: solana_program::pubkey::Pubkey,

    pub lock_account: solana_program::pubkey::Pubkey,

    pub vault: solana_program::pubkey::Pubkey,

    pub bot: solana_program::pubkey::Pubkey,

    pub payer: solana_program::pubkey::Pubkey,

    pub system_program: solana_program::pubkey::Pubkey,
}

impl Lock {
    pub fn instruction(
        &self,
        args: LockInstructionArgs,
    ) -> solana_program::instruction::Instruction {
        self.instruction_with_remaining_accounts(args, &[])
    }
    #[allow(clippy::vec_init_then_push)]
    pub fn instruction_with_remaining_accounts(
        &self,
        args: LockInstructionArgs,
        remaining_accounts: &[solana_program::instruction::AccountMeta],
    ) -> solana_program::instruction::Instruction {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.global_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.user, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.lock_account,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.vault, false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.bot, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            self.payer, true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            self.system_program,
            false,
        ));
        accounts.extend_from_slice(remaining_accounts);
        let mut data = LockInstructionData::new().try_to_vec().unwrap();
        let mut args = args.try_to_vec().unwrap();
        data.append(&mut args);

        solana_program::instruction::Instruction {
            program_id: crate::BALANCE_PAYMENT_ID,
            accounts,
            data,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockInstructionData {
    discriminator: [u8; 8],
}

impl LockInstructionData {
    pub fn new() -> Self {
        Self {
            discriminator: [21, 19, 208, 43, 237, 62, 255, 87],
        }
    }
}

impl Default for LockInstructionData {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(BorshSerialize, BorshDeserialize, Clone, Debug, Eq, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LockInstructionArgs {
    pub recover_info: ED25519RecoverInfo,
    pub amount: u64,
}

/// Instruction builder for `Lock`.
///
/// ### Accounts:
///
///   0. `[]` global_account
///   1. `[writable]` user_account
///   2. `[writable]` user
///   3. `[writable]` lock_account
///   4. `[]` vault
///   5. `[signer]` bot
///   6. `[writable, signer]` payer
///   7. `[optional]` system_program (default to `11111111111111111111111111111111`)
#[derive(Clone, Debug, Default)]
pub struct LockBuilder {
    global_account: Option<solana_program::pubkey::Pubkey>,
    user_account: Option<solana_program::pubkey::Pubkey>,
    user: Option<solana_program::pubkey::Pubkey>,
    lock_account: Option<solana_program::pubkey::Pubkey>,
    vault: Option<solana_program::pubkey::Pubkey>,
    bot: Option<solana_program::pubkey::Pubkey>,
    payer: Option<solana_program::pubkey::Pubkey>,
    system_program: Option<solana_program::pubkey::Pubkey>,
    recover_info: Option<ED25519RecoverInfo>,
    amount: Option<u64>,
    __remaining_accounts: Vec<solana_program::instruction::AccountMeta>,
}

impl LockBuilder {
    pub fn new() -> Self {
        Self::default()
    }
    #[inline(always)]
    pub fn global_account(&mut self, global_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.global_account = Some(global_account);
        self
    }
    #[inline(always)]
    pub fn user_account(&mut self, user_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user_account = Some(user_account);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: solana_program::pubkey::Pubkey) -> &mut Self {
        self.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn lock_account(&mut self, lock_account: solana_program::pubkey::Pubkey) -> &mut Self {
        self.lock_account = Some(lock_account);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: solana_program::pubkey::Pubkey) -> &mut Self {
        self.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn bot(&mut self, bot: solana_program::pubkey::Pubkey) -> &mut Self {
        self.bot = Some(bot);
        self
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: solana_program::pubkey::Pubkey) -> &mut Self {
        self.payer = Some(payer);
        self
    }
    /// `[optional account, default to '11111111111111111111111111111111']`
    #[inline(always)]
    pub fn system_program(&mut self, system_program: solana_program::pubkey::Pubkey) -> &mut Self {
        self.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn recover_info(&mut self, recover_info: ED25519RecoverInfo) -> &mut Self {
        self.recover_info = Some(recover_info);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.amount = Some(amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: solana_program::instruction::AccountMeta,
    ) -> &mut Self {
        self.__remaining_accounts.push(account);
        self
    }
    /// Add additional accounts to the instruction.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[solana_program::instruction::AccountMeta],
    ) -> &mut Self {
        self.__remaining_accounts.extend_from_slice(accounts);
        self
    }
    #[allow(clippy::clone_on_copy)]
    pub fn instruction(&self) -> solana_program::instruction::Instruction {
        let accounts = Lock {
            global_account: self.global_account.expect("global_account is not set"),
            user_account: self.user_account.expect("user_account is not set"),
            user: self.user.expect("user is not set"),
            lock_account: self.lock_account.expect("lock_account is not set"),
            vault: self.vault.expect("vault is not set"),
            bot: self.bot.expect("bot is not set"),
            payer: self.payer.expect("payer is not set"),
            system_program: self
                .system_program
                .unwrap_or(solana_program::pubkey!("11111111111111111111111111111111")),
        };
        let args = LockInstructionArgs {
            recover_info: self.recover_info.clone().expect("recover_info is not set"),
            amount: self.amount.clone().expect("amount is not set"),
        };

        accounts.instruction_with_remaining_accounts(args, &self.__remaining_accounts)
    }
}

/// `lock` CPI accounts.
pub struct LockCpiAccounts<'a, 'b> {
    pub global_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub lock_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub bot: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
}

/// `lock` CPI instruction.
pub struct LockCpi<'a, 'b> {
    /// The program to invoke.
    pub __program: &'b solana_program::account_info::AccountInfo<'a>,

    pub global_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub user: &'b solana_program::account_info::AccountInfo<'a>,

    pub lock_account: &'b solana_program::account_info::AccountInfo<'a>,

    pub vault: &'b solana_program::account_info::AccountInfo<'a>,

    pub bot: &'b solana_program::account_info::AccountInfo<'a>,

    pub payer: &'b solana_program::account_info::AccountInfo<'a>,

    pub system_program: &'b solana_program::account_info::AccountInfo<'a>,
    /// The arguments for the instruction.
    pub __args: LockInstructionArgs,
}

impl<'a, 'b> LockCpi<'a, 'b> {
    pub fn new(
        program: &'b solana_program::account_info::AccountInfo<'a>,
        accounts: LockCpiAccounts<'a, 'b>,
        args: LockInstructionArgs,
    ) -> Self {
        Self {
            __program: program,
            global_account: accounts.global_account,
            user_account: accounts.user_account,
            user: accounts.user,
            lock_account: accounts.lock_account,
            vault: accounts.vault,
            bot: accounts.bot,
            payer: accounts.payer,
            system_program: accounts.system_program,
            __args: args,
        }
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], &[])
    }
    #[inline(always)]
    pub fn invoke_with_remaining_accounts(
        &self,
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(&[], remaining_accounts)
    }
    #[inline(always)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed_with_remaining_accounts(signers_seeds, &[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed_with_remaining_accounts(
        &self,
        signers_seeds: &[&[&[u8]]],
        remaining_accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> solana_program::entrypoint::ProgramResult {
        let mut accounts = Vec::with_capacity(8 + remaining_accounts.len());
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.global_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.user.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.lock_account.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.vault.key,
            false,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.bot.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new(
            *self.payer.key,
            true,
        ));
        accounts.push(solana_program::instruction::AccountMeta::new_readonly(
            *self.system_program.key,
            false,
        ));
        remaining_accounts.iter().for_each(|remaining_account| {
            accounts.push(solana_program::instruction::AccountMeta {
                pubkey: *remaining_account.0.key,
                is_signer: remaining_account.1,
                is_writable: remaining_account.2,
            })
        });
        let mut data = LockInstructionData::new().try_to_vec().unwrap();
        let mut args = self.__args.try_to_vec().unwrap();
        data.append(&mut args);

        let instruction = solana_program::instruction::Instruction {
            program_id: crate::BALANCE_PAYMENT_ID,
            accounts,
            data,
        };
        let mut account_infos = Vec::with_capacity(9 + remaining_accounts.len());
        account_infos.push(self.__program.clone());
        account_infos.push(self.global_account.clone());
        account_infos.push(self.user_account.clone());
        account_infos.push(self.user.clone());
        account_infos.push(self.lock_account.clone());
        account_infos.push(self.vault.clone());
        account_infos.push(self.bot.clone());
        account_infos.push(self.payer.clone());
        account_infos.push(self.system_program.clone());
        remaining_accounts
            .iter()
            .for_each(|remaining_account| account_infos.push(remaining_account.0.clone()));

        if signers_seeds.is_empty() {
            solana_program::program::invoke(&instruction, &account_infos)
        } else {
            solana_program::program::invoke_signed(&instruction, &account_infos, signers_seeds)
        }
    }
}

/// Instruction builder for `Lock` via CPI.
///
/// ### Accounts:
///
///   0. `[]` global_account
///   1. `[writable]` user_account
///   2. `[writable]` user
///   3. `[writable]` lock_account
///   4. `[]` vault
///   5. `[signer]` bot
///   6. `[writable, signer]` payer
///   7. `[]` system_program
#[derive(Clone, Debug)]
pub struct LockCpiBuilder<'a, 'b> {
    instruction: Box<LockCpiBuilderInstruction<'a, 'b>>,
}

impl<'a, 'b> LockCpiBuilder<'a, 'b> {
    pub fn new(program: &'b solana_program::account_info::AccountInfo<'a>) -> Self {
        let instruction = Box::new(LockCpiBuilderInstruction {
            __program: program,
            global_account: None,
            user_account: None,
            user: None,
            lock_account: None,
            vault: None,
            bot: None,
            payer: None,
            system_program: None,
            recover_info: None,
            amount: None,
            __remaining_accounts: Vec::new(),
        });
        Self { instruction }
    }
    #[inline(always)]
    pub fn global_account(
        &mut self,
        global_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.global_account = Some(global_account);
        self
    }
    #[inline(always)]
    pub fn user_account(
        &mut self,
        user_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.user_account = Some(user_account);
        self
    }
    #[inline(always)]
    pub fn user(&mut self, user: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.user = Some(user);
        self
    }
    #[inline(always)]
    pub fn lock_account(
        &mut self,
        lock_account: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.lock_account = Some(lock_account);
        self
    }
    #[inline(always)]
    pub fn vault(&mut self, vault: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.vault = Some(vault);
        self
    }
    #[inline(always)]
    pub fn bot(&mut self, bot: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.bot = Some(bot);
        self
    }
    #[inline(always)]
    pub fn payer(&mut self, payer: &'b solana_program::account_info::AccountInfo<'a>) -> &mut Self {
        self.instruction.payer = Some(payer);
        self
    }
    #[inline(always)]
    pub fn system_program(
        &mut self,
        system_program: &'b solana_program::account_info::AccountInfo<'a>,
    ) -> &mut Self {
        self.instruction.system_program = Some(system_program);
        self
    }
    #[inline(always)]
    pub fn recover_info(&mut self, recover_info: ED25519RecoverInfo) -> &mut Self {
        self.instruction.recover_info = Some(recover_info);
        self
    }
    #[inline(always)]
    pub fn amount(&mut self, amount: u64) -> &mut Self {
        self.instruction.amount = Some(amount);
        self
    }
    /// Add an additional account to the instruction.
    #[inline(always)]
    pub fn add_remaining_account(
        &mut self,
        account: &'b solana_program::account_info::AccountInfo<'a>,
        is_writable: bool,
        is_signer: bool,
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .push((account, is_writable, is_signer));
        self
    }
    /// Add additional accounts to the instruction.
    ///
    /// Each account is represented by a tuple of the `AccountInfo`, a `bool` indicating whether the account is writable or not,
    /// and a `bool` indicating whether the account is a signer or not.
    #[inline(always)]
    pub fn add_remaining_accounts(
        &mut self,
        accounts: &[(
            &'b solana_program::account_info::AccountInfo<'a>,
            bool,
            bool,
        )],
    ) -> &mut Self {
        self.instruction
            .__remaining_accounts
            .extend_from_slice(accounts);
        self
    }
    #[inline(always)]
    pub fn invoke(&self) -> solana_program::entrypoint::ProgramResult {
        self.invoke_signed(&[])
    }
    #[allow(clippy::clone_on_copy)]
    #[allow(clippy::vec_init_then_push)]
    pub fn invoke_signed(
        &self,
        signers_seeds: &[&[&[u8]]],
    ) -> solana_program::entrypoint::ProgramResult {
        let args = LockInstructionArgs {
            recover_info: self
                .instruction
                .recover_info
                .clone()
                .expect("recover_info is not set"),
            amount: self.instruction.amount.clone().expect("amount is not set"),
        };
        let instruction = LockCpi {
            __program: self.instruction.__program,

            global_account: self
                .instruction
                .global_account
                .expect("global_account is not set"),

            user_account: self
                .instruction
                .user_account
                .expect("user_account is not set"),

            user: self.instruction.user.expect("user is not set"),

            lock_account: self
                .instruction
                .lock_account
                .expect("lock_account is not set"),

            vault: self.instruction.vault.expect("vault is not set"),

            bot: self.instruction.bot.expect("bot is not set"),

            payer: self.instruction.payer.expect("payer is not set"),

            system_program: self
                .instruction
                .system_program
                .expect("system_program is not set"),
            __args: args,
        };
        instruction.invoke_signed_with_remaining_accounts(
            signers_seeds,
            &self.instruction.__remaining_accounts,
        )
    }
}

#[derive(Clone, Debug)]
struct LockCpiBuilderInstruction<'a, 'b> {
    __program: &'b solana_program::account_info::AccountInfo<'a>,
    global_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    user: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    lock_account: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    vault: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    bot: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    payer: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    system_program: Option<&'b solana_program::account_info::AccountInfo<'a>>,
    recover_info: Option<ED25519RecoverInfo>,
    amount: Option<u64>,
    /// Additional instruction accounts `(AccountInfo, is_writable, is_signer)`.
    __remaining_accounts: Vec<(
        &'b solana_program::account_info::AccountInfo<'a>,
        bool,
        bool,
    )>,
}
