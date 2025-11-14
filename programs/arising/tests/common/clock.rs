use crate::common::context::TestContext;
use anchor_lang::solana_program::pubkey::Pubkey as AnchorPubkey;
use arising::character::Character;
use solana_clock::Clock;

pub fn fast_forward_quest(ctx: &mut TestContext, character: &AnchorPubkey) {
    let account: Character = ctx.get_anchor_account(character);
    if let Some(quest) = account.current_quest {
        advance_clock(ctx, quest.ready_at + 1);
    }
}

pub fn fast_forward_recipe(ctx: &mut TestContext, character: &AnchorPubkey) {
    let account: Character = ctx.get_anchor_account(character);
    if let Some(recipe) = account.current_recipe {
        advance_clock(ctx, recipe.ready_at + 1);
    }
}

fn advance_clock(ctx: &mut TestContext, unix_timestamp: i64) {
    let mut clock = ctx.svm.get_sysvar::<Clock>();
    if clock.unix_timestamp < unix_timestamp {
        clock.unix_timestamp = unix_timestamp;
        ctx.svm.set_sysvar(&clock);
    }
}
