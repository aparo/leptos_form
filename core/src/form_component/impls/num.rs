use crate::*;
use ::leptos::html::*;
use ::leptos::prelude::*;
use ::wasm_bindgen::{JsCast, JsValue};

macro_rules! num_impl {
    ($($ty:ty $({ $(type: $type:literal)?$(,)? $(min: $min:expr, max: $max:expr)?$(,)? })? ),*$(,)?) => { $(
        impl DefaultHtmlElement for $ty {
            type El = HtmlElement<Input, (), ()>;
        }

        impl FormField<HtmlElement<Input, (), ()>> for $ty {
            type Config = ();
            type Signal = FormFieldSignal<$ty>;

            fn default_signal(_: &Self::Config, initial: Option<Self>) -> Self::Signal {
                FormFieldSignal::new_with_default_value(initial)
            }
            fn is_default_value(_signal: &Self::Signal) -> bool {
                false
                // signal.value.with_untracked(|value| value == $ty::Default())
            }
            fn into_signal(self, _: &Self::Config, initial: Option<Self>) -> Self::Signal {
                FormFieldSignal::new(self, initial)
            }
            fn try_from_signal(signal: Self::Signal, _: &Self::Config) -> Result<Self, FormError> {
                Ok(signal.value.with_untracked(|value| *value))
            }
            fn recurse(signal: &Self::Signal) {
                signal.value.with_untracked(|_| {})
            }
            fn reset_initial_value(signal: &Self::Signal) {
                signal.value.with_untracked(|value| signal.initial.update(|initial| *initial = Some(value.clone())));
            }
            fn with_error<O>(signal: &Self::Signal, f: impl FnOnce(Option<&FormError>) -> O) -> O {
                signal.error.with_untracked(|error| f(error.as_ref()))
            }
        }

        #[cfg(feature = "thaw")]
        impl FormComponent<HtmlElement<Input, (), ()>> for $ty {
            fn render(props: RenderProps<Self::Signal, Self::Config>) -> impl IntoView {
                let class = props.class_signal();
                view! {
                    <thaw::SpinButton< $ty >
                        // type=num_impl!(@type $($($type)?)?)
                        class={class.get().map(|s| s.to_string())}
                        id={props.id.map(|s| s.to_string()).or_else(|| props.name.clone().map(|s| s.to_string()))}
                        max=num_impl!(@max $ty $($(, $max)?)?)
                        min=num_impl!(@min $ty $($(, $min)?)?)
                        step_page=(1 as $ty)
                        name={props.name.map(|s| s.to_string())}
                        on:keydown=num_impl!(@prevent_invalid_keystrokes value $($($type)?)?)
                        on:input=move |ev| {
                            let target_value = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                            props.signal.value.update(|value| *value = target_value.parse().unwrap_or_default())
                        }
                        on:change=move |_| {
                            if !props.is_optional || !<Self as FormField<HtmlElement<Input, (), ()>>>::is_default_value(&props.signal) {
                                if let Err(form_error) = <Self as FormField<HtmlElement<Input, (), ()>>>::try_from_signal(props.signal, &props.config) {
                                    props.signal.error.update(|error| *error = Some(form_error));
                                } else if props.signal.error.with_untracked(|error| error.is_some()) {
                                    props.signal.error.update(|error| *error = None);
                                }
                            } else {
                                props.signal.error.update(|error| *error = None);
                            }
                        }
                        prop:class={move || class.with_untracked(|x| x.as_ref().map(|x| JsValue::from_str(&*x)))}
                        prop:value={props.signal.value}
                        // style={props.style}
                        value=props.signal.value
                    />
                }
            }
        }
        #[cfg(not(feature = "thaw"))]
        impl FormComponent<HtmlElement<Input, (), ()>> for $ty {
            fn render(props: RenderProps<Self::Signal, Self::Config>) -> impl IntoView {
                let class = props.class_signal();
                view! {
                    <input
                        type=num_impl!(@type $($($type)?)?)
                        class={class}
                        id={props.id.or_else(|| props.name.clone())}
                        max=num_impl!(@max $ty $($(, $max)?)?)
                        min=num_impl!(@min $ty $($(, $min)?)?)
                        name={props.name}
                        on:keydown=num_impl!(@prevent_invalid_keystrokes value $($($type)?)?)
                        on:input=move |ev| {
                            let target_value = ev.target().unwrap().unchecked_into::<web_sys::HtmlInputElement>().value();
                            props.signal.value.update(|value| *value = target_value)
                        }
                        on:change=move |_| {
                            if !props.is_optional || !<Self as FormField<HtmlElement<Input, (), ()>>>::is_default_value(&props.signal) {
                                if let Err(form_error) = <Self as FormField<HtmlElement<Input, (), ()>>>::try_from_signal(props.signal, &props.config) {
                                    props.signal.error.update(|error| *error = Some(form_error));
                                } else if props.signal.error.with_untracked(|error| error.is_some()) {
                                    props.signal.error.update(|error| *error = None);
                                }
                            } else {
                                props.signal.error.update(|error| *error = None);
                            }
                        }
                        prop:class={move || class.with_untracked(|x| x.as_ref().map(|x| JsValue::from_str(&*x)))}
                        prop:value={props.signal.value}
                        style={props.style}
                        value=props.signal.value
                    />
                }
            }
        }
    )* };
    (@type $type:literal) => {$type};
    (@type) => {"number"};

    (@min $ty:ty, $min:expr) => {$min};
    (@min $ty:ty) => {<$ty>::MIN};

    (@max $ty:ty, $max:expr) => {$max};
    (@max $ty:ty) => {<$ty>::MAX};

    (@prevent_invalid_keystrokes $value:ident $type:literal) => {|ev| {
        let key = ev.key();
        if key.len() > 1 {
            return;
        }
        if let Some(c) = key.chars().next() {
            if !matches!(c, '0'|'1'|'2'|'3'|'4'|'5'|'6'|'7'|'8'|'9'|'.'|'-'|'+') {
                ev.prevent_default();
            }
        }
    }};
    (@prevent_invalid_keystrokes $value:ident) => {|_| {}};
}

num_impl!(
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    i8,
    i16,
    i32,
    i64,
    i128,
    isize,
    f32 { type: "text" },
    f64 { type: "text" },
);

#[cfg(feature = "num-bigint")]
num_impl!(
    num_bigint::BigInt {
        min: None::<&'static str>,
        max: None::<&'static str>
    },
    num_bigint::BigUint {
        min: "0",
        max: None::<&'static str>
    },
);

#[cfg(feature = "bigdecimal")]
num_impl!(
    bigdecimal::BigDecimal { type: "text", min: None::<&'static str>, max: None::<&'static str> },
);
