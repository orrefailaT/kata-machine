struct Node<T> {
    value: T,
    next: Option<Box<Node<T>>>,
}

pub struct SinglyLinkedList<T> {
    length: usize,
    head: Option<Box<Node<T>>>,
}

impl<T> SinglyLinkedList<T> {
    pub fn length(self) -> usize {
        self.length
    }

    pub fn new() -> Self {
        SinglyLinkedList {
            length: 0,
            head: None,
        }
    }

    pub fn prepend(&mut self, item: T) {
        self.head = Some(Box::new(Node {
            value: item,
            next: self.head.take(),
        }));

        self.length += 1;
    }

    pub fn insert(&mut self, item: T, idx: usize) -> Result<(), &str> {
        if idx > self.length {
            return Err("Index out of bounds")
        }

        let mut node = self.head.as_mut();
        for _ in 0..idx {
            node = node.unwrap().next.as_mut();
        }
        
        if let Some(mut n) = node {
            n.next = Some(Box::new(Node {
                value: item,
                next: n.next.take(),
            }));
        } else {
            self.head = Some(Box::new(Node {
                value: item,
                next: None,
            }));
        }
        self.length += 1;
        Ok(())
    }


    pub fn get(&self, idx: usize) -> Result<T, &str> {
        if idx > self.length {
            return Err("Index out of bounds")
        }

        let node = self.head;
        for _ in 0..idx {
            let node = node.unwrap().next;
        }

        Ok(node.unwrap().value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list() {
        let mut list: SinglyLinkedList<i32> = SinglyLinkedList::new();

        list.prepend(5);
        list.prepend(3);

        assert_eq!(list.length(), 2);
    }
}
