use component::blog::Blog;
use yew::prelude::*;
use yew::Callback;
use yew_oauth2::oauth2::*; // use `openid::*` when using OpenID connect
use yew_oauth2::prelude::*;
use yew_router::prelude::*;
pub mod component;

#[derive(Routable, Clone, PartialEq)]
enum AppRoute {
    #[at("/:blog_id")]
    Blog { blog_id: String },
}

fn switch(route: &AppRoute) -> Html {
    match route {
        AppRoute::Blog { blog_id } => html! {
            <Blog blog_id={blog_id.to_owned()} />
        },
    }
}

#[function_component(App)]
pub fn app() -> Html {
    let login = Callback::from(|_: MouseEvent| {
        OAuth2Dispatcher::<Client>::new().start_login();
    });
    let logout = Callback::from(|_: MouseEvent| {
        OAuth2Dispatcher::<Client>::new().logout();
    });

    let config = Config {
        client_id: "604tk757p8f5b61m4n7od2fj48".into(),
        auth_url: "https://konaa.auth.us-west-2.amazoncognito.com/login".into(),
        token_url: "https://localhost/api/token".into(),
    };

    html! {
        <OAuth2 {config}>
            <Failure><FailureMessage/></Failure>
            <Authenticated>
                <p> <button onclick={logout}>{ "Logout" }</button> </p>
                <h1>{"Authenticated!"}</h1>
                <BrowserRouter>
                    <Switch<AppRoute> render={Switch::render(switch)}/>
                </BrowserRouter>
            </Authenticated>
            <NotAuthenticated>
                <p>
                    { "You need to log in" }
                    <button onclick={login.clone()}>{ "Login" }</button>
                </p>
            </NotAuthenticated>
        </OAuth2>
    }
}
