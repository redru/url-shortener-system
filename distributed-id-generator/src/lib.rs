mod clock;

use clock::{Clock, SystemClock};
use std::sync::{Arc, Mutex};
use std::time::Duration;

const TIMESTAMP_SHIFT: u8 = 22;
const MACHINE_ID_SHIFT: u8 = 17;
const DATACENTER_ID_SHIFT: u8 = 12;

const TIMESTAMP_BITMASK: u64 = 0b1111111111_1111111111_1111111111_1111111111_1;
const MACHINE_ID_BITMASK: u64 = 0b11111;
const DATACENTER_ID_BITMASK: u64 = 0b11111;
const SEQUENCE_BITMASK: u64 = 0b11111_11111_11;

pub struct IdGenerator {
    pub machine_id: u64,
    pub datacenter_id: u64,
    sequence_value: u64,
    last_refresh: Duration,
    clock: Arc<Mutex<dyn Clock>>,
}

impl IdGenerator {
    pub fn new(machine_id: u64, datacenter_id: u64) -> Self {
        Self {
            machine_id,
            datacenter_id,
            sequence_value: 0,
            last_refresh: Duration::ZERO,
            clock: Arc::new(Mutex::new(SystemClock::new())),
        }
    }

    #[cfg(test)]
    fn with_clock(machine_id: u64, datacenter_id: u64, clock: Arc<Mutex<dyn Clock>>) -> Self {
        Self {
            machine_id,
            datacenter_id,
            sequence_value: 0,
            last_refresh: Duration::ZERO,
            clock,
        }
    }

    pub fn generate_id(&mut self) -> u64 {
        let mut id = 0u64;

        let now = self.clock.lock().unwrap().now();

        let sequence = if now.as_secs() > self.last_refresh.as_secs() + 1 {
            self.last_refresh = now;
            self.sequence_value = 0;
            self.sequence_value
        } else {
            self.sequence_value
        };

        self.sequence_value += 1;

        id |= (now.as_secs() & TIMESTAMP_BITMASK) << TIMESTAMP_SHIFT;
        id |= (self.machine_id & MACHINE_ID_BITMASK) << MACHINE_ID_SHIFT;
        id |= (self.datacenter_id & DATACENTER_ID_BITMASK) << DATACENTER_ID_SHIFT;
        id |= sequence;

        id
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Id {
    sign: u8,
    timestamp: Duration,
    machine_id: u8,
    datacenter_id: u8,
    sequence: u16,
}

impl Id {
    pub fn new() -> Self {
        Default::default()
    }
}

impl Default for Id {
    fn default() -> Self {
        Self {
            sign: 0,
            timestamp: Duration::ZERO,
            machine_id: 0,
            datacenter_id: 0,
            sequence: 0,
        }
    }
}

impl From<u64> for Id {
    fn from(value: u64) -> Self {
        let mut id = Id::new();
        id.sign = ((value >> 63) & 1) as u8;
        id.timestamp = Duration::from_secs((value >> TIMESTAMP_SHIFT) & TIMESTAMP_BITMASK);
        id.machine_id = ((value >> MACHINE_ID_SHIFT) & MACHINE_ID_BITMASK) as u8;
        id.datacenter_id = ((value >> DATACENTER_ID_SHIFT) & DATACENTER_ID_BITMASK) as u8;
        id.sequence = (value & SEQUENCE_BITMASK) as u16;
        id
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::clock::MockClock;

    fn to_parts(id: u64) -> (u64, u64, u64, u64) {
        let timestamp = (id >> TIMESTAMP_SHIFT) & TIMESTAMP_BITMASK;
        let machine_id = (id >> MACHINE_ID_SHIFT) & MACHINE_ID_BITMASK;
        let datacenter_id = (id >> DATACENTER_ID_SHIFT) & DATACENTER_ID_BITMASK;
        let sequence = id & SEQUENCE_BITMASK;
        (timestamp, machine_id, datacenter_id, sequence)
    }

    #[test]
    fn test_id_generator_new() {
        let generator = IdGenerator::new(1, 2);
        assert_eq!(generator.machine_id, 1);
        assert_eq!(generator.datacenter_id, 2);
    }

    #[test]
    fn test_generate_id() {
        let mut generator = IdGenerator::new(1, 2);
        let id = generator.generate_id();

        let (timestamp, machine_id, datacenter_id, _) = to_parts(id);

        assert!(timestamp > 0); // Timestamp should be non-zero
        assert_eq!(machine_id, 1); // Should match machine_id
        assert_eq!(datacenter_id, 2); // Should match datacenter_id
    }

    #[test]
    fn test_id_from_u64() {
        let mut generator = IdGenerator::new(1, 2);
        let original_id = generator.generate_id();
        let id = Id::from(original_id);

        let (timestamp, machine_id, datacenter_id, _) = to_parts(original_id);

        assert_eq!(id.timestamp.as_secs(), timestamp);
        assert_eq!(id.machine_id as u64, machine_id);
        assert_eq!(id.datacenter_id as u64, datacenter_id);
    }

    #[test]
    fn test_sequence_increment_and_reset() {
        let mock_clock = Arc::new(Mutex::new(MockClock::new(Duration::ZERO)));

        let mut generator = IdGenerator::with_clock(1, 2, mock_clock.clone());

        let first_id = generator.generate_id();
        let (_, _, _, first_sequence) = to_parts(first_id);

        let second_id = generator.generate_id();
        let (_, _, _, second_sequence) = to_parts(second_id);
        assert_eq!(second_sequence, first_sequence + 1);

        // Advance the clock
        mock_clock.lock().unwrap().advance(Duration::from_secs(3));

        let third_id = generator.generate_id();
        let (_, _, _, third_sequence) = to_parts(third_id);
        assert_eq!(third_sequence, 0);
    }
}
