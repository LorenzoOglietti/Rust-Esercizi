use std::collections::HashMap;
use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::borrow::Borrow;
use std::thread::ThreadId;
use rand::{random, Rng};
/// Struttura che rappresenta un oggetto all'asta.
#[derive(Clone, Debug)]
struct AuctionItem {
    /// Descrizione dell'oggetto all'asta.
    description: String,
    /// Prezzo minimo dell'oggetto all'asta.
    min_price: usize,
    /// ThreadId del vincitore temporaneo dell'asta.
    vincitore_temp: ThreadId,
}

impl AuctionItem {
    /// Crea un nuovo oggetto all'asta con la descrizione, il prezzo minimo e il ThreadId del vincitore temporaneo specificati.
    fn new(description: String, min_price: usize, vincitore_temp: ThreadId) -> Self {
        AuctionItem {
            description,
            min_price,
            vincitore_temp,
        }
    }
}

fn main() {
    let mut auction_item = AuctionItem::new("Vaso".to_string(), 100, thread::current().id());
    let num_threads = 5;
    let mut iterazione = Arc::new(Mutex::new(0));

    let mut handles = vec![];
    let mut channels_from_main = Vec::with_capacity(num_threads);
    let mut channels_from_child = Vec::with_capacity(num_threads);
    let mut channels_conf = Vec::with_capacity(num_threads);
    let mut channels_conf_ack = Vec::with_capacity(num_threads);
    let mut channels_termine_asta = Vec::with_capacity(num_threads);

    let mut partecipanti = Arc::new(Mutex::new(HashMap::new())); //Qui abbiamo tutti i partecipanti all'asta
    let mut info_generali = Arc::new(Mutex::new(AuctionItem::new("".to_string(), 0,thread::current().id())));

    for _ in 0..num_threads {

        let (main_conf, figlio_conf) = mpsc::channel();
        let (figlio_conf_acked, main_conf_acked) = mpsc::channel();
        let (main_mittente, figlio_ricevitore) = mpsc::channel::<AuctionItem>();
        let (figlio_mittente, main_ricevitore) = mpsc::channel();
        let (main_termine, figlio_termine) = mpsc::channel();
        //Io potevo creare due canali, n dal main ai figli e uno per ogni figlio dal figlio al main. Per far ciò mi basta mettere un tipo generico al canale

        channels_conf.push(main_conf.clone());
        channels_conf_ack.push(main_conf_acked);
        channels_from_main.push(main_mittente.clone());
        channels_from_child.push(main_ricevitore);
        channels_termine_asta.push(main_termine.clone());

        let partecipanti_clone = partecipanti.clone();
        let iterazione_clone = Arc::clone(&iterazione);
        let info_generali_clone = info_generali.clone();

        let handle = thread::spawn(move || {

            figlio_conf.recv().unwrap();

            let msg_ack = ("ACK", thread::current().id());

            figlio_conf_acked.send(msg_ack).unwrap();

            loop {
                let msg = figlio_ricevitore.recv().unwrap();

                let iterazione_value = iterazione_clone.lock().unwrap().clone();

                if iterazione_value == 0 || partecipanti_clone.lock().unwrap().get(&thread::current().id()) == Some(&1) {

                    if thread::current().id() == msg.vincitore_temp {

                        let messaggio = (thread::current().id(), (1, msg.min_price));
                        figlio_mittente.send(messaggio).unwrap();

                    } else {

                        let mut messaggio = (thread::current().id(), (0, 0));
                        let interested = interested(msg.min_price);

                        // Imposto il flag a 1 se il thread accetta l'offerta, altrimenti a 0
                        messaggio.1 = if interested.0 { (1, interested.1) } else { (0, 0) };

                        figlio_mittente.send(messaggio).unwrap();
                    }

                } else {
                    let messaggio = (thread::current().id(), (0, 0));
                    figlio_mittente.send(messaggio).unwrap();
                }

                let (vincitore_temp, msg) = figlio_termine.recv().unwrap();
                println!("{}", msg);
                if msg == "Asta terminata" {
                    if thread::current().id() == vincitore_temp {
                        println!("{:?} Hai vinto!", thread::current().id());
                        let info_value = info_generali_clone.lock().unwrap().clone();
                        println!("Vincitore {:?}", info_value);
                        break;
                    } else {
                        println!("{:?} Asta terminata", thread::current().id());
                        let info_value = info_generali_clone.lock().unwrap().clone();
                        println!("Perdente {:?}", info_value);
                        break;
                    }
                }
            }
        });
        handles.push(handle);
    }


    for main_conf in &channels_conf {
        main_conf.send("Aspetto").unwrap();
    }

    for main_conf_acked in &channels_conf_ack {
        let msg = main_conf_acked.recv().unwrap();
        partecipanti.lock().unwrap().insert(msg.1, 1); //Imposto che ogni thread partecipa all'asta
    }

    loop {

        for main_mittente in &channels_from_main {
            let item_to_send = auction_item.clone();
            if let Err(err) = main_mittente.send(item_to_send) {
                eprintln!("Errore nell'invio del messaggio: {:?}", err);
            }
            let mut iterazione_guard = iterazione.lock().unwrap();
            *iterazione_guard += 1;
            //println!("Iterazione main: {:?}", iterazione_guard);

        }

        for main_ricevitore in &channels_from_child {
            let msg = main_ricevitore.recv().unwrap();
            if msg.1.0 == 0 {
                partecipanti.lock().unwrap().insert(msg.0, 0);
            }
            if msg.1.1 >= auction_item.min_price {
                auction_item.min_price = msg.1.1;
                auction_item.vincitore_temp = msg.0;
            }
            println!("{:?}", msg);
            println!("Offerta maggiore: {} {:?}", auction_item.min_price, auction_item.vincitore_temp);
        }


        let count = {
            let partecipanti_guard = partecipanti.lock().unwrap();
            partecipanti_guard.values().filter(|&&v| v == 1).count()
        };


        if count == 1 || count == 0 {
            if auction_item.vincitore_temp == thread::current().id() {
                for main_mittente in &channels_termine_asta {
                    main_mittente.send((auction_item.vincitore_temp, "Asta deserta")).unwrap();
                };
            } else {
               for main_mittente in &channels_termine_asta {
                   main_mittente.send((auction_item.vincitore_temp, "Asta terminata")).unwrap();
               }
            }
            let mut info_guard = info_generali.lock().unwrap();
            *info_guard = auction_item;
            break;
        } else {
            for main_mittente in &channels_termine_asta {
                main_mittente.send((auction_item.vincitore_temp, "Asta in corso")).unwrap();
            }
        }

        println!("partecipanti post prima offerta {:?}", *partecipanti.lock().unwrap());

        // Breve attesa prima di inviare nuovamente le offerte
        thread::sleep(std::time::Duration::from_secs(1));
    }

    for handle in handles {
        handle.join().unwrap();
    }

    println!("Tutti i thread hanno terminato.");
}
/// Funzione ausiliaria per determinare se un partecipante è interessato all'offerta corrente.
fn interested(cifra: usize) -> (bool, usize) {
    let mut rng = rand::thread_rng();
    let offset = rng.gen_range(-50..100); // Genera un numero casuale tra -50 e 10
    let random_number = cifra as i32 + offset;
    if random_number > cifra as i32 {
        return (true, random_number as usize);
    } else {
        return (false, random_number as usize);
    }
}


