// use std::fmt::Debug;
use std::{marker::PhantomData, sync::Arc};

pub struct Service<F, D, R>
where
    F: Fn(&D) -> R,
{
    pub name: &'static str,
    pub function: F,
    _phantom: PhantomData<fn(&D) -> R>,
}

impl<F, D, R> Service<F, D, R>
where
    F: Fn(&D) -> R,
{
    pub fn new(name: &'static str, function: F) -> Self {
        Self {
            name,
            function,
            _phantom: PhantomData,
        }
    }

    pub async fn call(&self, arg: &D) -> R {
        (self.function)(arg)
    }
}

pub struct ServiceResult<R> {
    iteration: u32,
    previous: Vec<R>,
    current: R,
}

impl<R> ServiceResult<R> {
    fn push_current(&mut self, value: R) {
        self.current = value;
    }

    fn push_previous(&mut self) {
        self.previous.push(self.current);
    }
}

pub struct Infrastructure<F, D, R>
where
    F: Fn(&D) -> R,
{
    pub services: Vec<Service<F, D, R>>,
}

impl<F, D, R> Infrastructure<F, D, R>
where
    F: Fn(&D) -> R,
{
    pub async fn execute(&self, stream: R) {
        let number_to_execute: u32 = self.services.len() as u32;

        let mut infrastructure_result = ServiceResult {
            iteration: 0,
            previous: vec![],
            current: stream,
        };

        loop {
            if infrastructure_result.iteration < number_to_execute {
                let service = &self.services[infrastructure_result.iteration as usize];
                let arg = infrastructure_result.current;

                let result_from_current_service = service.call(&arg).await;

                // push old value to previous
                infrastructure_result.push_previous();

                // push new value to current
                infrastructure_result.push_current(result_from_current_service);

                infrastructure_result.iteration += 1;
            } else {
                break;
            }
        }
    }
}
