

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
pub struct NewTxFlowConfig {
    pub count_per_batch: usize,
    pub buffer_duration: u32, //in unit of ns
}
