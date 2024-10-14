use crate::config::AuthConfig;
use crate::data_types::{User, UserPayload, UserPayloadHashed};
use crate::errors::ServerError;
use crate::modules::user_module::UserModule;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use bcrypt::{hash, verify, BcryptError, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
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
    exp: i64,
}

#[derive(Clone)]
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
        let exp = (now + expire).timestamp();
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
        match self
            .user_module
            .get_user_by_login(&user_payload.login)
            .await
        {
            Ok(Some(_)) => {
                return Err(ServerError::BusinessLogic(format!(
                    "пользователь с логином {} уже существует в базе данных",
                    user_payload.login
                )))
            }
            Ok(None) => {}
            Err(err) => return Err(ServerError::Postgres(err)),
        }

        let user_payload_hashed = match UserPayloadHashed::from_user_payload(user_payload) {
            Ok(user_payload_hashed) => user_payload_hashed,
            Err(_) => {
                return Err(ServerError::Password(
                    "ошибка шифрования пароля".to_string(),
                ))
            }
        };

        info!(
            "пользователь {} зарегистрировался",
            &user_payload_hashed.login
        );

        self.user_module
            .insert_user(user_payload_hashed)
            .await
            .map_err(ServerError::Postgres)
    }

    pub async fn login_user(&self, user_payload: UserPayload) -> Result<String, ServerError> {
        let login = user_payload.login;
        let password = user_payload.password;

        let password_hash = match self.user_module.get_user_by_login(&login).await {
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

    pub async fn get_user_by_login(&self, login: &str) -> Result<Option<User>, Box<dyn Error>> {
        self.user_module.get_user_by_login(login).await
    }
}

pub async fn jwt_protected(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response<Body>, ServerError> {
    let auth_header = match request.headers_mut().get(AUTHORIZATION) {
        Some(header) => match header.to_str() {
            Err(_) => return Err(ServerError::Unauthorised("JWT в запросе пуст".to_string())),
            Ok(data) => data,
        },
        None => {
            return Err(ServerError::Unauthorised(
                "отсутствует JWT в запросе".to_string(),
            ))
        }
    };

    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    let token_data = match app_state.auth_module.decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(ServerError::Unauthorised(
                "неверный JWT в запросе".to_string(),
            ))
        }
    };

    if Utc::now().timestamp() > token_data.claims.exp {
        return return Err(ServerError::Unauthorised(
            "время жизни токена истекло".to_string(),
        ))
    }

    // проверка есть ли пользователь в базе и возврат пользователя из базы
    let user = match app_state
        .auth_module
        .get_user_by_login(&token_data.claims.login)
        .await
    {
        Ok(Some(user)) => user,
        Ok(None) => {
            return Err(ServerError::Unauthorised(
                "зарегестирированный пользователь отсутсвует в базе данных".to_string(),
            ))
        }
        Err(err) => return Err(ServerError::Postgres(err)),
    };
    request.extensions_mut().insert(user);

    Ok(next.run(request).await)
}
