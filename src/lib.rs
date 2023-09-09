use leptos::*;
use strum_macros::Display;

 #[component]
pub fn ProgressBar(
    cx: Scope,
//     // mark this prop optional
//     // you can specify it or not when you use <ProgressBar/>
    #[prop(optional)] _max: f64,
    progress: ReadSignal<i32>
) -> impl IntoView {
    view! { cx,
        <progress
            max=10
            value=progress.get()
        />
    }
}


#[derive(Display)]
pub enum LucidIconType {
    ChevronsRight
}

impl LucidIconType {

    fn to_file_name(&self) -> String {
        format!("svg/{}.svg", split_camel_case(&self.to_string()))
    }
}

fn split_camel_case(s: &str) -> String {
    let mut result = String::new();
    for (i, char) in s.char_indices() {
        if char.is_uppercase() {
            if i != 0 { // Don't prefix the first word with a dash
                result.push('-');
            }
        }
        result.push(char.to_ascii_lowercase());
    }
    result
}

#[component]
pub fn LucidIcon(cx: Scope,  icon_type: LucidIconType ) -> impl IntoView {

    view! { cx,
        // <div>{icon_type.to_file_name()}</div>
        <img src={icon_type.to_file_name()} />
    }
}
