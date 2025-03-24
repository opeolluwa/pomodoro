use crate::{
    components::typography::{heading::HeadingText, paragraph::BaseText},
    icons::elipses::ElipsesVertical,
    state::templates::{FocusTemplateKind, FocusTemplateTimerConfig},
};
use leptos::prelude::*;

#[leptos::component]
pub fn FocusCardTemplate(
    #[prop()] kind: FocusTemplateKind,
    #[prop()] title: String,
    #[prop()] description: String,
    #[prop()] timer: FocusTemplateTimerConfig,
) -> impl leptos::IntoView {
    let style = compute_template_style(&kind);

    view! {
        <div style=style class="w-full pb-[5px]">
            <div class="border-b border-b-white/50 border-b-[1.2px] flex items-center justify-between text-white px-[20px] py-3">
                <span>Single Task</span>
                <ElipsesVertical />
            </div>

            <div class="px-[20px]">
                <small class="text-[#0D1C36] block mb-2">{kind.to_string()}</small>
                <HeadingText class="text-white text-[16px]">{title}</HeadingText>
                <BaseText class="text-white mt-2">{description}</BaseText>
            </div>

            <div class=" rounded-lg min-h-[40px] px-[20px] hidden bg-[rgba(245, 245, 245, 0.5)]">
                <BaseText class="text-white mt-2">Time estimate:</BaseText>
                {timer.count}
                hr
                {timer.long_break}
                min
            </div>
        </div>
    }
}

fn compute_template_style(kind: &FocusTemplateKind) -> String {
    let style_rule = match kind {
        FocusTemplateKind::Work => {
            r#"
min-height: 182px;
background: linear-gradient(225deg, #68DBF2 0.01%, #509CF5 100%);
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;

        "#
        }
        FocusTemplateKind::Study => {
            r#"
height: 182px;
background: linear-gradient(70.45deg, #75CE8E 0%, #05595B 98.85%);
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;
"#
        }
        FocusTemplateKind::Personal => {
            r#"
background: linear-gradient(180deg, #FF92AE 0%, #FF5A85 100%);
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;

"#
        }
        FocusTemplateKind::Fitness => {
            r#"
min-height: 182px;
background: linear-gradient(225deg, #FFEF5E 0%, #F7936F 100%);
box-shadow: 0px 1px 2px 1px rgba(5, 89, 91, 0.05), 0px 1px 1px rgba(5, 89, 91, 0.15);
border-radius: 8px;
"#
        }
    };

    style_rule.to_string()
}
