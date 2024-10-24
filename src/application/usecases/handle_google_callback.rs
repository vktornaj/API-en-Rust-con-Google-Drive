use crate::{
    application::ports::{
        google_drive_service::GoogleDriveServiceTrait,
        user_repository::{self, UserRepositoryTrait},
    },
    domain::{
        entities::{token_data::TokenData, user::User},
        value_objects::email::Email,
    },
};

pub struct Payload {
    pub code: String,
}

pub async fn execute(
    user_repository: &impl UserRepositoryTrait,
    google_drive_service: &impl GoogleDriveServiceTrait,
    secret: &[u8],
    payload: Payload,
) -> Result<String, String> {
    let access_token = match google_drive_service
        .handle_google_callback(payload.code)
        .await
    {
        Ok(access_token) => access_token,
        Err(err) => return Err(err.to_string()),
    };

    let email = match google_drive_service
        .get_google_email(access_token.clone())
        .await
    {
        Ok(email) => email,
        Err(err) => return Err(err.to_string()),
    };

    let user = match user_repository.find_by_email(&email).await {
        Ok(user) => user,
        Err(user_repository::Error::NotFound) => {
            let email = Email::new(email).map_err(|x| x.to_string())?;
            let user = User::new(email, access_token);
            user_repository
                .create(user.clone())
                .await
                .map_err(|x| x.to_string())?;
            user
        }
        Err(err) => return Err(err.to_string()),
    };

    let user = user_repository
        .update(user)
        .await
        .map_err(|x| x.to_string())?;

    Ok(TokenData::new(&user.id).token(secret))
}
