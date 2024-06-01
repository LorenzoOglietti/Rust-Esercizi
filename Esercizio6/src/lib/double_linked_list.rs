/*
Per questo esercizio implementerete una struttura di lista doppiamente linkata .
Di seguito ho messo  le dichiarazioni delle strutture dati e le funzioni e i metodi che dovete implementare.
Dovete implementare anche dei test usando come tipo di T qualcosa di piu’ complesso di un tipo primitivo,
, ad esempio una struttura punto con 2 componenti intere.
use std::{cell::RefCell, rc::Rc};
#[derive(Default)]
struct Node<T:Copy> {
	item: T,
	next: Pointer<T>,
	prev: Pointer<T>,
}
impl<T:Copy> Node<T> {
	fn new(item: T) -> Self
}

type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

#[derive(Default)]
pub struct DoublyPointedList<T:Copy> {
	head: Pointer<T>,
	tail: Pointer<T>,
	size: usize,
}
impl<T:Copy> DoublyPointedList<T> {
	pub fn new() -> Self
     pub fn is_empty(&self) -> bool
     pub fn len(&self) -> usize
pub fn push_back(&mut self, item: T)
	pub fn push_front(&mut self, item: T)
	pub fn pop_back(&mut self) -> Option<T>
	pub fn pop_front(&mut self) -> Option<T>
	// Se n e' positivo ritornate l'ennesimo elemento dall'inizio
     //della lista mentre se e' negativo lo ritornate dalla coda
	//(-1 e' il primo elemento dalla coda)
	pub fn get(& self, n:i32) -> Option<T>
}
 */


/// Modulo che implementa una lista doppiamente puntata generica.
pub mod double_linked_list {
    use std::{cell::RefCell, rc::Rc};

    /// Struttura che rappresenta un nodo della lista.
    #[derive(Default)]
    struct Node<T: Copy> {
        item: T,
        next: Pointer<T>,
        prev: Pointer<T>,
    }

    impl<T: Copy> Node<T> {
        /// Crea un nuovo nodo con l'elemento specificato.
        fn new(item: T) -> Self {
            Node {
                item,
                next: None,
                prev: None,
            }
        }
    }

    /// Tipo alias per semplificare la gestione dei puntatori ai nodi.
    type Pointer<T> = Option<Rc<RefCell<Node<T>>>>;

    /// Struttura che rappresenta una lista doppiamente puntata.
    #[derive(Default)]
    pub struct DoublyPointedList<T: Copy> {
        head: Pointer<T>,
        tail: Pointer<T>,
        size: usize,
    }

    impl<T: Copy> DoublyPointedList<T> {
        /// Crea una nuova lista doppiamente puntata.
        pub fn new() -> Self {
            DoublyPointedList {
                head: None,
                tail: None,
                size: 0,
            }
        }

        /// Verifica se la lista è vuota.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** `true` se la lista è vuota, altrimenti `false`.
        pub fn is_empty(&self) -> bool {
            self.size == 0
        }

        /// Restituisce la lunghezza della lista.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** La lunghezza della lista.
        pub fn len(&self) -> usize {
            self.size
        }

        /// Aggiunge un elemento alla fine della lista.
        ///
        /// **Input:**
        /// - `item`: L'elemento da aggiungere alla lista.
        pub fn push_back(&mut self, item: T) {
            let nodo = Node::new(item);
            let nodo_clone = Rc::new(RefCell::new(nodo));
            if self.is_empty() {
                self.head = Some(nodo_clone.clone());
                self.tail = Some(nodo_clone);
            } else {
                let nodocoda = self.tail.clone();
                nodo_clone.borrow_mut().prev = nodocoda.clone();
                nodocoda.unwrap().borrow_mut().next = Some(nodo_clone.clone());
                self.tail = Some(nodo_clone);
            }
            self.size += 1;
        }

        /// Aggiunge un elemento all'inizio della lista.
        ///
        /// **Input:**
        /// - `item`: L'elemento da aggiungere alla lista.
        pub fn push_front(&mut self, item: T) {
            let nodo = Node::new(item);
            let nodo_clone = Rc::new(RefCell::new(nodo));
            if self.is_empty() {
                self.head = Some(nodo_clone.clone());
                self.tail = Some(nodo_clone);
            } else {
                let nodotesta = self.head.clone();
                nodo_clone.borrow_mut().next = nodotesta.clone();
                nodotesta.unwrap().borrow_mut().prev = Some(nodo_clone.clone());
                self.head = Some(nodo_clone);
            }
            self.size += 1;
        }

        /// Rimuove e restituisce l'elemento dalla fine della lista.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** L'elemento rimosso, se presente.
        pub fn pop_back(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }

            let nodotail = self.tail.clone();
            let newtail_ref = nodotail.as_ref().unwrap().borrow_mut().prev.clone();

            nodotail.clone().unwrap().borrow_mut().prev = None;
            if let Some(ref newtail) = newtail_ref {
                newtail.borrow_mut().next = None;
            }

            self.tail = newtail_ref;
            self.size -= 1;
            Some(nodotail.unwrap().borrow_mut().item)
        }

        /// Rimuove e restituisce l'elemento dall'inizio della lista.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** L'elemento rimosso, se presente.
        pub fn pop_front(&mut self) -> Option<T> {
            if self.is_empty() {
                return None;
            }

            let nodohead = self.head.clone();
            let newhead_ref = nodohead.as_ref().unwrap().borrow_mut().next.clone();

            nodohead.clone().unwrap().borrow_mut().next = None;
            if let Some(ref newhead) = newhead_ref {
                newhead.borrow_mut().prev = None;
            }

            self.head = newhead_ref;
            self.size -= 1;
            Some(nodohead.unwrap().borrow_mut().item)
        }

        /// Restituisce l'ennesimo elemento dalla lista, partendo dall'inizio se `n` è positivo,
        /// o dalla fine se `n` è negativo.
        ///
        /// **Input:**
        /// - `n`: L'indice dell'elemento da restituire. Se positivo, parte dall'inizio della lista;
        ///        se negativo, parte dalla fine.
        ///
        /// **Valore di ritorno:** L'elemento alla posizione specificata, se presente.
        pub fn get(&self, n: i32) -> Option<T> {
            if self.is_empty() {
                return None;
            }

            let mut index = if n >= 0 { n } else { -n - 1 };
            let mut current_node = if n >= 0 { self.head.clone() } else { self.tail.clone() };

            while let Some(node) = current_node {
                if index == 0 {
                    return Some(node.borrow().item);
                }

                current_node = if n >= 0 { node.borrow().next.clone() } else { node.borrow().prev.clone() };
                index -= 1;
            }

            None
        }
    }
}
