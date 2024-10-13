use crate::config::AuthConfig;
use crate::data_types::{UserPayload, UserPayloadHashed};
use crate::errors::ServerError;
use crate::modules::user_module::UserModule;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use tracing::info;

impl UserPayloadHashed {
    pub fn from_user_payload(user_payload: UserPayload) -> Result<Self, BcryptError> {
        let password_hash = hash(&user_payload.password, DEFAULT_COST)?;

        Ok(UserPayloadHashed {
            login: user_payload.login,
            password_hash,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    login: String,
    exp: usize,
}

pub struct AuthModule {
    auth_config: AuthConfig,
    user_module: UserModule,
}

impl AuthModule {
    // инициализация модуля авторизации
    pub fn new(auth_config: AuthConfig, user_module: UserModule) -> Self {
        AuthModule {
            auth_config,
            user_module,
        }
    }

    fn encode_jwt(&self, login: String) -> Result<String, jsonwebtoken::errors::Error> {
        let now = Utc::now();
        let expire: chrono::TimeDelta = Duration::seconds(self.auth_config.jwt_expiry_time);
        let exp: usize = (now + expire).timestamp() as usize;
        let claim = Claims { login, exp };

        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.auth_config.server_encoding_key.as_ref()),
        )
    }

    fn decode_jwt(&self, token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode(
            &token,
            &DecodingKey::from_secret(self.auth_config.server_encoding_key.as_ref()),
            &Validation::default(),
        )
    }

    pub async fn register_user(&self, user_payload: UserPayload) -> Result<(), ServerError> {
        match self.user_module.get_user_by_login(&user_payload.login).await {
            Ok(Some(_)) => {
                return Err(ServerError::BusinessLogic(format!(
                    "пользователь с логином {} уже существует в базе данных",
                    user_payload.login
                )))
            }
            Ok(None) => {}
            Err(err) => return Err(ServerError::Postgres(err)),
        }

        let user_payload_hashed_result = UserPayloadHashed::from_user_payload(user_payload);

        let user_payload_hashed = match user_payload_hashed_result {
            Ok(user_payload_hashed) => user_payload_hashed,
            Err(_) => {
                return Err(ServerError::Password(
                    "ошибка шифрования пароля".to_string(),
                ))
            }
        };

        info!("пользователь {} зарегистрировался", &user_payload_hashed.login);

        self.user_module
            .insert_user(user_payload_hashed)
            .await
            .map_err(ServerError::Postgres)
    }

    pub async fn login_user(&self, user_payload: UserPayload) -> Result<String, ServerError> {
        let login = user_payload.login;
        let password = user_payload.password;

        let get_user_result = self.user_module.get_user_by_login(&login).await;

        let password_hash = match get_user_result {
            Ok(None) => {
                return Err(ServerError::NotFound(format!(
                    "пользователь с логином {} отсутсвует в базе данных",
                    login
                )))
            }
            Ok(Some(user)) => user.password_hash,
            Err(err) => return Err(ServerError::Postgres(err)),
        };

        match verify(password, password_hash.as_str()) {
            Ok(true) => {}
            Ok(false) => {
                return Err(ServerError::Password(format!(
                    "пользователь с логином {} ввёл неверный пароль",
                    login
                )))
            }
            Err(_) => {
                return Err(ServerError::Password(
                    "ошибка шифрования пароля".to_string(),
                ))
            }
        }

        let jwt_generation_result = self.encode_jwt(login.clone());

        info!("пользователь {} получил новый JWT", &login);

        match jwt_generation_result {
            Ok(token) => Ok(token),
            Err(_) => Err(ServerError::Jwt(format!(
                "ошибка генерации jwt для пользователя {}",
                login
            ))),
        }
    }
}
