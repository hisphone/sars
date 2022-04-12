use crate::{
    balance::Balances, incoming::Incomings, outgoing::Outgoings, refund::Refunds, start::Starts,
};

pub struct SubAccount {
    info: SubInfos,
    start: Starts,
    incoming: Incomings,
    outgoing: Outgoings,
    refund: Refunds,
    balance: Balances,
}

pub type SubInfos = Vec<SubInfo>;

pub struct SubInfo;
