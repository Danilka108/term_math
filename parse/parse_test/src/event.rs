use std::rc::Rc;
use std::cell::RefCell;
use crate::context::PResult;

#[derive(Clone)]
pub struct EventManager<D: Clone> {
    listeners: Rc<RefCell<Vec<Box<dyn FnMut(D) -> PResult>>>>,
}

impl<D: Clone> Default for EventManager<D> {
    fn default() -> Self {
        Self {
            listeners: Default::default()
        }
    }
}

impl<D: Clone> EventManager<D> {
    pub fn listen(&self, listener: impl FnMut(D) -> PResult + 'static) {
        let mut listeners = self.listeners.borrow_mut();
        listeners.push(Box::new(listener));
    }

    fn notify(&self, data: D) -> PResult {
        let mut listeners = self.listeners.borrow_mut();

        for listen in listeners.iter_mut() {
            listen(data.clone())?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct Emitter<D: Clone> {
    event_manager: EventManager<D>,
}

impl<D: Clone> Emitter<D> {
    pub fn new(event_manager: EventManager<D>) -> Self {
        Self {
            event_manager,
        }
    }

    pub fn emit(&self, data: D) -> PResult {
        self.event_manager.notify(data)
    }
}
