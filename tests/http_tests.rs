//! 真实 Router 的内存 HTTP 集成测试。
//!
//! 不绑定端口、不走系统代理，也不依赖 Redis/MySQL；认证状态保存在隔离 SQLite 中。

use axum::{
    body::Body,
    http::{header, Method, Request, StatusCode},
    Extension, Router,
};
use hyper::body::to_bytes;
use rbatis::RBatis;
use rbdc_sqlite::SqliteDriver;
use serde_json::{json, Value};
use tower::{ServiceBuilder, ServiceExt};
use uuid::Uuid;

use axum_web_app::{auth_session, config, jwt, middleware, routes};

async fn test_app() -> (
    Router,
    config::Config,
    &'static RBatis,
    auth_session::SessionStore,
) {
    std::env::set_var("APP_PROFILE", "sqlite");
    let config = config::Config::from_file().expect("load SQLite config");
    let sqlite_path = std::env::temp_dir().join(format!("axum-router-{}.db", Uuid::new_v4()));
    let rb = RBatis::new();
    rb.init(
        SqliteDriver {},
        &format!("sqlite://{}", sqlite_path.display()),
    )
    .expect("initialize isolated SQLite");
    let rb = Box::leak(Box::new(rb));
    let session_store = auth_session::SessionStore::initialize(&config.session, rb)
        .await
        .expect("initialize auth schema");

    let app = Router::new().merge(routes::setup_routes()).layer(
        ServiceBuilder::new()
            .layer(axum::middleware::from_fn(
                middleware::security_headers::add_security_headers,
            ))
            .layer(Extension(config.clone()))
            .layer(Extension(session_store.clone()))
            .layer(Extension(rb as &'static RBatis)),
    );
    (app, config, rb, session_store)
}

fn json_request(method: Method, uri: &str, body: Value) -> Request<Body> {
    Request::builder()
        .method(method)
        .uri(uri)
        .header(header::CONTENT_TYPE, "application/json")
        .body(Body::from(body.to_string()))
        .expect("build JSON request")
}

async fn body_json(response: axum::response::Response) -> Value {
    let bytes = to_bytes(response.into_body()).await.expect("read body");
    serde_json::from_slice(&bytes).expect("parse JSON body")
}

#[tokio::test]
async fn cookie_auth_security_flow_uses_sqlite() {
    let (app, config, _rb, session_store) = test_app().await;

    let unauthenticated = app
        .clone()
        .oneshot(Request::builder().uri("/get").body(Body::empty()).unwrap())
        .await
        .unwrap();
    assert_eq!(unauthenticated.status(), StatusCode::UNAUTHORIZED);
    assert!(unauthenticated
        .headers()
        .contains_key("content-security-policy"));
    assert_eq!(
        unauthenticated.headers()["x-content-type-options"],
        "nosniff"
    );

    let invalid = app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/api/login",
            json!({"username": "wrong", "password": "wrong"}),
        ))
        .await
        .unwrap();
    assert_eq!(invalid.status(), StatusCode::UNAUTHORIZED);

    let mut hostile_login = json_request(
        Method::POST,
        "/api/login",
        json!({"username": config.auth.username, "password": config.auth.password}),
    );
    hostile_login
        .headers_mut()
        .insert(header::ORIGIN, "https://attacker.example".parse().unwrap());
    let hostile_login = app.clone().oneshot(hostile_login).await.unwrap();
    assert_eq!(hostile_login.status(), StatusCode::FORBIDDEN);

    let login = app
        .clone()
        .oneshot(json_request(
            Method::POST,
            "/api/login",
            json!({"username": config.auth.username, "password": config.auth.password}),
        ))
        .await
        .unwrap();
    assert_eq!(login.status(), StatusCode::OK);
    let raw_cookies = login
        .headers()
        .get_all(header::SET_COOKIE)
        .iter()
        .map(|value| value.to_str().unwrap().to_owned())
        .collect::<Vec<_>>();
    let access_cookie = raw_cookies
        .iter()
        .find(|cookie| cookie.starts_with("access_token="))
        .expect("access cookie");
    assert!(access_cookie.contains("HttpOnly"));
    assert!(access_cookie.contains("SameSite=Lax"));
    let csrf_cookie = raw_cookies
        .iter()
        .find(|cookie| cookie.starts_with("csrf_token="))
        .expect("CSRF cookie");
    assert!(!csrf_cookie.contains("HttpOnly"));
    let cookie_header = raw_cookies
        .iter()
        .map(|cookie| cookie.split(';').next().unwrap())
        .collect::<Vec<_>>()
        .join("; ");
    let csrf_token = csrf_cookie
        .split(';')
        .next()
        .unwrap()
        .strip_prefix("csrf_token=")
        .unwrap()
        .to_owned();
    let login_body = body_json(login).await;
    assert_eq!(login_body["role"], "admin");
    assert!(login_body.get("access_token").is_none());
    assert!(login_body.get("session_id").is_none());

    let missing_csrf = Request::builder()
        .method(Method::POST)
        .uri("/api/logout")
        .header(header::COOKIE, &cookie_header)
        .body(Body::empty())
        .unwrap();
    let missing_csrf = app.clone().oneshot(missing_csrf).await.unwrap();
    assert_eq!(missing_csrf.status(), StatusCode::FORBIDDEN);

    let hostile_origin = Request::builder()
        .method(Method::POST)
        .uri("/api/logout")
        .header(header::COOKIE, &cookie_header)
        .header("x-csrf-token", &csrf_token)
        .header(header::ORIGIN, "https://attacker.example")
        .body(Body::empty())
        .unwrap();
    let hostile_origin = app.clone().oneshot(hostile_origin).await.unwrap();
    assert_eq!(hostile_origin.status(), StatusCode::FORBIDDEN);

    let sanitized = Request::builder()
        .method(Method::POST)
        .uri("/getBody")
        .header(header::CONTENT_TYPE, "application/json")
        .header(header::COOKIE, &cookie_header)
        .header("x-csrf-token", &csrf_token)
        .body(Body::from(
            json!({"html_code": "<h1>safe</h1><script>alert(1)</script>"}).to_string(),
        ))
        .unwrap();
    let sanitized = app.clone().oneshot(sanitized).await.unwrap();
    assert_eq!(sanitized.status(), StatusCode::OK);
    let sanitized_body = to_bytes(sanitized.into_body()).await.unwrap();
    let sanitized_body = String::from_utf8(sanitized_body.to_vec()).unwrap();
    assert!(sanitized_body.contains("safe"));
    assert!(!sanitized_body.contains("script"));

    let (viewer_token, viewer_claims) =
        jwt::issue("readonly", "viewer", &config.jwt).expect("issue viewer token");
    auth_session::create(&session_store, &viewer_claims, "unused-for-bearer")
        .await
        .expect("save viewer session");
    let viewer_read = Request::builder()
        .uri("/get")
        .header(header::AUTHORIZATION, format!("Bearer {viewer_token}"))
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        app.clone().oneshot(viewer_read).await.unwrap().status(),
        StatusCode::OK
    );
    let viewer_write = json_request(Method::POST, "/category/delete", json!("not-found"));
    let (mut parts, body) = viewer_write.into_parts();
    parts.headers.insert(
        header::AUTHORIZATION,
        format!("Bearer {viewer_token}").parse().unwrap(),
    );
    let viewer_write = Request::from_parts(parts, body);
    assert_eq!(
        app.clone().oneshot(viewer_write).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );

    let (unknown_token, unknown_claims) =
        jwt::issue("unknown", "superuser", &config.jwt).expect("issue unknown-role token");
    auth_session::create(&session_store, &unknown_claims, "unused-for-bearer")
        .await
        .expect("save unknown-role session");
    let unknown_role = Request::builder()
        .uri("/get")
        .header(header::AUTHORIZATION, format!("Bearer {unknown_token}"))
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        app.clone().oneshot(unknown_role).await.unwrap().status(),
        StatusCode::FORBIDDEN
    );

    let logout = Request::builder()
        .method(Method::POST)
        .uri("/api/logout")
        .header(header::COOKIE, &cookie_header)
        .header("x-csrf-token", &csrf_token)
        .header(header::ORIGIN, "http://localhost:5173")
        .body(Body::empty())
        .unwrap();
    let logout = app.clone().oneshot(logout).await.unwrap();
    assert_eq!(logout.status(), StatusCode::OK);
    assert_eq!(
        logout.headers().get_all(header::SET_COOKIE).iter().count(),
        2
    );

    let replay = Request::builder()
        .uri("/get")
        .header(header::COOKIE, &cookie_header)
        .body(Body::empty())
        .unwrap();
    assert_eq!(
        app.oneshot(replay).await.unwrap().status(),
        StatusCode::UNAUTHORIZED,
        "a revoked jti must fail even while the signed JWT has not expired"
    );
}
