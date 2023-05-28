// use std::fmt::Debug;
use std::{marker::PhantomData, sync::{Arc}};

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

pub struct ServiceResult<D, R> {
    iteration: u32,
    previous: Vec<Arc<R>>,
    initial_value: Option<Arc<D>>,
    current: Option<Arc<R>>,
}

impl<D, R> ServiceResult<D, R> {
    fn push_current(&mut self, value: R) {
        self.current = Some(Arc::new(value));
    }

    fn push_previous(&mut self) {
        match self.current.as_ref() {
            Some(val) => self.previous.push(Arc::clone(val)),
            None => {
                if self.iteration == 0 {
                    // Handle the case where the iteration is 0 and there is no value.
                } else {
                    panic!("?? iteration is not 0 and there is not value");
                }
            }
        };
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
    pub async fn execute(&self, stream: D) {
        let number_to_execute: u32 = self.services.len() as u32;

        let mut infrastructure_result = ServiceResult {
            iteration: 0,
            previous: vec![],
            initial_value: Some(Arc::new(stream)),
            current: None,
        };

        loop {
            if infrastructure_result.iteration < number_to_execute {
                let service = &self.services[infrastructure_result.iteration as usize];

                // Determine the appropriate argument for the call function
                let arg = if infrastructure_result.iteration == 0 {
                    infrastructure_result.initial_value.as_ref().unwrap()
                } else {
                    infrastructure_result.current.as_ref().unwrap()
                };

                // push new value to current
                infrastructure_result.push_current(
                    service.call().await
                );
                infrastructure_result.iteration += 1;
            } else {
                break;
            }
        }
    }
}