use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::components::{DashBoard, Home, Login, NotFound404};
use crate::models::consts::ADMIN_ROUTE_PREFIX;

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    view! {
        <Html lang="en" attr:data-theme="light"/>
        <Stylesheet id="leptos" href="/pkg/dvorak_admin.css"/>
        <Link rel="shortcut icon" type_="image/ico" href="/favicon.ico"/>
        <Router>
            <Routes>
                <Route path=ADMIN_ROUTE_PREFIX view=Home>
                    <Route path="" view=DashBoard/>
                    <Route path="*any" view=NotFound404/>
                </Route>
                <Route path="login" view=Login/>
            </Routes>
        </Router>
    }
}
