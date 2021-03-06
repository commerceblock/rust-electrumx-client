#[derive(Debug, Deserialize)]
pub struct GetBlockHeadersResponse {
    pub count: usize,
    pub hex:   String,
    pub max:   usize,
}

#[derive(Debug, Deserialize)]
pub struct GetTipResponse {
    pub height: usize,
    pub hex:   String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetBalanceResponse {
    pub confirmed:   u64,
    pub unconfirmed: u64,
}

#[derive(Debug, Deserialize)]
pub struct GetHistoryResponse {
    pub height:  usize,
    pub tx_hash: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetListUnspentResponse {
    pub height:  usize,
    pub tx_hash: String,
    pub tx_pos:  usize,
    pub value:   usize,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTransactionConfStatus {
    pub in_active_chain: Option<bool>,
    pub confirmations: Option<u32>,
    pub blocktime: Option<usize>,
}
