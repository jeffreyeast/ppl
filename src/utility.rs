// Holds useful routines

use core::fmt;
use std::sync::{Condvar, Arc, Mutex};
use num_traits::Num;



pub fn convert_escape_sequences(input_string: &str) -> String {
    input_string.trim_end().replace('\r', "\\r").replace('\n', "\\n").replace('"', "\"\"")
}


//  The Set struct is used to efficiently hold ranges of numbers

#[derive(Clone)]
pub struct Set<T> where T: Num + Ord + Copy + fmt::Display {
    members: Vec<SetMember<T>>,
}

impl<T: Num + Ord + Copy + fmt::Display> Set<T> {
    pub fn add(&mut self, new_member: T) {
        let mut low = 1;
        let mut high = self.members.len();

        while high >= low {
            let target = (low + high) / 2;

            if new_member + T::one() == self.members[target - 1].get_first() {
                self.members[target - 1].set_first(new_member);
                return;
            }
            if new_member - T::one() == self.members[target - 1].get_last() {
                self.members[target - 1].set_last(new_member);
                return;
            }

            if new_member < self.members[target - 1].get_first() {
                high = target - 1;
            } else {
                low = target + 1;
            }
        }

        let insertion_point = std::cmp::min(low, high);

        if insertion_point < self.members.len() {
            assert!(new_member < self.members[insertion_point].get_first());
        }
        if insertion_point > 0 {
            assert!(new_member > self.members[insertion_point - 1].get_last());
        }

        self.members.insert(insertion_point, SetMember::new(new_member));
    }

    pub fn as_members(&self) -> &Vec<SetMember<T>> {
        &self.members
    }

    pub fn clear(&mut self) {
        self.members.clear();
    }

    pub fn contains(&self, test_value: T) -> bool {
        let mut low = 1;
        let mut high = self.members.len();

        while high >= low {
            let target = (low + high) / 2;

            if test_value >= self.members[target - 1].get_first() && test_value <= self.members[target - 1].get_last() {
                return true;
            }

            if test_value < self.members[target - 1].get_first() {
                high = target - 1;
            } else {
                low = target + 1;
            }
        }
        false
    }

    pub fn is_empty(&self) -> bool {
        self.members.len() == 0
    }

    pub fn iter(&self) -> SetIterator<'_,T> {
        SetIterator::new(self)
    }

    pub fn new() -> Set<T> {
        Set { members: Vec::new() }
    }

    pub fn new_with_elem(initial_member: T) -> Set<T> {
        Set { members: vec![SetMember::new(initial_member)] }
    }

    pub fn next(&self, token: &mut SetIterationToken<T>) -> Option<T> {
        match token.get_current_member_index() {
            Some(index) => {
                if index >= self.members.len() {
                    //println!("Set::next ({}) returns None", self);
                    return None;
                }
                if token.get_next_value().unwrap() > self.members[index].get_last() {
                    token.set_current_member_index(Some(index + 1));
                    if index + 1 >= self.members.len() {
                        //println!("Set::next ({}) returns None", self);
                        return None;
                    }
                    let current_value = self.members[index + 1].get_first();
                    token.set_next_value(Some(current_value + T::one()));
                    //println!("Set::next ({}) returns {}", self, current_value);
                    return Some(current_value);
                } else {
                    let current_value = token.get_next_value().unwrap();
                    token.set_next_value( Some(current_value + T::one()));
                    //println!("Set::next ({}) returns {}", self, current_value);
                    return Some(current_value);
                }
            },
            None => {
                token.set_current_member_index(Some(0));
                if self.members.len() > 0 {
                    let current_value = self.members[0].get_first();
                    token.set_next_value(Some(current_value + T::one()));
                    //println!("Set::next ({}) returns {}", self, current_value);
                    return Some(current_value);
                } else {
                    //println!("Set::next ({}) returns None", self);
                    return None;
                }
            },
        }
    }

    pub fn repeat(&self, token: &mut SetIterationToken<T>) {
        match token.get_current_member_index() {
            Some(mut index) => {
                if index >= self.members.len() {
                    index = self.members.len() - 1;
                    token.set_next_value(Some(self.members[index].get_last()));
                } else if token.get_next_value().unwrap() > self.members[index].get_first() {
                    token.set_next_value(Some(token.get_next_value().unwrap() - T::one()));
                } else if index > 0 {
                    token.set_current_member_index(Some(index - 1));
                    token.set_next_value(Some(self.members[index - 1].get_last()));
                } else {
                    panic!("internal error");
                }
            },
            None => panic!("internal error"),
        }
    }

}

impl<T: Num + Ord + Copy + fmt::Display> fmt::Display for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{{")?;
        let mut separator = "";
        for member in &self.members {
            write!(f, "{}{}", separator, member)?;
            separator = ", ";
        }
        write!(f, "}}")
    }
}

impl<T: Num + Ord + Copy + fmt::Display> fmt::Debug for Set<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}




#[derive(Clone)]
pub struct SetMember<T> where T: Num + Ord + Copy + fmt::Display {
    first: T,
    last: T,
}

impl<T: Num + Ord + Copy + fmt::Display> SetMember<T> {
    pub fn get_first(&self) -> T {
        self.first
    }

    pub fn get_last(&self) -> T {
        self.last
    }

    pub fn is_in(&self, test_value: T) -> bool {
        test_value >= self.first && test_value <= self.last
    }

    pub fn new(first: T) -> SetMember<T> {
        SetMember { first: first, last: first }
    }

    pub fn set_first(&mut self, new_first: T) {
        self.first = new_first;
    }

    pub fn set_last(&mut self, new_last: T) {
        self.last = new_last;
    }
}

impl<T: Num + Ord + Copy + fmt::Display> fmt::Display for SetMember<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}-{}", self.first, self.last)
    }
}

impl<T: Num + Ord + Copy + fmt::Display> fmt::Debug for SetMember<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}





pub struct SetIterator<'a, T: 'a + Num + Ord + Copy + fmt::Display> {
    set: &'a Set<T>,
    token: SetIterationToken<T>,
}

impl<'a, T: Num + Ord + Copy + fmt::Display> SetIterator<'a,T> {
    pub fn new(set: &'a Set<T>) -> SetIterator<'a,T> {
        SetIterator { set: set, token: SetIterationToken::new() }
    }

    pub fn repeat(&mut self) {
        self.set.repeat(&mut self.token);
    }
}

impl<'a, T: Num + Ord + Copy + fmt::Display> Iterator for SetIterator<'a,T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        self.set.next(&mut self.token)
    }
}



//  The Token structure is used to provide positional information that can be stored on the heap or stack, for
//  use with iteration

#[derive(Debug,Clone)]
pub struct SetIterationToken<T: Num + Ord + Copy + fmt::Display> {
    current_member_index: Option<usize>,
    next_value: Option<T>,
}

impl<T: Num + Ord + Copy + fmt::Display> SetIterationToken<T> {
    pub fn get_current_member_index(&self) -> Option<usize> {
        self.current_member_index
    }

    pub fn get_next_value(&self) -> Option<T> {
        self.next_value
    }

    pub fn new() -> SetIterationToken<T> {
        SetIterationToken { current_member_index: None, next_value: None }
    }

    pub fn set_current_member_index(&mut self, new_index: Option<usize>) {
        self.current_member_index = new_index;
    }

    pub fn set_next_value(&mut self, new_value: Option<T>) {
        self.next_value = new_value;
    }
}



//  The Event structure is used to synchronize two threads, so that one will wait until the other has performed some action

#[derive(Debug)]
pub struct Event {
    mutex: Mutex<bool>,
    cvar: Condvar,
}

impl Event {
    pub fn new() -> Arc<Self> {
        Arc::new(Event { mutex: Mutex::new(false), cvar: Condvar::new() })
    }

    pub fn reset(&self) {
        let mut done = self.mutex.lock().unwrap();
        *done = false;
    }

    pub fn signal(&self) {
        let mut done = self.mutex.lock().unwrap();
        *done = true;
        self.cvar.notify_one();
    }

    pub fn wait(&self) {
        let mut done = self.mutex.lock().unwrap();
        while !*done {
            done = self.cvar.wait(done).unwrap();
        }
    }
}