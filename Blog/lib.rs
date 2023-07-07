use anchor_lang::prelude::*;
pub mod constant;
use crate::constant::*;
declare_id!("nE6yX2gtU6CxVm1zPxptxiJnrsUFNZvdAGckzHsDN9v");

#[program]
pub mod noter {
    use super::*;

    pub fn create_user(ctx: Context<CreateUser>, name: String, avatar: String) -> Result<()> {
        let user = &mut ctx.accounts.user_account;
        let authority = &mut ctx.accounts.authority;
        user.name = name;
        user.avatar = avatar;
        user.last_post_id=0;
        user.authority = authority.key();

        Ok(())
    }

    pub fn create_post(ctx: Context<CreatePost>, content: String) -> Result<()> {
        
        let user_account = &mut ctx.accounts.user_account;
        let post = &mut ctx.accounts.post;
        let authority = &mut ctx.accounts.authority;
        post.authority = authority.key();
        post.user=user_account.key();
        post.id=user_account.last_post_id;
        post.content=content;
        user_account.last_post_id=user_account.last_post_id
            .checked_add(1)
            .unwrap();

        Ok(())
    }

}

#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init,seeds=[USER_SEED,authority.key().as_ref()],
          bump,
          payer=authority,
          space=3065+8
             )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct CreatePost<'info> {
    #[account(init,seeds=[POST_SEED,authority.key().as_ref(),&[user_account.last_post_id as u8].as_ref()],
          bump,
          payer=authority,
          space=2068+8
             )]
    pub post: Account<'info, Post>,

    #[account(mut,
            seeds=[USER_SEED,authority.key().as_ref()],
            bump,
            has_one=authority
          
             )]
    pub user_account: Account<'info, UserAccount>,

    #[account(mut)]
    pub authority: Signer<'info>,

    pub system_program: Program<'info, System>,
}

#[account]
pub struct UserAccount {
    pub name: String,      //4+25
    pub avatar: String,    //4+3000
    pub last_post_id: u8, //1
    pub authority: Pubkey, //32
}

#[account]
pub struct Post {
    pub id: u8, //1
    pub content: String,      //4+2000
    pub user: Pubkey,    //32
    pub authority: Pubkey, //32
}
