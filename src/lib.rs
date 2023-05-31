// I finally fixed it, thanks guys! - noctis
use indicatif;
use std::{marker::PhantomData, sync::Arc};

pub struct Service<R> {
    pub name: &'static str,
    pub function: Box<dyn Fn(&R) -> R>,
    _phantom: PhantomData<Box<fn(&R) -> R>>,
}

impl<R> Service<R> {
    pub fn new(name: &'static str, function: Box<dyn Fn(&R) -> R>) -> Self {
        Self {
            name,
            function,
            _phantom: PhantomData,
        }
    }

    pub async fn call(&self, arg: &R) -> R {
        (self.function)(arg)
    }
}

pub struct ServiceResult<R> {
    pub iteration: u32,
    pub previous: Vec<Arc<R>>,
    pub current: Arc<R>,
}

impl<R> ServiceResult<R> {
    fn push_current(&mut self, value: R) {
        self.current = Arc::new(value);
    }

    // maybe use ARC::new() to share a memory reference to the value so it doesn't need to be cloned.
    fn push_previous(&mut self) {
        self.previous.push(self.current.clone());
    }
}

pub struct Infrastructure<R> {
    pub services: Vec<Service<R>>,
    pub result: ServiceResult<R>,
}

impl<R> Infrastructure<R> {
    pub async fn execute(&mut self) -> &ServiceResult<R> {
        let number_to_execute: u32 = self.services.len() as u32;

        let progress = indicatif::ProgressBar::new(number_to_execute as u64);

        while self.result.iteration < number_to_execute {
            let service = &self.services[self.result.iteration as usize];
            let arg = &self.result.current;

            let result_from_current_service = service.call(arg).await;

            // push old value to previous
            self.result.push_previous();

            // push new value to current
            self.result.push_current(result_from_current_service);

            self.result.iteration += 1;
            progress.inc(1);
        }

        return &self.result;
    }
}
