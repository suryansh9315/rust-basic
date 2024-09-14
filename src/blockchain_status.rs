#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Blockbook {
    pub coin: String,
    // pub inSync: bool,
    // pub decimals: i64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Backend {
    pub chain: String
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainStatus {
    pub blockbook: Blockbook,
    pub backend: Backend
}