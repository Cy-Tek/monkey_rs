use std::any::Any;

pub trait AsAny: Any {
    fn as_any(&self) -> &dyn Any;
    fn as_any_mut(&mut self) -> &mut dyn Any;
    fn type_name(&self) -> &'static str;
}

impl<T: Any> AsAny for T {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn type_name(&self) -> &'static str {
        std::any::type_name::<T>()
    }
}

pub trait Downcast: AsAny {
    fn is<T: AsAny>(&self) -> bool {
        self.as_any().is::<T>()
    }

    fn downcast_ref<T: AsAny>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    fn downcast_mut<T: AsAny>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}

impl<T: ?Sized + AsAny> Downcast for T {}
