use anchor_lang::prelude::*;

declare_id!("47qqbdvufNiv9HoYojiKbcmeQUH4DAMaxwv2ffX1TrN5");

const DISCRIMINATOR: usize = 8;
const MAX_NAME_LENGTH: usize = 20;
const MAX_MESSAGE_LENGTH: usize = 50;

#[program]
pub mod student_into_program {
    use super::*;

    pub fn add_student_intro(ctx: Context<AddStudentIntro>, name: String, message: String) -> Result<()> {
        require!(name.len() <= MAX_NAME_LENGTH, StudentIntroError::NameTooLong);
        require!(message.len() <= MAX_MESSAGE_LENGTH, StudentIntroError::MessageTooLong);

        msg!("Student Intro Account Created");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.name = name;
        student_intro.message = message;

        Ok(())
    }

    pub fn update_student_intro(ctx: Context<UpdateStudentIntro>, name: String, message: String) -> Result<()> {
        require!(message.len() <= MAX_MESSAGE_LENGTH, StudentIntroError::MessageTooLong);

        msg!("Student Intro account space reallocated");
        msg!("Name: {}", name);
        msg!("Message: {}", message);

        let student_intro = &mut ctx.accounts.student_intro;
        student_intro.message = message;

        Ok(())
    }

    pub fn delete_student_intro(ctx: Context<DeleteStudentIntro>, name: String) -> Result<()> {
        msg!("Student Intro for {} student, deleted", name);
        Ok(())
    }
}

#[derive(Accounts)]
#[instruction(name:String, description:String)]
pub struct AddStudentIntro<'info> {
    #[account(init, seeds=[name.as_bytes(), initializer.key().as_ref()], bump, payer=initializer, space=DISCRIMINATOR + StudentIntoState::INIT_SPACE
    )]
    pub student_intro: Account<'info, StudentIntoState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String, description:String)]
pub struct UpdateStudentIntro<'info> {
    #[account(mut, seeds=[name.as_bytes(),
        initializer.key().as_ref()],
        bump,
        realloc=DISCRIMINATOR + StudentIntoState::INIT_SPACE,
        realloc::payer = initializer,
        realloc::zero = true,
    )]
    pub student_intro: Account<'info, StudentIntoState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(name:String)]
pub struct DeleteStudentIntro<'info> {
    #[account(mut, seeds=[name.as_bytes(),
        initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub student_intro: Account<'info, StudentIntoState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[account]
#[derive(InitSpace)]
pub struct StudentIntoState {
    pub student: Pubkey,
    #[max_len(20)]
    pub name: String,
    #[max_len(50)]
    pub message: String,
}


#[error_code]
enum StudentIntroError {
    #[msg("Student Name too long")]
    NameTooLong,
    #[msg("Student Message too long")]
    MessageTooLong,
}