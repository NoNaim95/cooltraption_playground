#![allow(dead_code)]
use std::cell::{Ref, RefCell};
use std::sync::Arc;

pub struct OverwriteChannelWriter<T> {
    arc: Arc<RefCell<T>>,
}

pub struct OverwriteChannelReader<T> {
    arc: Arc<RefCell<T>>,
}

impl<T> OverwriteChannelReader<T> {
    pub fn read(&self) -> Ref<T> {
        self.arc.borrow()
    }
}

impl<T> OverwriteChannelWriter<T> {
    pub fn write(&self, value: T) {
        self.arc.replace(value);
    }
}

pub fn overwrite_channel<T>(value: T) -> (OverwriteChannelWriter<T>, OverwriteChannelReader<T>) {
    let reader = OverwriteChannelReader {
        arc: Arc::new(RefCell::new(value)),
    };
    let writer = OverwriteChannelWriter {
        arc: Arc::clone(&reader.arc),
    };
    (writer, reader)
}
