mod account;
mod assets;
mod scripts;
mod index;
mod login;
mod register;

pub use assets::get_assets as GET_ASSETS;
pub use scripts::get_scripts as GET_SCRIPTS;
pub use index::get_index as GET_INDEX;

pub use register::get_register as GET_REGISTER;
pub use register::post_register as POST_REGISTER;

pub use login::get_login as GET_LOGIN;
pub use login::post_login as POST_LOGIN;

pub use account::get_account as GET_ACCOUNT;
