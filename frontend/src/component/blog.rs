use common::model::blog::Blog as BlogModel;
use common::model::post::Post;
use reqwasm::http::Request;
use yew::prelude::*;
use yew_oauth2::context::OAuth2Context;

async fn get_blog(id: &String, token: &str) -> BlogModel {
    let url = format!("/api/{}", id);
    Request::get(&url)
        .header("Authorization", token)
        .send()
        .await
        .unwrap()
        .json()
        .await
        .unwrap()
}

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct BlogViewProps {
    pub blog_id: String,
}

fn post_to_html(post: &Post) -> Html {
    let title_html = if let Some(title) = &post.title {
        html! {<h1>{title}</h1>}
    } else {
        html! {}
    };

    html! {
        <>
            {title_html}
            <h5>{post.post_id.clone()}</h5>
            <p>{post.content.clone()}</p>
        </>
    }
}

#[function_component(Blog)]
pub fn blog(props: &BlogViewProps) -> Html {
    let blog_id = props.blog_id.clone();
    let posts = use_state(Vec::new);
    let title = use_state(String::new);
    let subtitle = use_state(String::new);
    let credentials = use_context::<OAuth2Context>();
    {
        let posts = posts.clone();
        use_effect_with_deps(
            move |_| {
                let posts = posts.clone();
                wasm_bindgen_futures::spawn_local(async move {
                    let creds = credentials.unwrap();
                    let token = creds.access_token().unwrap();
                    let blog: BlogModel = get_blog(&blog_id, token).await;
                    posts.set(blog.posts);
                    if let Some(blog_title) = blog.title {
                        title.set(blog_title)
                    }
                    if let Some(blog_subtitle) = blog.subtitle {
                        subtitle.set(blog_subtitle)
                    }
                });
                || ()
            },
            (),
        );
    }

    let posts_view = (*posts).iter().map(post_to_html);

    let current_time: String = chrono::Local::now().to_string();

    if posts_view.len() > 0 {
        html! {
            <>
                {posts_view.collect::<Html>()}
            </>
        }
    } else {
        html! {
            <>
                <h5>{"Render Time "} {current_time}</h5>
                <div>{"Loading..."}</div>
            </>
        }
    }
}
