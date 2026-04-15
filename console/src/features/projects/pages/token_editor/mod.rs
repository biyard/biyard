mod editor_card;
mod token_create;
mod token_edit;

pub use editor_card::*;
pub use token_create::*;
pub use token_edit::*;

use crate::features::tokens::TokenResponse;

#[derive(Clone, PartialEq)]
pub enum TokenEditorMode {
    Create,
    Edit { token: TokenResponse },
}
