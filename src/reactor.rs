use std::marker::PhantomData;

use ::Update;

struct Upgrade<T, U> {
    update: Box<Update<Message = T>>,
    _message: PhantomData<U>,
}

impl<T, U> Upgrade<T, U> where T: From<U> {
    fn wrap(update: Box<Update<Message = T>>) -> Self {
        Upgrade {
            update,
            _message: PhantomData,
        }
    }
}

impl<T, U> Update for Upgrade<T, U>
where
    T: 'static + Send + From<U>,
    U: 'static + Send,
{
    type Message = U;

    fn reduce(&self, message: U) {
        self.update.reduce(T::from(message));
    }

    fn clone(&self) -> Box<Update<Message = U>> {
        Box::new(Upgrade {
            update: self.update.clone(),
            _message: PhantomData,
        })
    }
}

pub struct Reactor<M> {
    update: Box<Update<Message = M>>,
}

impl<M> Reactor<M> where M: Send + 'static {
    pub fn new<U>(update: U) -> Self where U: Update<Message = M> {
        Self { update: Box::new(update) }
    }

    pub fn downgrade<T>(&self) -> Reactor<T>
    where
        M: From<T>,
        T: 'static + Send,
    {
        Reactor {
            update: Box::new(Upgrade::wrap(self.update.clone())),
        }
    }
}
