#![allow(dead_code)]
use std::cell::{Ref, RefCell};
use std::rc::Rc;

struct OverwriteChannelWriter<T> {
    rc: Rc<RefCell<T>>,
}

struct OverwriteChannelReader<T> {
    rc: Rc<RefCell<T>>,
}

impl<T> OverwriteChannelReader<T> {
    fn read(&self) -> Ref<T> {
        self.rc.borrow()
    }
}

impl<T> OverwriteChannelWriter<T> {
    fn write(&self, value: T) {
        self.rc.replace(value);
    }
}

fn overwrite_channel<T>(value: T) -> (OverwriteChannelWriter<T>, OverwriteChannelReader<T>) {
    let reader = OverwriteChannelReader {
        rc: Rc::new(RefCell::new(value)),
    };
    let writer = OverwriteChannelWriter {
        rc: Rc::clone(&reader.rc),
    };
    (writer, reader)
}
