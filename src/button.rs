use dioxus::prelude::*;

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ButtonSize {
    Small,
    Big,
    Full,
}

#[component]
pub fn Button(
    value: String,
    size: ButtonSize,
    onclick: EventHandler<MouseEvent>,
    disabled: Option<bool>,
) -> Element {
    let dynamic_class = {
        let c_width = match size {
            ButtonSize::Small => "w-24",
            ButtonSize::Big => "w-32",
            ButtonSize::Full => "w-full",
        };
        let c_disabled = if disabled.unwrap_or_default() {
            "bg-disabled"
        } else {
            "bg-accent hover:bg-accent-hover cursor-pointer"
        };

        format!("{c_width} {c_disabled}")
    };

    rsx! {
        button {
            class: "h-8 flex items-center justify-center select-none",
            class: "text-primary rounded-md",
            class: "{dynamic_class}",
            onclick: onclick,
            disabled: disabled,
            {value}
        }
    }
}
