use anchor_lang::prelude::*;

declare_id!("62hFzjxKcWwR3i1F76czzxsQP5UHSGE5HbKNhcSwbPSu");

const MIN_RATING: u8 = 1;
const MAX_RATING: u8 = 5;
const MAX_TITLE_LENGTH: usize = 20;
const MAX_DESCRIPTION_LENGTH: usize = 50;

#[program]
pub mod anchor_movie_review_program {
    use super::*;

    pub fn add_movie_review(ctx: Context<AddMovieReview>, title: String, description: String, rating: u8) -> Result<()> {
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);

        // We require that the title is not longer that 20 characters
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);

        // We require that the description is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);

        msg!("Movie Review Account Created");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.reviewer = ctx.accounts.initializer.key();
        movie_review.title = title;
        movie_review.rating = rating;
        movie_review.description = description;
        Ok(())
    }

    pub fn update_movie_review(ctx: Context<UpdateMovieReview>, title: String, description: String, rating: u8) -> Result<()> {
        // We require that the rating is between 1 and 5
        require!(rating >= MIN_RATING && rating <= MAX_RATING, MovieReviewError::InvalidRating);

        // We require that the title is not longer than 20 characters
        require!(title.len() <= MAX_TITLE_LENGTH, MovieReviewError::TitleTooLong);

        // We require that the description is not longer than 50 characters
        require!(description.len() <= MAX_DESCRIPTION_LENGTH, MovieReviewError::DescriptionTooLong);

        msg!("Movie review account space reallocated");
        msg!("Title: {}", title);
        msg!("Description: {}", description);
        msg!("Rating: {}", rating);

        let movie_review = &mut ctx.accounts.movie_review;
        movie_review.rating = rating;
        movie_review.description = description;

        Ok(())
    }

    pub fn delete_movie_review(_ctx: Context<DeleteMovieReview>, title: String) -> Result<()> {
        msg!("Movie review for {} deleted", title);
        Ok(())
    }

}



#[account]
#[derive(InitSpace)]
pub struct MovieAccountState {
    pub reviewer: Pubkey,
    pub rating: u8,
    #[max_len(20)]
    pub title: String,
    #[max_len(50)]
    pub description: String
}

const DISCRIMINATOR: usize = 8;


#[error_code]
enum MovieReviewError {
    #[msg("Rating must be between 1 and 5")]
    InvalidRating,
    #[msg("Movie Title too long")]
    TitleTooLong,
    #[msg("Movie Description too long")]
    DescriptionTooLong
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct AddMovieReview<'info> {
    #[account(
        init,
        seeds = [title.as_bytes(), initializer.key().as_ref()],
        bump,
        payer = initializer,
        space = DISCRIMINATOR + MovieAccountState::INIT_SPACE
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[derive(Accounts)]
#[instruction(title:String)]
pub struct UpdateMovieReview<'info> {
    #[account(
        mut,
        seeds = [title.as_bytes(), initializer.key().as_ref()], 
        bump, 
        realloc = DISCRIMINATOR + MovieAccountState::INIT_SPACE, 
        realloc::payer = initializer, 
        realloc::zero = true
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System> 
}

#[derive(Accounts)]
#[instruction(title: String)]
pub struct DeleteMovieReview<'info> {
    #[account(
        mut,
        seeds=[title.as_bytes(), initializer.key().as_ref()],
        bump,
        close=initializer
    )]
    pub movie_review: Account<'info, MovieAccountState>,
    #[account(mut)]
    pub initializer: Signer<'info>,
    pub system_program: Program<'info, System>
}