#![cfg(feature = "std")]

use std::sync::{Arc, Mutex, atomic::AtomicU32};

use osom_lib_arc::std::{StdCArc, StdCWeak};

#[test]
fn test_carc() {
    let arc = StdCArc::new(42).unwrap();
    assert_eq!(StdCArc::strong_count(&arc), 1);
    assert_eq!(StdCArc::weak_count(&arc), 1);
    let weak = StdCArc::downgrade(&arc).unwrap();
    assert_eq!(StdCArc::strong_count(&arc), 1);
    assert_eq!(StdCArc::weak_count(&arc), 2);
    let up = weak.upgrade().unwrap();
    assert_eq!(StdCArc::strong_count(&arc), 2);
    assert_eq!(StdCArc::weak_count(&arc), 2);
    drop(weak);
    assert_eq!(StdCArc::strong_count(&arc), 2);
    assert_eq!(StdCArc::weak_count(&arc), 1);
    drop(up);
    assert_eq!(StdCArc::strong_count(&arc), 1);
    assert_eq!(StdCArc::weak_count(&arc), 1);

    let arc_clone = arc.clone();
    assert_eq!(StdCArc::strong_count(&arc), 2);
    assert_eq!(StdCArc::weak_count(&arc), 1);
    assert_eq!(StdCArc::strong_count(&arc_clone), 2);
    assert_eq!(StdCArc::weak_count(&arc_clone), 1);
    assert_eq!(StdCArc::data(&arc), &42);
    assert_eq!(StdCArc::data(&arc_clone), &42);
}

#[test]
fn test_drop() {
    struct Data {
        pub counter: Arc<AtomicU32>,
    }

    impl Drop for Data {
        fn drop(&mut self) {
            self.counter.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    let counter = Arc::new(AtomicU32::new(0));
    let get = || counter.load(std::sync::atomic::Ordering::SeqCst);

    let data = StdCArc::new(Data {
        counter: counter.clone(),
    })
    .unwrap();
    assert_eq!(get(), 0);
    let data2 = data.clone();
    assert_eq!(get(), 0);
    drop(data2);
    assert_eq!(get(), 0);
    drop(data);
    assert_eq!(get(), 1);
}

#[test]
#[allow(unused)]
fn test_cyclic_dependencies() {
    struct Counter {
        inner: Arc<AtomicU32>,
    }

    impl Counter {
        pub fn new() -> Self {
            Self {
                inner: Arc::new(AtomicU32::new(0)),
            }
        }

        pub fn get(&self) -> u32 {
            self.inner.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    impl Drop for Counter {
        fn drop(&mut self) {
            self.inner.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        }
    }

    impl Clone for Counter {
        fn clone(&self) -> Self {
            self.inner.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
            Self {
                inner: self.inner.clone(),
            }
        }
    }

    struct Parent {
        children: Arc<Mutex<Vec<StdCArc<Child>>>>,
        counter: Counter,
    }

    struct Child {
        parent: StdCWeak<Parent>,
        counter: Counter,
    }

    let counter = Counter::new();
    let parent = Parent {
        children: Arc::new(Mutex::new(Vec::new())),
        counter: counter.clone(),
    };
    let parent = StdCArc::new(parent).unwrap();

    for _ in 0..10 {
        let child = Child {
            parent: StdCArc::downgrade(&parent).unwrap(),
            counter: counter.clone(),
        };
        let child = StdCArc::new(child).unwrap();
        let mut children = parent.children.lock().unwrap();
        children.push(child);
    }

    assert_eq!(counter.get(), 11);
    drop(parent);
    assert_eq!(counter.get(), 0);
}
