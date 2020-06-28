#[derive(Debug)]
pub enum SendError {
    ReceiverDisconnected,
    NoResponseExpected,
}

#[derive(Debug)]
pub enum ReceiveError {
    AllSendersDisconnected,
}
