use uchat_domain::ids::UserId;

pub const ACCOUNT_REGISTER: &str = "/account/register";
pub const ACCOUNT_LOGIN: &str = "/account/login";
pub const HOME: &str = "/home";
pub const HOME_BOOKMARKED: &str = "/home/bookmarked";
pub const HOME_LIKED: &str = "/home/liked";
pub const POST_NEW_CHAT: &str = "/post/new_chat";
pub const POST_NEW_IMAGE: &str = "/post/new_image";
pub const POST_NEW_POLL: &str = "/post/new_poll";
pub const POSTS_TRENDING: &str = "/posts/trending";
pub const PROFILE_EDIT: &str = "/profile/edit";
pub const PROFILE_VIEW: &str = "/profile/view/:user";

pub fn profile_view(user_id: UserId) -> String {
    PROFILE_VIEW.replace(":user", &user_id.to_string())
}
