struct SubAccount {
    info: SubInfos,
    start: Starts,
    incoming: Incomings,
    outgoing: Outgoings,
    refund: Refunds,
    balance: Balances,
}

type SubInfos = Vec<SubInfo>;
type Starts = Vec<Start>;
type Incomings = Vec<Incoming>;
type Outgoings = Vec<Outgoing>;
type Balances = Vec<Balance>;
type Refunds = Vec<Refund>;

struct SubInfo;
struct Start;
struct Incoming;
struct Outgoing;
struct Balance;
struct Refund;
