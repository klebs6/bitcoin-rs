// ---------------- [ File: bitcoin-packages/src/packages.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/policy/packages.h]
//-------------------------------------------[.cpp/bitcoin/src/policy/packages.cpp]

/**
  | Default maximum number of transactions
  | in a package.
  |
  */
pub const MAX_PACKAGE_COUNT: usize = 25;

/**
  | Default maximum total virtual size
  | of transactions in a package in KvB.
  |
  */
pub const MAX_PACKAGE_SIZE: usize = 101;

const_assert!{
    MAX_PACKAGE_SIZE * WITNESS_SCALE_FACTOR * 1000 >= MAX_STANDARD_TX_WEIGHT
}

/**
  | A "reason" why a package was invalid.
  | It may be that one or more of the included
  | transactions is invalid or the package
  | itself violates our rules.
  | 
  | We don't distinguish between consensus
  | and policy violations right now.
  |
  */
pub enum PackageValidationResult {

    /**
      | Initial value. The package has not yet
      | been rejected.
      |
      */
    PCKG_RESULT_UNSET = 0,        

    /**
      | The package itself is invalid (e.g.
      | too many transactions).
      |
      */
    PCKG_POLICY,                  

    /**
      | At least one tx is invalid.
      |
      */
    PCKG_TX,                      
}

/**
  | A package is an ordered list of transactions.
  | The transactions cannot conflict with
  | (spend the same inputs as) one another.
  |
  */
pub type Package = Vec<TransactionRef>;

pub struct PackageValidationState {
    base: ValidationState<PackageValidationResult>,
}

/**
  | Context-free package policy checks:
  | 
  | 1. The number of transactions cannot
  | exceed MAX_PACKAGE_COUNT.
  | 
  | 2. The total virtual size cannot exceed
  | MAX_PACKAGE_SIZE.
  | 
  | 3. If any dependencies exist between
  | transactions, parents must appear
  | before children.
  | 
  | 4. Transactions cannot conflict,
  | i.e., spend the same inputs.
  |
  */
pub fn check_package(
        txns:  &Package,
        state: &mut PackageValidationState) -> bool {
    
    todo!();
        /*
            const unsigned int package_count = txns.size();

        if (package_count > MAX_PACKAGE_COUNT) {
            return state.Invalid(PackageValidationResult::PCKG_POLICY, "package-too-many-transactions");
        }

        const int64_t total_size = std::accumulate(txns.cbegin(), txns.cend(), 0,
                                   [](int64_t sum, const auto& tx) { return sum + GetVirtualTransactionSize(*tx); });
        // If the package only contains 1 tx, it's better to report the policy violation on individual tx size.
        if (package_count > 1 && total_size > MAX_PACKAGE_SIZE * 1000) {
            return state.Invalid(PackageValidationResult::PCKG_POLICY, "package-too-large");
        }

        // Require the package to be sorted in order of dependency, i.e. parents appear before children.
        // An unsorted package will fail anyway on missing-inputs, but it's better to quit earlier and
        // fail on something less ambiguous (missing-inputs could also be an orphan or trying to
        // spend nonexistent coins).
        std::unordered_set<uint256, SaltedTxidHasher> later_txids;
        std::transform(txns.cbegin(), txns.cend(), std::inserter(later_txids, later_txids.end()),
                       [](const auto& tx) { return tx->GetHash(); });
        for (const auto& tx : txns) {
            for (const auto& input : tx->vin) {
                if (later_txids.find(input.prevout.hash) != later_txids.end()) {
                    // The parent is a subsequent transaction in the package.
                    return state.Invalid(PackageValidationResult::PCKG_POLICY, "package-not-sorted");
                }
            }
            later_txids.erase(tx->GetHash());
        }

        // Don't allow any conflicting transactions, i.e. spending the same inputs, in a package.
        std::unordered_set<OutPoint, SaltedOutpointHasher> inputs_seen;
        for (const auto& tx : txns) {
            for (const auto& input : tx->vin) {
                if (inputs_seen.find(input.prevout) != inputs_seen.end()) {
                    // This input is also present in another tx in the package.
                    return state.Invalid(PackageValidationResult::PCKG_POLICY, "conflict-in-package");
                }
            }
            // Batch-add all the inputs for a tx at a time. If we added them 1 at a time, we could
            // catch duplicate inputs within a single tx.  This is a more severe, consensus error,
            // and we want to report that from CheckTransaction instead.
            std::transform(tx->vin.cbegin(), tx->vin.cend(), std::inserter(inputs_seen, inputs_seen.end()),
                           [](const auto& input) { return input.prevout; });
        }
        return true;
        */
}
