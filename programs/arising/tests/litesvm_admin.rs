#[path = "common/mod.rs"]
mod common;

use arising::codex::Civilization;
use arising::errors::ArisingError;
use common::{
    context::{TestContext, TransactionResultExt},
    conversions, fixtures, instructions,
};
use solana_keypair::Keypair;
use solana_signer::Signer;

#[test]
fn update_cap_requires_authority() {
    let mut ctx = TestContext::new();
    let mint_state = fixtures::create_mint_state(&mut ctx);

    let authority = ctx.payer();
    ctx.execute_instruction(
        instructions::update_civilization_cap(
            mint_state,
            conversions::keypair_pubkey(&authority),
            Civilization::Ard,
            10,
        ),
        &[],
    )
    .assert_success();

    let outsider = Keypair::new();
    ctx.svm.airdrop(&outsider.pubkey(), 1_000_000_000).unwrap();
    ctx.execute_instruction(
        instructions::update_civilization_cap(
            mint_state,
            conversions::keypair_pubkey(&outsider),
            Civilization::Ard,
            20,
        ),
        &[&outsider],
    )
    .assert_anchor_error(ArisingError::Unauthorized);
}
