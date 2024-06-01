/*Fare una implementazione delle liste doppiamenti linkate come la precedente, ma i metodo get
                rimuove l’elemento nella posizione n e lo ritorna.
                Per questa implementazione il tipo generico T non deve implementare Copy.

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
    struct Node<T> {
        item: T,
        next: Pointer<T>,
        prev: Pointer<T>,
    }

    impl<T> Node<T> {
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
    pub struct DoublyPointedList<T> {
        head: Pointer<T>,
        tail: Pointer<T>,
        size: usize,
    }

    impl<T: std::default::Default> DoublyPointedList<T> {
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
            if self.size == 0 {
                return true;
            }
            return false;
        }
        /// Restituisce la lunghezza della lista.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** La lunghezza della lista.
        pub fn len(&self) -> usize {
            return self.size;
        }

        /// Aggiunge un elemento alla fine della lista.
        ///
        /// **Input:**
        /// - `item`: L'elemento da aggiungere alla lista.
        pub fn push_back(&mut self, item: T) { //Metti in coda un elemento
            /*let nodo = Node::new(item);
            let nodo_clone = Rc::new(RefCell::new(nodo));
            if self.is_empty() {
                self.head = Some(nodo_clone.clone());
                self.tail = Some(nodo_clone);
            } else {
                let nodocoda = self.tail.clone().unwrap();
                nodocoda.borrow_mut().next = Some(nodo_clone.clone());
                nodo_clone.borrow_mut().prev = Some(nodocoda);
                self.tail = Some(nodo_clone);
            }
            self.size += 1;*/

            let node = Rc::new(RefCell::new(Node::new(item)));
            if let Some(prev_tail) = self.tail.take() {
                prev_tail.borrow_mut().next = Some(Rc::clone(&node));
                node.borrow_mut().prev = Some(prev_tail);
                self.tail = Some(node);
                self.size += 1;
            } else {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
                self.size = 1;
            }
        }

        /// Aggiunge un elemento all'inizio della lista.
        ///
        /// **Input:**
        /// - `item`: L'elemento da aggiungere alla lista.

        pub fn push_front(&mut self, item: T) { //Metti in testa un elemento
            /*let nodo = Node::new(item);
            let nodo_clone = Rc::new(RefCell::new(nodo));
            if self.is_empty() {
                self.head = Some(nodo_clone.clone());
                self.tail = Some(nodo_clone);
            } else {
                let head_node = self.head.clone().unwrap();
                head_node.borrow_mut().prev = Some(nodo_clone.clone());
                nodo_clone.borrow_mut().next = Some(head_node);
                self.head = Some(nodo_clone);
            }
            self.size += 1;*/

            let node = Rc::new(RefCell::new(Node::new(item)));
            if let Some(prev_head) = self.head.take() {
                prev_head.borrow_mut().prev = Some(Rc::clone(&node));
                node.borrow_mut().next = Some(prev_head);
                self.head = Some(node);
                self.size += 1;
            } else {
                self.head = Some(Rc::clone(&node));
                self.tail = Some(node);
                self.size = 1;
            }
        }
        /// Rimuove e restituisce l'elemento dalla fine della lista.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** L'elemento rimosso, se presente.
        pub fn pop_back(&mut self) -> Option<T> { //Togli dalla coda un elemento
            /*if self.is_empty() {
                return None;
            }

            let old_tail = self.tail.take()?;
            self.size -= 1;
            if let Some(prev_node) = old_tail.borrow_mut().prev.take() {
                prev_node.borrow_mut().next = None;
                self.tail = Some(prev_node);
            } else {
                self.head = None;
            }

            let item = {
                let mut node = old_tail.borrow_mut();
                let item = std::mem::replace(&mut node.item, Default::default());
                Some(item)
            };

            return item;*/

            self.tail.take().map(|prev_tail| {
                self.size -= 1;
                match prev_tail.borrow_mut().prev.take() {
                    Some(node) => {
                        node.borrow_mut().next = None;
                        self.tail = Some(node);
                    }
                    None => {
                        self.head.take();
                    }
                }
                Rc::try_unwrap(prev_tail).ok().unwrap().into_inner().item
            })
        }
        /// Rimuove e restituisce l'elemento dall'inizio della lista.
        ///
        /// **Input:** Nessuno.
        ///
        /// **Valore di ritorno:** L'elemento rimosso, se presente.
        pub fn pop_front(&mut self) -> Option<T> { //Togli dalla testa un elemento
            /*if self.is_empty() {
                return None;
            }

            let old_head = self.head.take()?;
            self.size -= 1;
            if let Some(next_node) = old_head.borrow_mut().next.take() {
                next_node.borrow_mut().prev = None;
                self.head = Some(next_node);
            } else {
                self.tail = None;
            }

            let item = {
                let mut node = old_head.borrow_mut();
                let item = std::mem::replace(&mut node.item, Default::default());
                Some(item)
            };

            return item;*/

            self.head.take().map(|prev_head| {
                self.size -= 1;
                match prev_head.borrow_mut().next.take() {
                    Some(node) => {
                        node.borrow_mut().prev = None;
                        self.head = Some(node);
                    }
                    None => {
                        self.tail.take();
                    }
                }
                Rc::try_unwrap(prev_head).ok().unwrap().into_inner().item
            })
        }
        /// Restituisce l'ennesimo elemento dalla lista, partendo dall'inizio se `n` è positivo,
        /// o dalla fine se `n` è negativo.
        ///
        /// **Input:**
        /// - `n`: L'indice dell'elemento da restituire. Se positivo, parte dall'inizio della lista;
        ///        se negativo, parte dalla fine.
        ///
        /// **Valore di ritorno:** L'elemento alla posizione specificata, se presente.
        // Se n e' positivo ritornate l'ennesimo elemento dall'inizio
        //della lista mentre se e' negativo lo ritornate dalla coda
        //(-1 e' il primo elemento dalla coda)
        pub fn get(&mut self, n: i32) -> Option<T> {
            // Controlla se la lista è vuota
            if self.is_empty() {
                return None;
            }

            // Determina se l'indice è positivo o negativo
            let mut index = if n >= 0 { n } else { -n - 1 };
            let mut current_node = if n >= 0 {
                self.head.clone()
            } else {
                self.tail.clone()
            };

            // Scorre la lista in avanti o all'indietro in base all'indice
            while let Some(node) = current_node {
                if index == 0 {
                    // Se l'indice è zero, restituisci il nodo corrente
                    let mut node_taken = node.take();
                    // Rimuovi il nodo dalla lista
                    if let Some(next_node) = node_taken.next.clone() {
                        if let Some(prev_node) = node_taken.prev.clone() {
                            next_node.borrow_mut().prev = Some(prev_node.clone());
                            prev_node.borrow_mut().next = Some(next_node.clone());
                        } else { // Se non esiste un nodo precedente
                            next_node.borrow_mut().prev = None;
                            self.head = Some(next_node.clone());
                        }
                    } else {
                        if let Some(prev_node) = node_taken.prev.clone() {
                            prev_node.borrow_mut().next = None;
                            self.tail = Some(prev_node.clone());
                        }
                    }
                    self.size -= 1;
                    // Estrai e restituisci il valore contenuto nel nodo
                    return Some(std::mem::replace(&mut node_taken.item, Default::default()));
                }

                // Avanza o retrocede nel percorso della lista in base all'indice
                current_node = if n >= 0 { node.borrow().next.clone() } else { node.borrow().prev.clone() };
                index -= 1;
            }

            // Se l'indice è fuori dai limiti della lista, restituisci None
            None
        }


    }
}


