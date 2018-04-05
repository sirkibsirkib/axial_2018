
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

pub struct MinorId(u8); // not cyclic!


////////////////////////////////////////////////////////////

type Range = (MinorId, MinorId); //both sides inclusive

struct SortedRanges {
	ranges: Vec<Range>,
}
impl SortedRanges {
	pub fn new(init_range: Range) -> Self {
		SortedRanges { ranges: vec![init_range] }
	}

	fn bin_search(&self, x: MinorId, a: usize, b: usize) -> Option<usize> {
		if a >= b {
			return None;
		}
		let mid = (a+b)/2;
		let z = self.ranges[mid];
		if x < z.0 { // left
			self.bin_search(x, a, z.0-1)
		} else if x > z.1 { //right
			self.bin_search(x, z.1+1, b)
		} else {
			Some(mid)
		}
	}
	pub fn remove(x: MinorId) -> bool {
		if let Some(i) = self.bin_search(x, 0, self.ranges.len()) {
			let (l, r) = self.ranges.remove(i);
			if x < r {
				self.ranges.insert((MinorId(x.0+1), r));
			}
			if x > l {
				self.ranges.insert((l, MinorId(x.0-1)));
			}
		} else {
			None
		}
	}
	#[inline]
	pub fn is_empty(&self) -> bool {
		self.ranges.is_empty()
	}
	pub fn contains(x: MinorId) -> bool {
		self.bin_search(x, 0, self.ranges.len()).is_some()
	}
}


struct ExpectingRanges {
	// map of vectors
	// each vector is a SORTED 2-tuple with from-to ranges
	range_vectors: HashMap<MajorId, SortedRanges>,
}

impl ExpectingRanges {
	pub fn define_whole_range(&mut self, maj: MajorId,  incl_bounds: (MinorId, MinorId)) {
		self.range_vectors.insert(maj, SortedRanges::new(incl_bounds));
	}

	pub fn remove_expecting(&mut self, maj: MajorId, min: MinorId) -> bool {
		let mut del = false;
		{
			let r = self.range_vectors.get_mut(&maj).unwrap() {
			assert!(r.remove(min));
		}
		if del {
			self.range_vectors.remove(&maj);
		}
	}

	pub fn expecting(&self, maj: MajorId, min: MinorId) -> bool {
		if let Some(vector) = self.ranges.get_mut(&maj) {
			vector.contains(min)
		} else {
			false
		}
	}
}


struct MessageManager<M> where M: Message {
	// X where all messages with maj <= X have arrived 
	largest_maj_ready: MajorId, 
	// maj_ids > last_completed for which 1+ messages have been received
	size_info: HashMap<MajorId, u16>,
	// ids of messages that we EXPECT but have not yet received
	expecting_ranges: ExpectingRanges,
	// messages that have arrived but haven't been read yet
	waiting_messages: BinaryHeap<WaitingMessage>,
}

impl<M> MessageManager<M> where M: Message {
	pub fn give_next(&mut self) -> Option<M> {

	}

	pub fn got_already(&mut self, maj: MajorId, min: MinorId) -> bool {

	}

	pub fn remove_from_expecting(&mut self, maj: MajorId, min: MinorId) {

	}

	pub fn accept_incoming(&mut self, m: M) {
		let w_m: WrappedMessage<M> = read(self. ...);

		if w_m.maj <= self.largest_maj_ready {
			// message is a duplicate of something I have already!
			return;
		}
		if w_m.maj.abs_modulo_difference(self.largest_maj_ready) > 15 {
			// message is for too far in the future! 
			return;
		}
		if self.size_info.contains_key(& w_m.maj) {
			// this is from a known group
			if ! self.got_already() {
				self.remove_from_expecting(w_m.maj, w_m.min);
				self.waiting_messages.insert(w_m);
			}
		} else {
			// this is not from a known group
		}
	}
}