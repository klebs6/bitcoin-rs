crate::ix!();

pub trait GetType {
    fn get_type(&self) -> i32;
}

pub trait GetVersion {
    fn get_version(&self) -> i32;
}

pub trait StreamItems {
    fn stream<Item>(&mut self, x: Item);
}

pub trait StreamInto {
    fn stream_into<Item>(&self, x: &mut Item);
}

pub trait Init {

    /**
      | Initialize internal state from the
      | database and block index.
      |
      */
    fn init(&mut self) -> bool;
}
