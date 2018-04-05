use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, HashMap};
use std::time::Duration;


const FULL_MAJOR: u32 = 0xFFFFFFFF;
const HALF_MAJOR: u32 = FULL_MAJOR/2;
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct MajorId(u32); // cyclic!
impl PartialOrd for MajorId {
	fn partial_cmp(&self, other: &MajorId) -> Option<Ordering> {
		// if angle between us is OBTUSE, I assume we are one revolution apart
		// if the angle between us is ACUTE, I assume we are on the same revolution
		if self.0 == other.0 { return Some(Ordering::Equal) }
		let i_lead_raw = self.0 > other.0;
		let acute = if i_lead_raw {
			self.0 - other.0 < HALF_MAJOR
		} else {
			other.0 - self.0 < HALF_MAJOR
		};
		if i_lead_raw == acute {
			Some(Ordering::Greater)
		} else {
			Some(Ordering::Less)
		}
    }
}
impl MajorId {
	pub fn abs_modulo_difference(self, other: Self) -> u32 {
		if self.0 == other.0 { return 0 }
		let i_lead_raw = self.0 > other.0;
		let acute = if i_lead_raw {
			self.0 - other.0 < HALF_MAJOR
		} else {
			other.0 - self.0 < HALF_MAJOR
		};
		if acute {
			if i_lead_raw {
				self.0 - other.0
			} else {
				other.0 - self.0
			}
		} else { //obtuse
			if i_lead_raw {
				self.0 + (FULL_MAJOR - other.0)
			} else {
				other.0 + (FULL_MAJOR - self.0)
			}
		}
	}
}


pub fn test() {
	let a = MajorId(0x5);
	let b = MajorId(0xFFFFFFFF);

	println!("{:?} {:?} {:?}", &a, a.partial_cmp(&b), &b);

}


pub struct MinorId(u8); // not cyclic!





// ////////////////////////////

struct WrappedMessage<M> where M: Message {
	maj: MajorId,
	min: MinorId,
	min_count: u8,
	payload: M,
}

type RawHeapUnit = (MajorId, MinorId);

// small wrapper struct to inverse the ordedring such that the Max-Heap behaves like a Min-heap
struct MissingRange {
	maj: MajorId,
	min_f_inc: MinorId,
	min_t_inc: MinorId,
}
impl PartialOrd for MissingRange {
	// REVERSED to leverage the max heap and have it act as a min heap
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		match self.maj.partial_cmp(other).unwrap() {
			Ordering::Less => Ordering::More,
			Ordering::More => Ordering::Less,
			Ordering::Equal => self.cmp(other).reverse() //more readable
		}
	}
}


struct WaitingMessage<M> where M: Message {
	maj: MajorId,
	min: MinorId,
	m: M,
}
impl PartialOrd for WaitingMessage {
	// REVERSED to leverage the max heap and have it act as a min heap
	fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
		(self.maj, self.min).cmp(
			(other.maj, other.min)
		).reverse()
	}
}

pub trait Message {}

pub struct Channel<M> where M:Message {
	//outgoing
	current_major: MajorId,

	//incoming

	// X where all messages with maj <= X have arrived 
	largest_maj_ready: MajorId, 
	// maj_ids > last_completed for which 1+ messages have been received
	size_info: HashMap<MajorId, u16>,
	// ids of messages that we EXPECT but have not yet received
	expecting_ranges: BinaryHeap<MissingRange>,
	// messages that have arrived but haven't been read yet
	waiting_messages: BinaryHeap<WaitingMessage>,
}

impl<M> Channel<M> where M: Message {

	fn recv_something(&mut self) {
		let w_m: WrappedMessage<M> = read(...);
		if w_m.maj <= self.largest_maj_ready {
			// message is a duplicate of something I have already!
			return;
		}
		if w_m.maj.abs_modulo_difference(self.largest_maj_ready) > 15 {
			// message is for too far in the future! 
			return;
		}
		if self.size_info.contains(&w_m.maj) {
			// I've already gotten a message from this group
			if "I am expecting it" {
				self.waiting_messages.insert(w_m);
			}
			
		} else {
			// I've never gotten a message from this group
		}
	}

	fn raw_recv(&mut self) -> Option<M> {
		/*
		### Step1: Try read a ready message
		A message is `ready` iff:
		- It's maj is <= last_completed
		*/



		// let w_m: WrappedMessage<M> = read(...);
		// if w_m.maj <= self.last_completed {
		// 	/*
		// 	invalid message!
		// 	I've moved on from this major_id
		// 	drop the message
		// 	*/
		// 	return;
		// }
		// if w_m.maj.abs_modulo_difference(self.last_completed) > 16 {
		// 	/*
		// 	Too many major messages inbetween (probably!)
		// 	drop this one. its too far in the future
		// 	*/
		// 	return;
		// }
		// if !self.size_info.contains(&w_m.maj) {
		// 	// first time seeing this major id!
		// 	self.size_info.insert(w_m.maj, w_m.size);
		// 	// lets mark all the forthcoming messages!
		// 	for q in 0..w_m.size {
		// 		let m = MinorId(q);
		// 		if m == w_m.minor_id {
		// 			continue;
		// 		}
		// 		expecting_messages.insert(HeapUnit((w_m.maj, MinorId(q))));
		// 	}
		// } else {
		// 	expecting_messages.remove(HeapUnit((w_m.maj, w_m.min)));
		// }


		// finally, try to yield a legal message
		let mut can_yield = false;
		if let Some(waiting) = self.waiting_messages.peek() {
			if self.largest_maj_ready >= waiting.peek().maj {
				can_yield = true;
			}
		}
		if can_yield {
			Some(self.waiting_messages.pop().unwrap().m)
		} else {
			None
		}
	}

	fn raw_send(&mut self, minor_id: MinorId, m: M) {
		let major_id = self.current_major;
		// send!
	}

	pub fn new() -> Channel<M> {
		Channel {
			next_clock: C::first(),
			incoming: vec![],
		}
	}

	fn incriment_clock(&mut self) {
		self.next_clock = self.next_clock.next();
	}

	pub fn solo_send(&mut self, m: M) {
		self.incriment_clock();
		self.raw_send(m);
	}

	pub fn group_send(&mut self, m: Vec<M>) {

	}

	// pub fn group_handle<'a>(&'a mut self) -> ChannelHandle<'a,M> {
	// 	self.incriment_clock();
	// 	ChannelHandle {
	// 		channel: self,
	// 		next_minor: MinorId(0),
	// 	}
	// }

	// fn group_being_dropped(&mut self, handle: &ChannelHandle<M>) {
	// 	//TODO send a control message notifying the listener that
	// }
}




// pub struct ChannelHandle<'a, M>
// where
// 	M: Message + 'a
// {
// 	channel: &'a mut Channel<M>,
// 	next_minor: MinorId,
// }

// impl<'a, M> ChannelHandle<'a, M>
// where
// 	M: Message + 'a
// {
// 	pub fn group_send(&mut self, m: M) {
// 		if self.capacity_left == 0x00 {panic!("exhausted group capacity!")}
// 	}

// 	pub fn sent_so_far(&self) -> u8 {
// 		0xFF - self.capacity_left
// 	}

// 	pub fn capacity_left(&self) -> u8 {
// 		self.capacity_left
// 	}
// }
// impl<'a, M> Drop for ChannelHandle<'a, M>
// where
// 	M: Message + 'a
// {
//     fn drop(&mut self) {
//     	self.channel.group_being_dropped(self);
//     }
// }