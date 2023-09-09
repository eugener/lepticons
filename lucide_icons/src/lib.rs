mod types;

use leptos::*;
use strum::EnumProperty;
pub use types::LucideIconType;


// #[component]
// pub fn ProgressBar(
//     cx: Scope,
// //     // mark this prop optional
// //     // you can specify it or not when you use <ProgressBar/>
//     #[prop(optional)] _max: f64,
//     progress: ReadSignal<i32>
// ) -> impl IntoView {
//     view! { cx,
//         <progress
//             max=10
//             value=progress.get()
//         />
//     }
// }


impl LucideIconType {
    fn file(&self) -> String {
        self.get_str("File").unwrap().to_string()
    }

    fn to_file_name(&self) -> String {
        format!("icons/{}.svg", self.file())
    }
}


#[component]
pub fn LucideIcon(cx: Scope, icon_type: LucideIconType) -> impl IntoView {
    let file_name = icon_type.to_file_name();

    view! { cx,
        // <div class="text-sm">{file_name.to_owned()}</div>
        <img src={file_name.to_owned()} />
    }
}