use anchor_lang::prelude::*;

use crate::errors::TwitterError;
use crate::states::*;

pub fn add_comment(ctx: Context<AddCommentContext>, comment_content: String) -> Result<()> {
    let comment = &mut ctx.accounts.comment;

    // -------------------------------------------------------------------------------------------

    require!(
        comment_content.as_bytes().len() <= COMMENT_LENGTH,
        TwitterError::CommentTooLong
    );
    // -------------------------------------------------------------------------------------------

    let mut content_data = [0u8; COMMENT_LENGTH];
    content_data[..comment_content.as_bytes().len()].copy_from_slice(comment_content.as_bytes());
    comment.content = content_data;

    // TODO: Lastly, we want to setup all other Comment variables, check out Comment struct inside states.rs.

    // HINT:
    // - comment_author
    comment.comment_author = ctx.accounts.comment_author.key();
    // - parent_tweet
    comment.parent_tweet = ctx.accounts.tweet.key();
    // - content_length
    // comment.content_length = comment_content.as_bytes().len() as u16;
    // - bump (check initialize_tweet for reference)

    comment.content_length = comment_content.len() as u16;
    comment.bump = ctx.bumps.comment;
    // -------------------------------------------------------------------------------------------

    Ok(())
}
#[derive(Accounts)]
#[instruction(comment_content: String)]
pub struct AddCommentContext<'info> {
    #[account(mut)]
    pub comment_author: Signer<'info>,
    #[account(
        init,
        payer = comment_author,
        space = 8 + Comment::LEN,
        seeds = [
            COMMENT_SEED.as_bytes(),
            comment_author.key().as_ref(),
            {anchor_lang::solana_program::hash::hash(comment_content.as_bytes()).to_bytes().as_ref()},
            tweet.key().as_ref(),
            ],
        bump)]
    pub comment: Account<'info, Comment>,

    // -------------------------------------------------------------------------------------------
    // TODO: Fill the required account macro below - we want to check that a PDA account with correct
    // seeds and bump was submitted.

    // HINT:
    // - account must be mutable
    // - seeds are :    tweet.topic[..tweet.topic_length as usize].as_ref()
    //                  TWEET_SEED.as_bytes(),
    //                  tweet.tweet_author.key().as_ref()
    // - lastly, check the correctness of bump using: bump = tweet.bump
    // -------------------------------------------------------------------------------------------
    #[account(
        mut,
        seeds = [
            tweet.topic[..tweet.topic_length as usize].as_ref(),
            TWEET_SEED.as_bytes(),
            tweet.tweet_author.key().as_ref()
        ],
        bump = tweet.bump
    )]
    pub tweet: Account<'info, Tweet>,

    pub system_program: Program<'info, System>,
}
