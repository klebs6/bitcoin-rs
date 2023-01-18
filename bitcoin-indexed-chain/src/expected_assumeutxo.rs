crate::ix!();

/**
  | Return the expected assumeutxo value
  | for a given height, if one exists.
  | 
  | -----------
  | @param[in] height
  | 
  | Get the assumeutxo value for this height.
  | 
  | -----------
  | @return
  | 
  | empty if no assumeutxo configuration
  | exists for the given height.
  |
  */
pub fn expected_assumeutxo(
    height:      i32,
    chainparams: &ChainParams) -> Arc<AssumeUtxoData> {
    
    todo!();
        /*
            const MapAssumeutxo& valid_assumeutxos_map = chainparams.Assumeutxo();
        const auto assumeutxo_found = valid_assumeutxos_map.find(height);

        if (assumeutxo_found != valid_assumeutxos_map.end()) {
            return &assumeutxo_found->second;
        }
        return nullptr;
        */
}
