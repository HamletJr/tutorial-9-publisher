# Tutorial 9
**Nama:**   Joshua Montolalu<br>
**NPM:**    2306275746<br>
**Kelas:**  Pengjut A<br>

## Modul 9
### Reflection 1
1. How much data your publisher program will send to the message broker in one run?
    - Berdasarkan kode dalam `main.rs`:
    ```rs
    fn main() {
        let mut p = CrosstownBus::new_queue_publisher("amqp://guest:guest@localhost:5672".to_owned()).unwrap();
        _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "1".to_owned(), user_name: "2306275746-Amir".to_owned() });
        _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "2".to_owned(), user_name: "2306275746-Budi".to_owned() });
        _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "3".to_owned(), user_name: "2306275746-Cica".to_owned() });
        _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "4".to_owned(), user_name: "2306275746-Dira".to_owned() });
        _ = p.publish_event("user_created".to_owned(), UserCreatedEventMessage {
        user_id: "5".to_owned(), user_name: "2306275746-Emir".to_owned() });
    }
    ```
    Terlihat ada 5 pemanggilan *method* `publish_event`, sehingga dalam satu *run* akan ada 5 *event*/*data* yang dikirim ke *message broker*.

2. The url of: `amqp://guest:guest@localhost:5672` is the same as in the subscriber program, what does it mean?
    - Karena URL yang digunakan sama dengan URL pada program *subscriber*, kedua program akan tersambung pada instance RabbitMQ yang sama. Dengan kata lain, kedua program akan tersambung pada *message broker* yang sama dan bisa saling berkomunikasi; *publisher* akan mengirim *event* ke *message broker*, dan subscriber akan *consume* dan proses *event* tersebut dari *message broker* yang sama.

### Reflection 2 (Running RabbitMQ)
![Screenshot of running RabbitMQ on local machine](RabbitMQ.png)

### Reflection 3 (Sending and processing events)
![Screenshot of sending events with publisher and consuming events with subscriber](SendingProcessingEvent.png)

### Reflection 4 (RabbitMQ chart)
![Screenshot of spikes in RabbitMQ dashboard chart](Spike.png)
- Graf kedua yang terlihat pada *screenshot* (yang memiliki lonjakan/*spike*) adalah graf yang menunjukkan kecepatan *event* di-*publish* (garis berwarna kuning) dan di-*consume* (garis berwarna ungu) lewat RabbitMQ. Terjadi lonjakan karena *publisher* mengirim 5 *event* sekaligus ke RabbitMQ ketika dijalankan, sehingga ketika dijalankan tiga kali, ada 3 lonjakan pada garis kuning. *Event*-*event* tersebut kemudian di-*consume* oleh *subscriber* sehingga setelah beberapa saat, terdapat lonjakan pada garis berwarna ungu.