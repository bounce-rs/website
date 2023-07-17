use bounce_website_view::View;
use stellation_backend::{Request, ServerAppProps};
use stellation_stylist::BackendManagerProvider;
use yew::prelude::*;

#[function_component]
pub fn ServerApp<REQ>(_props: &ServerAppProps<(), REQ>) -> Html
where
    REQ: Request,
{
    html! {
        <Suspense fallback={Html::default()}>
            <BackendManagerProvider>
                <View />
            </BackendManagerProvider>
        </Suspense>
    }
}
