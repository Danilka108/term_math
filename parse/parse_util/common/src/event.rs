use std::any::Any;
use std::collections::HashMap;
use std::marker::PhantomData;
use std::rc::Rc;
use crate::SpanWrapper;

pub type EResult<V = ()> = Result<V, SpanWrapper<String>>;

pub trait Event: Clone {
    fn __id() -> &'static str;
}

#[derive(Clone)]
pub struct Emitter<E: Event> {
    events_manager: EventsManager,
    _phantom: PhantomData<E>,
}

impl<E: 'static + Event> Emitter<E> {
    fn emit(&mut self, data: E) -> EResult {
        self.events_manager.emit(data)
    }
}

#[derive(Clone)]
pub struct EventsManager {
    events_listeners:
        Rc<HashMap<&'static str, Vec<(Box<dyn FnMut(Box<dyn Any>) -> EResult>, usize)>>>,
}

impl Default for EventsManager {
    fn default() -> Self {
        Self {
            events_listeners: Rc::new(HashMap::new()),
        }
    }
}

impl EventsManager {
    pub fn listen<E: 'static + Event, F: 'static + FnMut(E) -> EResult>(
        &mut self,
        mut callback: F,
        priority: usize,
    ) {
        let events_listeners = if let Some(e) = Rc::get_mut(&mut self.events_listeners) {
            e
        } else {
            return;
        };

        let event_listeners = match events_listeners.get_mut(E::__id()) {
            Some(l) => l,
            _ => {
                events_listeners.insert(E::__id(), Vec::new());
                match events_listeners.get_mut(E::__id()) {
                    Some(l) => l,
                    _ => return,
                }
            }
        };

        let callback_wrapper = move |data: Box<dyn Any>| {
            if let Ok(data) = data.downcast() {
                callback(*data)
            } else {
                Ok(())
            }
        };

        event_listeners.push((Box::new(callback_wrapper), priority));
    }

    pub fn emit<E: 'static + Event>(&mut self, data: E) -> EResult {
        let event_listeners = match Rc::get_mut(&mut self.events_listeners) {
            Some(e) => match e.get_mut(E::__id()) {
                Some(l) => l,
                _ => return Ok(()),
            },
            _ => return Ok(()),
        };

        event_listeners.sort_by(|a, b| a.1.cmp(&b.1).reverse());
        let (_, errors): (Vec<_>, Vec<_>) = event_listeners
            .iter_mut()
            .map(move |(callback, _)| callback(Box::new(data.clone())))
            .partition(|r| r.is_ok());

        match errors.first() {
            Some(Err(span_wrapper)) => Err(span_wrapper.clone()),
            _ => Ok(()),
        }
    }

    pub fn get_emitter<E: 'static + Event>(&self) -> Emitter<E> {
        Emitter {
            events_manager: self.clone(),
            _phantom: PhantomData,
        }
    }
}
