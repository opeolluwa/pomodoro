use crate::{
    components::typography::heading::HeadingText, components::typography::paragraph::BaseText,
    icons::elipses::ElipsesVertical, state::templates::FocusTemplateKind,
};
use leptos::prelude::*;

#[leptos::component]
pub fn FocusCardTemplate(
    #[prop()] kind: FocusTemplateKind,
    #[prop()] title: String,
    #[prop()] description: String,
    // #[prop()] timer: FocusTemplateTimerConfig,
) -> impl leptos::IntoView {
    let style = compute_template_style(&kind);
    // let title = kind.to_owned().to_string();
    view! {
        <div style=style>
            <div class="border-b border-white border-[2px] flex items-center justify-between text-white">
                <span>Single Task</span>
                <ElipsesVertical />
            </div>

            <BaseText>{description}</BaseText>

            <HeadingText>{title}</HeadingText>
        </div>
    }
}

fn compute_template_style(kind: &FocusTemplateKind) -> String {
    let style_rule = match kind {
        FocusTemplateKind::Work => {
            r#"
        /* CONTENT BOX */

/* Auto layout */
display: flex;
flex-direction: column;
align-items: flex-start;
padding: 10px 20px;
gap: 10px;


height: 182px;
left: 0px;
top: 0px;

/* Gradient/sky blue */
background: linear-gradient(225deg, #68DBF2 0.01%, #509CF5 100%);
/* Shadow */
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;

        "#
        }
        FocusTemplateKind::Study => {
            r#"/* CONTENT BOX */

/* Auto layout */
display: flex;
flex-direction: column;
align-items: flex-start;
padding: 10px 20px;
gap: 10px;

height: 182px;
left: 0px;
top: 0px;

background: linear-gradient(70.45deg, #75CE8E 0%, #05595B 98.85%);
/* Shadow */
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;
"#
        }
        FocusTemplateKind::Personal => {
            r#"
        /* CONTENT BOX */

/* Auto layout */
display: flex;
flex-direction: column;
align-items: flex-start;
padding: 10px 20px;
gap: 10px;

height: 182px;
left: 0px;
top: 0px;

background: linear-gradient(225deg, #FFEF5E 0%, #F7936F 100%);
/* Shadow */
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;
"#
        }
        FocusTemplateKind::Fitness => todo!(),
    };

    style_rule.to_string()
}
