use std::{
    marker::PhantomData,
    mem::forget,
    sync::Arc,
    sync::mpsc::{Sender, channel},
    thread::{park, spawn},
};

use tauri_runtime::UserEvent;
use tauri_runtime_wry::{
    Context, EventLoopIterationContext, Message, PluginBuilder, WebContextStore, WindowMessage,
    WindowsStore,
    tao::{
        event::{Event, WindowEvent},
        event_loop::{ControlFlow, EventLoopProxy, EventLoopWindowTarget},
    },
};

struct Msg<T: 'static> {
    windows: Arc<WindowsStore>,
    proxy: EventLoopProxy<Message<T>>,
}

pub struct TauriRuntimeWryNaughtyRefcellPlugin;

impl<T: UserEvent> PluginBuilder<T> for TauriRuntimeWryNaughtyRefcellPlugin {
    type Plugin = Plugin<T>;

    fn build(self, _: Context<T>) -> Self::Plugin {
        let (tx, rx) = channel();
        spawn(move || {
            loop {
                if let Ok(Msg { windows, proxy }) = rx.recv() {
                    loop {
                        if let Ok(windows) = windows.0.try_borrow_mut() {
                            for (&id, _) in windows.iter() {
                                if let Err(e) =
                                    proxy.send_event(Message::Window(id, WindowMessage::Close))
                                {
                                    eprintln!("Error sending WindowMessage::Close: {e}");
                                }
                            }
                            forget(windows);
                            park();
                        }
                    }
                }
            }
        });

        Self::Plugin {
            tx,
            _marker: PhantomData,
        }
    }
}

#[doc(hidden)]
pub struct Plugin<T: 'static> {
    tx: Sender<Msg<T>>,
    _marker: PhantomData<T>,
}

impl<T: UserEvent> tauri_runtime_wry::Plugin<T> for Plugin<T> {
    fn on_event(
        &mut self,
        event: &Event<Message<T>>,
        _: &EventLoopWindowTarget<Message<T>>,
        proxy: &EventLoopProxy<Message<T>>,
        _: &mut ControlFlow,
        context: EventLoopIterationContext<'_, T>,
        _: &WebContextStore,
    ) -> bool {
        if let Event::WindowEvent {
            event: WindowEvent::Focused(true),
            ..
        } = event
        {
            self.tx
                .send(Msg {
                    windows: context.windows,
                    proxy: proxy.clone(),
                })
                .unwrap();
        }

        false
    }
}
