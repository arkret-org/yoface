use dioxus::prelude::*;
use dioxus_icons::lucide::{Check, ChevronDown};
use dioxus_primitives::dioxus_attributes::attributes;
use dioxus_primitives::merge_attributes;
pub use dioxus_primitives::select::SelectGroup;
use dioxus_primitives::select::{
    self, SelectGroupLabelProps, SelectMultiProps, SelectOptionProps, SelectProps,
};

#[css_module("/src/ui/select/style.css")]
struct Styles;

#[component]
pub fn Select<T: Clone + PartialEq + 'static>(props: SelectProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::Select {
            value: props.value,
            default_value: props.default_value,
            on_value_change: props.on_value_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectMulti<T: Clone + PartialEq + 'static>(props: SelectMultiProps<T>) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectMulti {
            values: props.values,
            default_values: props.default_values,
            on_values_change: props.on_values_change,
            disabled: props.disabled,
            open: props.open,
            default_open: props.default_open,
            on_open_change: props.on_open_change,
            name: props.name,
            roving_loop: props.roving_loop,
            typeahead_timeout: props.typeahead_timeout,
            attributes: merged,
            select::SelectTrigger {
                class: Styles::dx_select_trigger,
                select::SelectValue {}
                ChevronDown {
                    class: "dx-select-expand-icon",
                    size: "20px",
                    stroke: "var(--primary-color-7)",
                }
            }
            select::SelectList {
                class: Styles::dx_select_list,
                {props.children}
            }
        }
    }
}

#[component]
pub fn SelectGroupLabel(props: SelectGroupLabelProps) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_group_label
    });
    let merged = merge_attributes(vec![base, props.attributes]);

    rsx! {
        select::SelectGroupLabel {
            id: props.id,
            attributes: merged,
            {props.children}
        }
    }
}

#[component]
pub fn SelectOption<T: Clone + PartialEq + std::fmt::Display + 'static>(
    props: SelectOptionProps<T>,
) -> Element {
    let base = attributes!(div {
        class: Styles::dx_select_option
    });
    // An e2e test hook: the dxc Select renders a custom popup layer (divs with
    // `role="option"`, not native `<option>`s), and Playwright's
    // `locator.selectOption()` only works on a native `<select>`. Expose the
    // underlying value as `data-value` so cotest's `selectDxcOption()` helper
    // can locate and click by value. The value is identical to the string that
    // was passed to the original `selectOption(value)`.
    let value_attr = attributes!(div {
        "data-value": props.value.cloned().to_string()
    });
    let merged = merge_attributes(vec![base, value_attr, props.attributes]);

    rsx! {
        select::SelectOption::<T> {
            value: props.value,
            text_value: props.text_value,
            disabled: props.disabled,
            id: props.id,
            index: props.index,
            aria_label: props.aria_label,
            aria_roledescription: props.aria_roledescription,
            attributes: merged,
            {props.children}
            select::SelectItemIndicator {
                Check {
                    size: "1rem",
                    stroke: "var(--secondary-color-5)",
                }
            }
        }
    }
}
