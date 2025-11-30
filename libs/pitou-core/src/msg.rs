use crate::PitouFile;
use serde::{Deserialize, Serialize};
use std::{collections::LinkedList, time::Duration};

pub enum SearchMsg {
    Active(LinkedList<PitouFile>),
    Terminated(LinkedList<PitouFile>),
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TransferState {
    Initializing(u64),
    Active(TransferSize),
    Terminated(TransferSize),
}

impl TransferState {
    pub const fn is_terminted(&self) -> bool {
        matches!(self, Self::Terminated(_))
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub enum TransferMsg {
    Copy {
        id: TransferSessionID,
        state: TransferState,
        time_elapsed: Duration,
    },
    Move {
        id: TransferSessionID,
        state: TransferState,
        time_elapsed: Duration,
    },
}


impl TransferMsg {
    pub fn details(self) -> (TransferState, Duration) {
        match self {
            TransferMsg::Copy {
                id: _,
                state,
                time_elapsed,
            } => (state, time_elapsed),
            TransferMsg::Move {
                id: _,
                state,
                time_elapsed,
            } => (state, time_elapsed),
        }
    }

    pub fn id(&self) -> TransferSessionID {
        match self {
            Self::Copy { id, state: _, time_elapsed: _ } => *id,
            Self::Move { id, state: _, time_elapsed: _ } => *id,
        }
    }

    pub fn is_terminated(&self) -> bool {
        match self {
            TransferMsg::Copy { id: _, state, time_elapsed: _ } => state.is_terminted(),
            TransferMsg::Move { id: _, state, time_elapsed: _ } => state.is_terminted(),
        }
    }
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct TransferSize {
    pub total: u64,
    pub current: u64,
}

#[derive(Clone, Copy, Serialize, Deserialize, Hash, Eq, PartialEq)]
pub struct TransferSessionID {
    pub idx: i64,
    pub parity: i64,
}
