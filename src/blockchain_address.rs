#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockchainAddress {
    pub address: String,
    pub txids: Vec<String>
}