use spmc;
use std::io;
use std::thread;

fn main() {
    println!("Input N:");

    let mut n = String::new(); // считываем N

    io::stdin().read_line(&mut n).expect("Failed to read line");

    let n: u32 = match n.trim().parse() {
        Ok(num) => num,
        // break;
        Err(_) => panic!("Input value not integer"),
    }; // проверяем N на валидность

    let (mut sender, receiver) = spmc::channel(); // создаем spmc канал
    let mut handles = Vec::new();
    for worker_num in 0..n {
        let receiver = receiver.clone(); // клонируем получателя для каждого потока
        handles.push(thread::spawn(move || {
            let msg = receiver.recv().unwrap(); // считываем данные из канала
            println!("worker {} recvd: {}", worker_num, msg);
        }));
    }

    for i in 0..n {
        sender.send(i * 2).unwrap();
    }

    // Завершаем работу всех потоков
    for handle in handles {
        handle.join().unwrap();
    }
}
