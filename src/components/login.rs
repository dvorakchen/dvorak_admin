use leptos::*;
use leptos_router::*;

#[server(UserLogin, "/api")]
pub async fn user_login(username: String, password: String) -> Result<(), ServerFnError<String>> {
    use crate::models::{User, UserError};
    use crate::server::{AppDataCipher, LOGIN_COOKIE_NAME};
    use actix_web::{
        cookie::{
            time::Duration,
            Cookie, SameSite,
        },
        http::header,
        http::header::HeaderValue,
    };
    use base64::prelude::*;
    use leptos_actix::{extract, redirect, ResponseOptions};

    let cipher: AppDataCipher = extract()
        .await
        .map_err(|_| ServerFnError::from("extract fail").to_string())?;
    let mut cipher = cipher.lock().unwrap();

    let user = User::login(username.to_owned(), password.to_owned()).map_err(|e| match e {
        UserError::NotExist => ServerFnError::from("user not exist".to_string()),
    })?;
    let jsoned_user = user.to_json();

    if let Ok(encrypted_user) = cipher.encrypt(&jsoned_user.as_bytes().to_vec()) {
        let encrypted_user = BASE64_STANDARD.encode(encrypted_user);
        let response = expect_context::<ResponseOptions>();

        let cookie = Cookie::build(LOGIN_COOKIE_NAME, encrypted_user)
            .max_age(Duration::WEEK)
            .secure(true)
            .http_only(true)
            .same_site(SameSite::Strict)
            .path("/")
            .finish();
        if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
            response.insert_header(header::SET_COOKIE, cookie);
        }

        redirect("/admin");

        return Ok(());
    }

    return Err(ServerFnError::from("user not exist".to_string()));
}

#[component]
pub fn Login() -> impl IntoView {
    let login = create_server_action::<UserLogin>();

    view! {
        <main class="flex w-full h-screen">
            <div class="w-1/2 bg-gears bg-cover border-r relative">
                <a
                    href="https://unsplash.com/photos/assorted-gears-macro-photography-5Taa8b9E55k"
                    target="_blank"
                    class="absolute right-2 bottom-2 p-1 backdrop-blur text-gray-400"
                >
                    "image from Unsplash"
                </a>
                <div class=""></div>
            </div>
            <div class="w-1/2 min-w-80 h-full flex place-items-center">
                <div class="card">
                    <div class="flex place-content-center">
                        <img src="/images/rustsoft.png" alt="Rust Soft"/>
                    </div>
                    <div class="card-body">
                        <ActionForm action=login>
                            <div class="form-control">
                                <label class="label" for="username">
                                    <span class="label-text">Username</span>
                                </label>
                                <input
                                    type="text"
                                    class="input input-bordered"
                                    required
                                    id="username"
                                    name="username"
                                />
                            </div>
                            <div class="form-control">
                                <label class="label" for="password">
                                    <span class="label-text">Password</span>
                                </label>
                                <input
                                    type="password"
                                    class="input input-bordered"
                                    required
                                    id="password"
                                    name="password"
                                />
                            </div>
                            <div class="form-control mt-6">
                                <button class="btn btn-primary">"Login"</button>
                            </div>
                        </ActionForm>

                        <div class="card-actions justify-end">
                            <button class="btn btn-link" onclick="forgot_modal.showModal()">
                                "Forgot Password"
                            </button>
                            <dialog id="forgot_modal" class="modal">
                                <div class="modal-box">
                                    <form method="dialog">
                                        <button class="btn btn-sm btn-circle btn-ghost absolute right-2 top-2">
                                            "✕"
                                        </button>
                                    </form>
                                    <h3 class="font-bold text-lg">"Forgot password"</h3>
                                    <p class="py-4">"Please contact the Administrator"</p>
                                </div>
                            </dialog>
                        </div>
                    </div>

                </div>

            </div>
        </main>
    }
}
