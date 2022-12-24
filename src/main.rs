use std::{
    fs::{File, OpenOptions},
    io::{Read, Write},
    path::PathBuf,
};

mod events;

struct EventFile {
    event_listener: events::EventPublisher,
    file: File,
    path: PathBuf,
}

impl EventFile {
    fn new(file: PathBuf) -> Self {
        Self {
            event_listener: events::EventPublisher::default(),
            file: OpenOptions::new()
                .create(true)
                .read(true)
                .write(true)
                .open(&file)
                .unwrap(),
            path: file,
        }
    }

    fn read(&mut self, buffer: &mut String) {
        self.event_listener.notify(events::Event::Load, &self.path);
        self.file.read_to_string(buffer).unwrap();
    }

    fn write(&mut self, buffer: &String) {
        self.event_listener.notify(events::Event::Save, &self.path);
        self.file.write(buffer.as_bytes()).unwrap();
    }

    fn on_read(&mut self, listener: fn(PathBuf)) {
        self.event_listener.subscribe(events::Event::Load, listener)
    }
    fn on_write(&mut self, listener: fn(PathBuf)) {
        self.event_listener.subscribe(events::Event::Save, listener)
    }
}

fn main() {
    println!("Hello, world!");
}
