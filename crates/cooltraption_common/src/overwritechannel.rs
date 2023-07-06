#![allow(dead_code)]
use std::cell::{Ref, RefCell};
use std::rc::Rc;

pub struct OverwriteChannelWriter<T> {
    rc: Rc<RefCell<T>>,
}

pub struct OverwriteChannelReader<T> {
    rc: Rc<RefCell<T>>,
}

impl<T> OverwriteChannelReader<T> {
    pub fn read(&self) -> Ref<T> {
        self.rc.borrow()
    }
}

impl<T> OverwriteChannelWriter<T> {
    pub fn write(&self, value: T) {
        self.rc.replace(value);
    }
}

pub fn overwrite_channel<T>(value: T) -> (OverwriteChannelWriter<T>, OverwriteChannelReader<T>) {
    let reader = OverwriteChannelReader {
        rc: Rc::new(RefCell::new(value)),
    };
    let writer = OverwriteChannelWriter {
        rc: Rc::clone(&reader.rc),
    };
    (writer, reader)
}
