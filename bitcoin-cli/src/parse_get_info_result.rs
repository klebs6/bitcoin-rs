// ---------------- [ File: bitcoin-cli/src/parse_get_info_result.rs ]
crate::ix!();

/**
  | ParseGetInfoResult takes in -getinfo
  | result in UniValue object and parses
  | it into a user friendly UniValue string
  | to be printed on the console.
  | 
  | -----------
  | @param[out] result
  | 
  | Reference to UniValue result containing
  | the -getinfo output.
  |
  */
pub fn parse_get_info_result(result: &mut UniValue) -> Result<(),StdException> {

    if !find_value(result,"error").is_null() {
        return Ok(());
    }

    let mut RESET:   String = String::new();
    let mut GREEN:   String = String::new();
    let mut BLUE:    String = String::new();
    let mut YELLOW:  String = String::new();
    let mut MAGENTA: String = String::new();
    let mut CYAN:    String = String::new();

    let mut should_colorize: bool = false;

    #[cfg(not(WIN32))]
    if unsafe { isatty(libc::fileno(stdout())) } != 0 {

        // By default, only print colored text if
        // OS is not WIN32 and stdout is
        // connected to a terminal.
        should_colorize = true;
    }

    if G_ARGS
        .lock()
        //.unwrap()
        .is_arg_set("-color") 
    {
        let color: String 
        = G_ARGS
            .lock()
            //.unwrap()
            .get_arg("-color", DEFAULT_COLOR_SETTING);

        if color == "always" {

            should_colorize = true;

        } else {

            if color == "never" {
                should_colorize = false;

            } else {

                if color != "auto" {
                    return Err(runtime_error("Invalid value for -color option. Valid values: always, auto, never."));
                }
            }
        }
    }

    if should_colorize {
        RESET   = "\x1B[0m".to_string();
        GREEN   = "\x1B[32m".to_string();
        BLUE    = "\x1B[34m".to_string();
        YELLOW  = "\x1B[33m".to_string();
        MAGENTA = "\x1B[35m".to_string();
        CYAN    = "\x1B[36m".to_string();
    }

    let mut result_string: String = format!("{}Chain: {}{}\n",BLUE,result["chain"].get_val_str(), RESET);

    result_string += format!("Blocks:  {}\n",result["blocks"].get_val_str()).as_str();
    result_string += format!("Headers: {}\n",result["headers"].get_val_str()).as_str();

    let ibd_progress: f64 = result["verificationprogress"].get_real();
    let mut ibd_progress_bar = String::default();

    // Display the progress bar only if IBD
    // progress is less than 99%
    if ibd_progress < 0.99 {

        get_progress_bar(ibd_progress, &mut ibd_progress_bar);

        // Add padding between progress bar and
        // IBD progress
        ibd_progress_bar += " ";
    }

    result_string += format!("Verification progress: {}{:.4}%%\n",ibd_progress_bar,ibd_progress * 100.0).as_str();
    result_string += format!("Difficulty: {}\n\n",result["difficulty"].get_val_str()).as_str();

    result_string += format!(
        "{}Network: in {}, out {}, total {}{}\n",
        GREEN,
        result["connections"]["in"].get_val_str(),
        result["connections"]["out"].get_val_str(),
        result["connections"]["total"].get_val_str(),
        RESET
    ).as_str();

    result_string += format!("Version: {}\n",result["version"].get_val_str()).as_str();
    result_string += format!("Time offset (s): {}\n",result["timeoffset"].get_val_str()).as_str();

    // proxies
    let mut proxy_networks  = HashMap::<String,Vec::<String>>::default();;
    let mut ordered_proxies = Vec::<String>::default();

    for network in result["networks"].get_values().unwrap().iter() {

        let mut proxy: String = network["proxy"].get_val_str().to_string();

        if proxy.is_empty() {
            continue;
        }

        //  Add proxy to ordered_proxy if has not been processed
        if proxy_networks.get(&proxy) == None {
            ordered_proxies.push(proxy.clone());
        }

        let name = network["name"].get_val_str().to_string();

        proxy_networks.get_mut(&proxy).unwrap().push(name);
    }

    let mut formatted_proxies = Vec::<String>::default();

    for proxy in ordered_proxies.iter() {
        formatted_proxies.push(
            format!(
                "{} ({})",
                proxy,
                join(proxy_networks.get(proxy).unwrap(),", ")
            )
        );
    }

    result_string += format!("Proxies: {}\n",match formatted_proxies.is_empty() {
        true   => "n/a".to_string(),
        false  => join(&formatted_proxies,", ")
    }).as_str();

    result_string += format!(
        "Min tx relay fee rate ({}/kvB): {}\n\n",
        CURRENCY_UNIT,
        result["relayfee"].get_val_str()
    ).as_str();

    if !result["has_wallet"].is_null() {

        let walletname: &String = result["walletname"].get_val_str();

        result_string += format!(
            "{}Wallet: {}{}\n",MAGENTA,
            match walletname.is_empty() {
                true   => "\"\"",
                false  => walletname.as_str()
            },
            RESET
        ).as_str();

        result_string += format!("Keypool size: {}\n",result["keypoolsize"].get_val_str()).as_str();

        if !result["unlocked_until"].is_null() {

            result_string += format!("Unlocked until: {}\n",result["unlocked_until"].get_val_str()).as_str();
        }

        result_string += format!(
            "Transaction fee rate (-paytxfee) ({}/kvB): {}\n\n",
            CURRENCY_UNIT,
            result["paytxfee"].get_val_str()
        ).as_str();
    }

    if !result["balance"].is_null() {
        result_string += format!("{}Balance:{} {}\n\n",CYAN,RESET,result["balance"].get_val_str()).as_str();
    }

    if !result["balances"].is_null() {

        result_string += format!("{}Balances{}\n",CYAN,RESET).as_str();

        let mut max_balance_length: usize = 10;

        for wallet in result["balances"].get_keys().unwrap().iter() {
            max_balance_length = max(result["balances"][wallet.as_str()].get_val_str().len(),max_balance_length);
        }

        for wallet in result["balances"].get_keys().unwrap().iter() {
            result_string += format!(
                "{:max_balance_length$} {}\n",
                result["balances"][wallet.as_str()].get_val_str(),
                match wallet.is_empty() {
                    true   => "\"\"",
                    false  => wallet
                }
            ).as_str();
        }

        result_string += "\n";
    }

    result_string += format!(
        "{}Warnings:{} {}",
        YELLOW,
        RESET,
        result["warnings"].get_val_str()
    ).as_str();

    result.set_str(&result_string);

    Ok(())
}
