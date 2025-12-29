use dbn::{FlagSet, UNDEF_PRICE};
use hashbrown::HashMap;
use std::collections::{BTreeMap, VecDeque};

use crate::stream::hotloop::Mbo;

#[derive(Debug, Default)]
pub struct Book {
    /// order_id -> (side, price)
    pub orders_by_id: HashMap<u64, (i8, i64)>,
    pub offers: BTreeMap<i64, Level>,
    pub bids: BTreeMap<i64, Level>,
}

type Level = VecDeque<LobMbo>;

impl Book {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply(&mut self, mbo: LobMbo) {
        match mbo.action {
            val if val == b'M' as i8 => self.modify(mbo),
            val if val == b'C' as i8 => self.cancel(mbo),
            val if val == b'A' as i8 => self.add(mbo),
            val if val == b'R' as i8 => self.clear(),
            _ => {},
        }
    }

    #[inline]
    pub fn clear(&mut self) {
        self.orders_by_id.clear();
        self.offers.clear();
        self.bids.clear();
    }

    #[inline]
    fn add(&mut self, mbo: LobMbo) {
        let price = mbo.price;
        let side = mbo.side;
        if mbo.flags.is_tob() {
            let levels = self.side_levels_mut(side);
            levels.clear();
            // UNDEF_PRICE indicates the side's book should be cleared
            // and doesn't represent an order that should be added
            if price != UNDEF_PRICE {
                levels.insert(price, VecDeque::from([mbo]));
            }
        } else {
            if price == UNDEF_PRICE {
                return;
            };
            assert!(self.orders_by_id.insert(mbo.order_id, (side, price)).is_none());
            let level = self.get_or_insert_level(side, price);
            level.push_back(mbo);
        }
    }

    #[inline]
    fn modify(&mut self, mbo: LobMbo) {
        let order_id = mbo.order_id;
        let side = mbo.side;
        // If order not found, treat it as an add
        let Some((id_side, id_price)) = self.orders_by_id.get_mut(&order_id) else {
            return self.add(mbo);
        };
        let prev_side = *id_side;
        let prev_price = *id_price;
        // Update orders by ID
        *id_side = side;
        *id_price = mbo.price;
        // Update level order
        let level = self.level_mut(prev_side, prev_price);
        let order_idx = Self::find_order(level, order_id);
        let existing_order = level.get_mut(order_idx).unwrap();
        if prev_price == mbo.price && existing_order.size >= mbo.size {
            return;
        }
        if prev_price != mbo.price {
            let prev_level = level;
            Self::remove_order(prev_level, order_id);
            if prev_level.is_empty() {
                self.remove_level(side, prev_price);
            }
            let level = self.get_or_insert_level(side, mbo.price);
            level.push_back(mbo);
        } else {
            Self::remove_order(level, order_id);
            level.push_back(mbo);
        }
    }

    #[inline]
    fn cancel(&mut self, mbo: LobMbo) {
        let side = mbo.side;
        let level = self.level_mut(side, mbo.price);
        let order_idx = Self::find_order(level, mbo.order_id);
        let existing_order = level.get_mut(order_idx).unwrap();
        assert!(existing_order.size >= mbo.size);
        existing_order.size -= mbo.size;
        if existing_order.size == 0 {
            level.remove(order_idx).unwrap();
            if level.is_empty() {
                self.remove_level(side, mbo.price);
            }
            self.orders_by_id.remove(&mbo.order_id).unwrap();
        }
    }

    #[inline]
    fn side_levels_mut(&mut self, side: i8) -> &mut BTreeMap<i64, Level> {
        match side {
            val if val == b'A' as i8 => &mut self.offers,
            val if val == b'B' as i8 => &mut self.bids,
            _ => panic!("Invalid side None"),
        }
    }

    #[inline]
    fn get_or_insert_level(&mut self, side: i8, price: i64) -> &mut Level {
        let levels = self.side_levels_mut(side);
        levels.entry(price).or_default()
    }

    #[inline]
    fn level_mut(&mut self, side: i8, price: i64) -> &mut Level {
        let levels = self.side_levels_mut(side);
        levels.get_mut(&price).unwrap()
    }

    #[inline]
    fn find_order(level: &VecDeque<LobMbo>, order_id: u64) -> usize {
        level.iter().position(|order| order.order_id == order_id).unwrap()
    }

    #[inline]
    fn remove_order(level: &mut VecDeque<LobMbo>, order_id: u64) {
        let index = Self::find_order(level, order_id);
        level.remove(index).unwrap();
    }

    #[inline]
    fn remove_level(&mut self, side: i8, price: i64) {
        self.side_levels_mut(side).remove(&price).unwrap();
    }
}

#[derive(Debug, Clone)]
pub struct LobMbo {
    pub ts_recv: u64,
    pub ts_event: u64,
    pub action: i8,
    pub side: i8,
    pub price: i64,
    pub size: u32,
    pub order_id: u64,
    pub flags: FlagSet,
}

impl From<&Mbo> for LobMbo {
    #[inline]
    fn from(mbo: &Mbo) -> Self {
        Self {
            ts_recv: mbo.ts_recv,
            ts_event: mbo.ts_event,
            action: mbo.action,
            side: mbo.side,
            price: mbo.price,
            size: mbo.size,
            order_id: mbo.order_id,
            flags: mbo.flags,
        }
    }
}
