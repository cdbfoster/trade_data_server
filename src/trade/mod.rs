// This file is part of trade-data.
//
// trade-data is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// trade-data is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with trade-data.  If not, see <http://www.gnu.org/licenses/>.

use value::Value;

pub enum OrderSide {
    Ask,
    Bid,
}

pub struct Trade<T, U> where T: Value, U: Value {
    pub amount: T,
    pub price: U,
    pub side: OrderSide,
}

impl<T, U> Trade<T, U> where T: Value, U: Value {
    pub fn new(amount: T, price: U, side: OrderSide) -> Self {
        Self {
            amount: amount,
            price: price,
            side: side,
        }
    }
}

mod storable;
