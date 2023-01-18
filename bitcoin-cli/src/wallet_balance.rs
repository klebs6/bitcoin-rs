crate::ix!();

/**
  | GetWalletBalances calls listwallets;
  | if more than one wallet is loaded, it
  | then fetches mine.trusted balances
  | for each loaded wallet and pushes them
  | to `result`.
  | 
  | -----------
  | @param result
  | 
  | Reference to UniValue object the wallet
  | names and balances are pushed to.
  |
  */
pub fn get_wallet_balances(result: &mut UniValue)  {
    
    let mut rh: Box<dyn BaseRequestHandler> = Box::new(DefaultRequestHandler::default());

    let listwallets: UniValue = connect_and_callrpc(
        &mut rh,
        "listwallets",
        &vec![],
        None
    ).unwrap();

    if !find_value(&listwallets,"error").is_null() {
        return;
    }

    let wallets: &UniValue = find_value(&listwallets,"result");

    if wallets.size() <= 1 {
        return;
    }

    let mut balances: UniValue = UniValue::new(uni_value::VType::VOBJ, None);

    for wallet in wallets.get_values().unwrap().iter() {

        let wallet_name: String = wallet.get_str().to_string();

        let getbalances: UniValue = connect_and_callrpc(
            &mut rh,
            "getbalances",
            &vec![],
            Some(&wallet_name)
        ).unwrap();

        let mut balance: &UniValue = &find_value(&getbalances,"result")["mine"]["trusted"];

        balances.pushkv(&wallet_name, balance);
    }

    result.pushkv("balances", &balances);
}
