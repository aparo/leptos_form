use leptos::prelude::*;
use leptos_form;
use uuid::Uuid;

#[derive(leptos_form::Form, Clone)]
#[form(component)]
pub struct MyForm {
    pub id: Uuid,
    pub name: String,
    pub age: u32,
    pub date: chrono::NaiveDate,
}

#[leptos::component]
pub fn AnotherComponent() -> impl leptos::IntoView {
    let initial = MyForm {
        id: Uuid::new_v4(),
        name: "John Doe".to_string(),
        age: 30,
        date: chrono::NaiveDate::from_ymd_opt(2023, 10, 1).unwrap(),
    };
    view! {
        <MyForm initial={initial} />
    }
}
