use std::{
    sync::mpsc::channel,
    thread::{self, JoinHandle},
};

struct Message<T> {
    r#type: MessageType,
    payload: Option<T>,
}

enum MessageType {
    Done,
    Data,
}

fn main() {
    let (tx, rx) = channel();

    let _ = (0..=100)
        .into_iter()
        .map(|x| (x, tx.clone()))
        .map(|(i, tx)| {
            thread::spawn(move || {
                match tx.send(Message {
                    r#type: MessageType::Data,
                    payload: Some(i),
                }) {
                    Ok(_) => (),
                    Err(err) => panic!("can't send: {}", err),
                }
            })
        })
        .map(JoinHandle::join)
        .collect::<Vec<_>>();

    tx.send(Message {
        r#type: MessageType::Done,
        payload: None,
    })
    .unwrap();

    let mut sum: u32 = 0;

    for recv in rx {
        match recv.r#type {
            MessageType::Data => sum += recv.payload.unwrap(),
            MessageType::Done => break,
        }
    }

    println!("sum: {}", sum);
}
