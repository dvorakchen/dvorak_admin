use crate::components::icons::*;
use crate::models::consts::ADMIN_ROUTE_PREFIX;
use crate::models::{Menu as MenuModel, MenuList};
use leptos::*;
use leptos_router::*;

#[component]
pub fn Home() -> impl IntoView {
    view! {
        <main class="flex relative min-h-screen bg-base-200 bg-admin">
            <Header/>
            <MenuList/>
            <div class="h-screen overflow-y-scroll flex-grow pt-16">
                <Outlet/>
            </div>
        </main>
    }
}

#[server]
async fn logout() -> Result<(), ServerFnError<String>> {
    use crate::server::LOGIN_COOKIE_NAME;
    use actix_web::{
        cookie::{Cookie, SameSite},
        http::header,
        http::header::HeaderValue,
    };
    use leptos_actix::{redirect, ResponseOptions};

    let response = expect_context::<ResponseOptions>();

    //  clean login cookie
    let cookie = Cookie::build(LOGIN_COOKIE_NAME, "")
        .secure(true)
        .http_only(true)
        .same_site(SameSite::Strict)
        .path("/")
        .finish();

    if let Ok(cookie) = HeaderValue::from_str(&cookie.to_string()) {
        response.insert_header(header::SET_COOKIE, cookie);
    } else {
        return Err(ServerFnError::from(
            "server cannot clear cookies".to_string(),
        ));
    }

    redirect("/login");

    return Ok(());
}

#[component]
fn Header() -> impl IntoView {
    let handle_logout = move |_| {
        spawn_local(async move {
            logout().await.expect("logout failed");
        });
    };

    view! {
        <header class="flex justify-between fixed z-10 top-0 left-0 right-0 h-16 py-2 px-4 shadow-sm backdrop-blur">
            <A href="/admin">
                <img class="h-full" src="/images/rustsoft.png" alt="LOGO"/>
            </A>

            <div class="mr-4">
                <div class="dropdown dropdown-end">
                    <div class="indicator">
                        <span class="indicator-item badge badge-primary mt-1">99+</span>
                        <div class="avatar" tabindex="0" role="button">
                            <div class="w-12 rounded-full ring ring-base-100 ring-offset-base-100 ring-offset-2">
                                <img class="h-full" src="/images/avatar.png" alt="Avatar"/>
                            </div>
                        </div>
                    </div>

                    <ul
                        tabindex="0"
                        class="dropdown-content z-[1] menu p-2 mt-2 shadow bg-base-100 rounded-box w-52"
                    >
                        <li>
                            <div class="flex">
                                <Person/>
                                <a href="javascript:;">"Dvorak Profile"</a>
                            </div>

                        </li>
                        <li>
                            <div class="flex">
                                <Bell/>
                                <a href="javascript:;">"Messages"</a>
                            </div>

                        </li>
                        <li>
                            <div class="flex">
                                <BoxArroweft/>
                                <a href="javascript:;" on:click=handle_logout>
                                    "Logout"
                                </a>
                            </div>
                        </li>
                    </ul>
                </div>
            </div>
        </header>
    }
}

#[server]
async fn get_menu() -> Result<MenuList, ServerFnError> {
    use crate::server::get_menu_list;
    let menu_list = get_menu_list().await;
    Ok(menu_list)
}

#[component]
fn MenuList() -> impl IntoView {
    let list = create_resource(|| {}, |_| async move { get_menu().await.unwrap() });

    view! {
        <nav class="h-screen overflow-y-scroll pt-16 w-72 border border-r-gray-500">
            <div class="p-2">
                <ul>
                    <Suspense fallback=move || {
                        view! { <p>"Loading..."</p> }
                    }>
                        {move || {
                            match list.get() {
                                Some(menu_list) => {
                                    menu_list
                                        .into_iter()
                                        .map(|menu| {
                                            view! {
                                                <li key=menu.id>
                                                    <Menu menu=menu/>
                                                </li>
                                            }
                                        })
                                        .collect_view()
                                }
                                None => "".into_view(),
                            }
                        }}

                    </Suspense>
                </ul>
            </div>
        </nav>
    }
}

#[component]
fn Menu(menu: MenuModel) -> impl IntoView {
    view! {
        <div class="collapse hover:bg-base-300 rounded">
            <input type="checkbox" class="peer min-h-4"/>
            <div class="collapse-title p-2 min-h-4 text-base-content">
                <div class="flex gap-2 items-center">
                    {Icons::from(menu.icon)} <span class="grow">{menu.title}</span> <Down/>
                </div>
            </div>
            <div class="collapse-content px-0 text-base-content space-y-1">

                {menu
                    .sub_menu
                    .into_iter()
                    .map(|sub| {
                        let link = if sub.link.starts_with(ADMIN_ROUTE_PREFIX) {
                            sub.link
                        } else {
                            format!("{}/{}", ADMIN_ROUTE_PREFIX, sub.link)
                        }
                            .clone();
                        view! {
                            <div class="px-2" key=menu.id>
                                <A
                                    href=link
                                    class="py-2 pl-8 block rounded-lg
                                    hover:bg-base-200"
                                >
                                    {sub.title}
                                </A>
                            </div>
                        }
                    })
                    .collect_view()}

            </div>
        </div>
    }
}
