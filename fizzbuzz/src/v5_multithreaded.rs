use std::thread;
use crossbeam::{
    channel::{unbounded, bounded},
    select
};

use crate::v3_iterator::{FizzBuzzable, FizzBuzzItem};

fn spawn_for(start: usize, end: usize) -> Vec<FizzBuzzItem> {
    let (sender, receiver) = unbounded();

    let handle = thread::spawn(move || {
        for i in start..=end {
            sender.send(i.fuzz()).unwrap();
        }
    });

    let mut res = Vec::with_capacity(end - start + 1);
    while let Ok(data) = receiver.recv() {
        res.push(data);
    }

    handle.join().unwrap();

    res
}

fn spawn_collect(start: usize, end: usize) -> Vec<FizzBuzzItem> {
    let (sender, receiver) = unbounded();

    let handle = thread::spawn(move || {
        for i in start..=end {
            sender.send(i.fuzz()).unwrap();
        }
    });

    let res = receiver.iter().collect();

    handle.join().unwrap();

    res
}

enum Token {
    Stop,
}

fn spawn_infinite(start: usize, end: usize) -> Vec<FizzBuzzItem> {
    let (sender, receiver) = bounded(10);
    // Not actually a one-shot channel, but still
    let (token_sender, token_receiver): (crossbeam::channel::Sender<Token>, crossbeam::channel::Receiver<Token>) = unbounded();

    let handle = thread::spawn(move || {
        for i in start.. {
            select! {
                send(sender, i.fuzz()) -> _res => {},
                recv(token_receiver) -> msg => {
                    if let Ok(t) = msg {
                        match t {
                            Token::Stop => {
                                break;
                            }
                        }
                    }
                },
            }
        }
    });

    let res = receiver.iter().take(end - start + 1).collect();

    token_sender.send(Token::Stop).unwrap();
    handle.join().unwrap();

    res
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple() {
        assert_eq!(
            vec![
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Number(11),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(13),
                FizzBuzzItem::Number(14),
                FizzBuzzItem::FizzBuzz
            ],
            spawn_for(10, 15)
        );
    }

    #[test]
    fn simple_collect() {
        assert_eq!(
            vec![
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Number(11),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(13),
                FizzBuzzItem::Number(14),
                FizzBuzzItem::FizzBuzz
            ],
            spawn_collect(10, 15)
        );
    }

    #[test]
    fn simple_infinite() {
        assert_eq!(
            vec![
                FizzBuzzItem::Buzz,
                FizzBuzzItem::Number(11),
                FizzBuzzItem::Fizz,
                FizzBuzzItem::Number(13),
                FizzBuzzItem::Number(14),
                FizzBuzzItem::FizzBuzz
            ],
            spawn_infinite(10, 15)
        );
    }
}