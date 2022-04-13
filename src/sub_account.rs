use crate::{
    balance::Balances, incoming::Incomings, outgoing::Outgoings, refund::Refunds, start::Starts,
};

pub struct SubAccount<'sa> {
    info: &'sa SubInfos,
    start: &'sa Starts,
    incoming: &'sa Incomings,
    outgoing: &'sa Outgoings,
    refund: &'sa Refunds,
    balance: &'sa Balances,
}

impl<'sa, 'r, 'b, 'o, 'i, 's, 'si> SubAccount<'sa>
where
    'r: 'sa,
    'b: 'sa,
    'o: 'sa,
    'i: 'sa,
    's: 'sa,
    'si: 'sa,
{
    pub fn new(
        info: &'si SubInfos,
        start: &'s Starts,
        incoming: &'i Incomings,
        outgoing: &'o Outgoings,
        refund: &'r Refunds,
        balance: &'b Balances,
    ) -> Self {
        Self {
            info,
            start,
            incoming,
            outgoing,
            refund,
            balance,
        }
    }
    pub fn get_info(&self) -> &'sa SubInfos {
        self.info
    }
    pub fn get_start(&self) -> &'sa Starts {
        self.start
    }
    pub fn get_incoming(&self) -> &'sa Incomings {
        self.incoming
    }
    pub fn get_outgoing(&self) -> &'sa Outgoings {
        self.outgoing
    }
    pub fn get_refund(&self) -> &'sa Refunds {
        self.refund
    }
    pub fn get_balance(&self) -> &'sa Balances {
        self.balance
    }
}

pub type SubInfos = Vec<SubInfo>;

pub struct SubInfo;
