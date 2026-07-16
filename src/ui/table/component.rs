use dioxus::prelude::*;

#[css_module("/src/ui/table/style.css")]
struct Styles;

/// The data table shell. Wraps a horizontally scrollable container +
/// `<table>`.
#[component]
pub fn Table(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        div { class: Styles::dx_table_wrap,
            table {
                class: Styles::dx_table,
                ..attributes,
                {children}
            }
        }
    }
}

#[component]
pub fn TableHeader(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        thead { class: Styles::dx_table_header, ..attributes, {children} }
    }
}

#[component]
pub fn TableBody(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        tbody { class: Styles::dx_table_body, ..attributes, {children} }
    }
}

#[component]
pub fn TableRow(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        tr { class: Styles::dx_table_row, ..attributes, {children} }
    }
}

#[component]
pub fn TableHead(
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        th { class: Styles::dx_table_head, ..attributes, {children} }
    }
}

#[component]
pub fn TableCell(
    #[props(default)] colspan: i64,
    #[props(extends = GlobalAttributes)] attributes: Vec<Attribute>,
    children: Element,
) -> Element {
    rsx! {
        td {
            class: Styles::dx_table_cell,
            colspan: if colspan > 0 { colspan },
            ..attributes,
            {children}
        }
    }
}

/// The placeholder row rendered inside `TableBody` when the list is empty.
#[component]
pub fn EmptyRow(colspan: i64, message: String) -> Element {
    rsx! {
        TableRow {
            td {
                class: Styles::dx_table_empty_cell,
                colspan,
                "{message}"
            }
        }
    }
}
