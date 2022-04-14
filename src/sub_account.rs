use std::cell::{Ref, RefCell};

use crate::{
    balance::Balances, incoming::Incomings, outgoing::Outgoings, refund::Refunds, start::Starts,
};

pub struct SubAccount {
    info: RefCell<SubInfos>,
    start: RefCell<Starts>,
    incoming: RefCell<Incomings>,
    outgoing: RefCell<Outgoings>,
    refund: RefCell<Refunds>,
    balance: RefCell<Balances>,
}

impl SubAccount {
    pub fn new() -> Self {
        Self {
            info: RefCell::new(SubInfos::default()),
            start: RefCell::new(Starts::default()),
            incoming: RefCell::new(Incomings::default()),
            outgoing: RefCell::new(Outgoings::default()),
            refund: RefCell::new(Refunds::default()),
            balance: RefCell::new(Balances::default()),
        }
    }
    pub fn get_info(&self) -> Ref<SubInfos> {
        self.info.borrow()
    }
    pub fn get_start(&self) -> Ref<Starts> {
        self.start.borrow()
    }
    pub fn get_incoming(&self) -> Ref<Incomings> {
        self.incoming.borrow()
    }
    pub fn get_outgoing(&self) -> Ref<Outgoings> {
        self.outgoing.borrow()
    }
    pub fn get_refund(&self) -> Ref<Refunds> {
        self.refund.borrow()
    }
    pub fn get_balance(&self) -> Ref<Balances> {
        self.balance.borrow()
    }
}
#[derive(Default)]
pub struct SubInfos(Vec<SubInfo>);

pub struct SubInfo;
