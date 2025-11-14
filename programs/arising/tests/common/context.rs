use crate::common::conversions;
use anchor_lang::error::ERROR_CODE_OFFSET;
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use anchor_lang::{AccountDeserialize, AccountSerialize};
use arising::errors::ArisingError;
use litesvm::types::{FailedTransactionMetadata, TransactionMetadata, TransactionResult};
use litesvm::LiteSVM;
use solana_instruction::Instruction;
use solana_keypair::Keypair;
use solana_message::{Message, VersionedMessage};
use solana_signer::Signer;
use solana_transaction::versioned::VersionedTransaction;
use solana_transaction_error::TransactionError;
use std::io::Cursor;

pub struct TestContext {
    pub svm: LiteSVM,
    payer: Keypair,
}

impl TestContext {
    pub fn new() -> Self {
        let mut svm = LiteSVM::new();
        let program_bytes = load_program_bytes();
        svm.add_program(conversions::to_solana_pubkey(arising::ID), &program_bytes)
            .expect("program deployed");
        let payer = Keypair::new();
        svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
        Self { svm, payer }
    }

    pub fn payer(&self) -> &Keypair {
        &self.payer
    }

    pub fn execute_instruction(
        &mut self,
        instruction: Instruction,
        signers: &[&Keypair],
    ) -> TransactionResult {
        let signer_refs: Vec<&Keypair> = if signers.is_empty() {
            vec![self.payer()]
        } else {
            signers.to_vec()
        };
        let payer = signer_refs[0].pubkey();
        let blockhash = self.svm.latest_blockhash();
        let message = Message::new_with_blockhash(&[instruction], Some(&payer), &blockhash);
        let tx =
            VersionedTransaction::try_new(VersionedMessage::Legacy(message), &signer_refs).unwrap();
        let result = self.svm.send_transaction(tx);
        self.svm.expire_blockhash();
        result
    }

    pub fn store_anchor_account<T: AccountSerialize>(
        &mut self,
        address: AnchorPubkey,
        owner: AnchorPubkey,
        account: &T,
        space: usize,
    ) {
        let lamports = self.svm.minimum_balance_for_rent_exemption(space);
        let mut data = vec![0u8; space];
        {
            let mut cursor = Cursor::new(&mut data[..]);
            account.try_serialize(&mut cursor).unwrap();
        }
        let account = solana_account::Account {
            lamports,
            data,
            owner: conversions::to_solana_pubkey(owner),
            executable: false,
            rent_epoch: 0,
        };
        self.svm
            .set_account(conversions::to_solana_pubkey(address), account)
            .expect("account stored");
    }

    pub fn get_anchor_account<T: AccountDeserialize>(&self, address: &AnchorPubkey) -> T {
        let data = self
            .svm
            .get_account(&conversions::to_solana_pubkey(*address))
            .expect("account not found");
        let mut slice: &[u8] = &data.data;
        T::try_deserialize(&mut slice).expect("account deserialize")
    }
}

fn load_program_bytes() -> Vec<u8> {
    let path = format!(
        "{}/../../target/deploy/arising.so",
        env!("CARGO_MANIFEST_DIR")
    );
    std::fs::read(&path).unwrap_or_else(|err| {
        panic!(
            "failed to read program artifact at {}. Run `anchor build` first? {err}",
            path
        )
    })
}

pub trait TransactionResultExt {
    fn assert_success(self);
    fn assert_success_with(self, label: &str);
    fn assert_anchor_error(self, error: ArisingError);
}

impl TransactionResultExt for TransactionResult {
    fn assert_success(self) {
        self.assert_success_with("transaction");
    }

    fn assert_success_with(self, label: &str) {
        if let Err(err) = self {
            let log_output = err.meta.pretty_logs();
            panic!("{label} failed: {:?}\nlogs:\n{}", err.err, log_output);
        }
    }

    fn assert_anchor_error(self, error: ArisingError) {
        match self {
            Ok(TransactionMetadata { .. }) => {
                panic!("expected error {:?}, but transaction succeeded", error)
            }
            Err(FailedTransactionMetadata { err, .. }) => {
                let TransactionError::InstructionError(_, instruction_err) = err else {
                    panic!("unexpected transaction error: {:?}", err);
                };
                match instruction_err {
                    solana_instruction::error::InstructionError::Custom(code) => {
                        let expected = ERROR_CODE_OFFSET + error as u32;
                        assert_eq!(code, expected);
                    }
                    other => panic!("unexpected instruction error: {:?}", other),
                }
            }
        }
    }
}
