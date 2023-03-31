crate::ix!();

pub trait WalletInitInterface:
HasWalletSupport
+ AddWalletOptions
+ ParameterInteraction
+ Construct
{ }

pub trait HasWalletSupport {

    /**
      | Is the wallet component enabled
      |
      */
    fn has_wallet_support(&self) -> bool;
}

pub trait AddWalletOptions {

    /**
      | Get wallet help string
      |
      */
    fn add_wallet_options(&self, argsman: &mut ArgsManager);
}

pub trait ParameterInteraction {

    /**
      | Check wallet parameter interaction
      |
      */
    fn parameter_interaction(&self) -> bool;
}

pub trait Construct {

    /**
      | Add wallets that should be opened to
      | list of chain clients.
      |
      */
    fn construct(&self, node: &mut NodeContext);
}

pub trait AttachChain {

    fn attach_chain<'a>(
        wallet:          &Arc<Wallet>,
        chain:           &'a mut dyn ChainInterface,
        rescan_required: bool,
        error:           &mut BilingualStr,
        warnings:        &mut Vec<BilingualStr>) -> bool;
}

pub trait Create {

    fn create(
        context:               &mut WalletContext,
        name:                  &String,
        database:              Box<WalletDatabase>,
        wallet_creation_flags: u64,
        error:                 &mut BilingualStr,
        warnings:              &mut Vec<BilingualStr>) -> Arc<Wallet>;
}
