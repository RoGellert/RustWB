use crate::app_data::AppData;
use crate::config::AuthConfig;
use crate::data_model::{UserPayload, UserPayloadHashed, Validate};
use crate::errors::ServerError;
use crate::AppState;
use axum::body::Body;
use axum::extract::{Request, State};
use axum::http::header::AUTHORIZATION;
use axum::middleware::Next;
use axum::response::Response;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::sync::Arc;
use tracing::info;

// структура с логином и хэшем пароля
impl UserPayloadHashed {
    // хэширование пароля и возврат структуры
    pub fn from_user_payload(user_payload: UserPayload) -> Result<Self, Box<dyn Error>> {
        let password_hash = hash(&user_payload.password, DEFAULT_COST)?;

        Ok(UserPayloadHashed {
            login: user_payload.login,
            password_hash,
        })
    }
}

// структура для генерации jwt
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    login: String,
    exp: i64,
}

// модуль аутентификации
pub struct AuthModule {
    auth_config: AuthConfig,
    app_data: Arc<AppData>,
}

impl AuthModule {
    // инициализация модуля авторизации
    pub fn new(app_data: Arc<AppData>, auth_config: AuthConfig) -> Self {
        AuthModule {
            app_data,
            auth_config,
        }
    }

    // генерация jwt
    fn encode_jwt(&self, login: String) -> Result<String, jsonwebtoken::errors::Error> {
        let delta: chrono::TimeDelta = Duration::seconds(self.auth_config.jwt_expiry_time);
        let exp = (Utc::now() + delta).timestamp();
        let claim = Claims { login, exp };

        encode(
            &Header::default(),
            &claim,
            &EncodingKey::from_secret(self.auth_config.server_encoding_key.as_ref()),
        )
    }

    // расшифровка jwt
    fn decode_jwt(&self, token: String) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
        decode(
            &token,
            &DecodingKey::from_secret(self.auth_config.server_encoding_key.as_ref()),
            &Validation::default(),
        )
    }

    // регистрация пользователя
    pub async fn register_user(&self, user_payload: UserPayload) -> Result<(), ServerError> {
        // валидация
        if let Err(text) = user_payload.is_valid() {
            return Err(ServerError::Validation(text));
        }

        // проверка на наличие пользователя с таким логином в базе
        if self
            .app_data
            .get_user_by_login(&user_payload.login)
            .is_some()
        {
            return Err(ServerError::BusinessLogic(format!(
                "пользователь с логином {} уже существует в базе данных",
                user_payload.login
            )));
        }

        // хэширование пароля и возврат стуртуры с хэшем пароля и логином
        let user_payload_hashed = match UserPayloadHashed::from_user_payload(user_payload) {
            Ok(user_payload_hashed) => user_payload_hashed,
            Err(_) => {
                return Err(ServerError::PasswordHashGeneration(
                    "ошибка генерации шифрования для пароля".to_string(),
                ))
            }
        };

        let login_ref = user_payload_hashed.login.clone();

        // добавление пользователя в базу
        self.app_data.insert_new_user(user_payload_hashed);

        info!("пользователь {} зарегистрировался", login_ref);

        Ok(())
    }

    // log-in пользователя
    pub async fn login_user(&self, user_payload: UserPayload) -> Result<String, ServerError> {
        // валидация
        if let Err(text) = user_payload.is_valid() {
            return Err(ServerError::Validation(text));
        }

        let login = user_payload.login;
        let password = user_payload.password;

        // проверка на наличие пользователя в базе
        let user = match self.app_data.get_user_by_login(&login) {
            None => {
                return Err(ServerError::NotFound(format!(
                    "пользователь с логином {} отсутсвует в базе данных",
                    login
                )))
            }
            Some(user) => user,
        };

        let password_hash = user.password_hash;
        let login = user.login;

        // верификация пароля
        match verify(password, password_hash.as_str()) {
            Ok(true) => {}
            Ok(false) => {
                return Err(ServerError::Unauthorised(format!(
                    "пользователь с логином {} ввёл неверный пароль",
                    login
                )))
            }
            Err(_) => {
                return Err(ServerError::PasswordHashGeneration(
                    "ошибка генерации шифрования для пароля".to_string(),
                ))
            }
        }

        // генерация jwt
        let token = match self.encode_jwt(login.clone()) {
            Ok(token) => token,
            Err(_) => {
                return Err(ServerError::Jwt(format!(
                    "ошибка генерации jwt для пользователя {}",
                    login
                )))
            }
        };

        info!("пользователь {} получил новый JWT", login);

        Ok(token)
    }
}

// миддлвара для авторизации запроса
pub async fn jwt_protected(
    State(app_state): State<Arc<AppState>>,
    mut request: Request,
    next: Next,
) -> Result<Response<Body>, ServerError> {
    // получение заголовка авторизации
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

    // получение токена из заголовка
    let mut header = auth_header.split_whitespace();
    let (_, token) = (header.next(), header.next());

    // расшифровка токена
    let token_data = match app_state.auth_module.decode_jwt(token.unwrap().to_string()) {
        Ok(data) => data,
        Err(_) => {
            return Err(ServerError::Unauthorised(
                "неверный JWT в запросе".to_string(),
            ))
        }
    };

    // проверка истёк ли токен
    if Utc::now().timestamp() > token_data.claims.exp {
        return Err(ServerError::Unauthorised(
            "время жизни токена истекло".to_string(),
        ));
    }

    // передача логина из миддлвары в хэндлер
    request.extensions_mut().insert(token_data.claims.login);

    Ok(next.run(request).await)
}
