use bounce_website_view::View;
use stellation_stylist::FrontendManagerProvider;
use yew::prelude::*;

#[function_component]
pub fn App() -> Html {
    html! {
        <Suspense fallback={Html::default()}>
            <FrontendManagerProvider>
                <View />
            </FrontendManagerProvider>
        </Suspense>
    }
}
