mod form;
use form::AnotherComponent;
use leptos::prelude::*;
use thaw::*;

fn main() {
    _ = console_log::init_with_level(log::Level::Debug);
    console_error_panic_hook::set_once();

    mount_to_body(|| {
        let open = RwSignal::new(false);
        view! {
            <ConfigProvider>
             <Button on_click=move |_| open.set(true)>"Open Dialog"</Button>
                <Dialog open>
                    <DialogSurface>
                        <DialogBody>
                            <DialogTitle>"Dialog title"</DialogTitle>
                            <DialogContent>
                                <AnotherComponent />
                            </DialogContent>
                            <DialogActions>
                                <Button appearance=ButtonAppearance::Primary>"Do Something"</Button>
                            </DialogActions>
                        </DialogBody>
                    </DialogSurface>
                </Dialog>
            </ConfigProvider>
        }
    })
}
