use anchor_lang::prelude::*;

mod constants;
mod errors;
mod states;

use crate::{constants::*, errors::*, states::*};

declare_id!("");

#[program]
pub mod satyam_todo {
    use super::*;

    pub fn initialize_user(ctx: Context<InitializeUser>) -> Result<()> {
        // initialize user
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.authority = ctx.accounts.authority.key();
        user_profile.last_todo = 0;
        user_profile.todo_count = 0;

        Ok(())
    }

    pub fn add_todo(ctx: Context<AddTodo>, _content: String) -> Result<()> {
        // initialize todo
        let todo_account = &mut ctx.accounts.todo_account;
        let user_profile = &mut ctx.accounts.user_profile;

        todo_account.authority = ctx.accounts.authority.key();
        todo_account.idx = user_profile.last_todo;
        todo_account.content = _content;
        todo_account.marked = false;

        user_profile.last_todo = user_profile.last_todo.checked_add(1).unwrap();

        user_profile.todo_count = user_profile.todo_count.checked_add(1).unwrap();

        Ok(())
    }

    pub fn update_todo(ctx: Context<UpdateTodo>, todo_idx: u8, content: String) -> Result<()> {
        let todo_account = &mut ctx.accounts.todo_account;

        todo_account.content = content.to_string();

        Ok(())
    }

    pub fn remove_todo(ctx: Context<RemoveTodo>, idx: u8) -> Result<()> {
        let user_profile = &mut ctx.accounts.user_profile;

        user_profile.todo_count = user_profile.todo_count.checked_sub(1).unwrap();

        Ok(())
    }
}

#[derive(Accounts)]
#[instruction()]
pub struct InitializeUser<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        init,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<UserProfile>(),
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction()]
pub struct AddTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority,
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        init,
        seeds = [TODO_TAG, authority.key().as_ref(), &[(user_profile.last_todo as u8)].as_ref()],
        bump,
        payer = authority,
        space = 8 + std::mem::size_of::<TodoAccount>(),
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct UpdateTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(todo_idx: u8)]
pub struct RemoveTodo<'info> {
    #[account(mut)]
    pub authority: Signer<'info>,

    #[account(
        mut,
        seeds = [USER_TAG, authority.key().as_ref()],
        bump,
        has_one = authority
    )]
    pub user_profile: Box<Account<'info, UserProfile>>,

    #[account(
        mut,
        close = authority,
        seeds = [TODO_TAG, authority.key().as_ref(), &[todo_idx].as_ref()],
        bump,
        has_one = authority
    )]
    pub todo_account: Box<Account<'info, TodoAccount>>,

    pub system_program: Program<'info, System>,
}
