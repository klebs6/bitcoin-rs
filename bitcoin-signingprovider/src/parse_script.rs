crate::ix!();

/**
  | Parse a script in a particular context.
  |
  */
pub fn parse_script(
        key_exp_index: &mut u32,
        sp:            &mut [u8],
        ctx:           ParseScriptContext,
        out:           &mut FlatSigningProvider,
        error:         &mut String) -> Box<DescriptorImpl> {
    
    todo!();
        /*
            using namespace spanparsing;

        auto expr = Expr(sp);
        bool sorted_multi = false;
        if (Func("pk", expr)) {
            auto pubkey = ParsePubkey(key_exp_index, expr, ctx, out, error);
            if (!pubkey) return nullptr;
            ++key_exp_index;
            return std::make_unique<PKDescriptor>(std::move(pubkey), ctx == ParseScriptContext::P2TR);
        }
        if ((ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH || ctx == ParseScriptContext::P2WSH) && Func("pkh", expr)) {
            auto pubkey = ParsePubkey(key_exp_index, expr, ctx, out, error);
            if (!pubkey) return nullptr;
            ++key_exp_index;
            return std::make_unique<PKHDescriptor>(std::move(pubkey));
        } else if (Func("pkh", expr)) {
            error = "Can only have pkh at top level, in sh(), or in wsh()";
            return nullptr;
        }
        if (ctx == ParseScriptContext::TOP && Func("combo", expr)) {
            auto pubkey = ParsePubkey(key_exp_index, expr, ctx, out, error);
            if (!pubkey) return nullptr;
            ++key_exp_index;
            return std::make_unique<ComboDescriptor>(std::move(pubkey));
        } else if (Func("combo", expr)) {
            error = "Can only have combo() at top level";
            return nullptr;
        }
        if ((ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH || ctx == ParseScriptContext::P2WSH) && ((sorted_multi = Func("sortedmulti", expr)) || Func("multi", expr))) {
            auto threshold = Expr(expr);
            uint32_t thres;
            std::vector<std::unique_ptr<PubkeyProvider>> providers;
            if (!ParseUInt32(std::string(threshold.begin(), threshold.end()), &thres)) {
                error = strprintf("Multi threshold '%s' is not valid", std::string(threshold.begin(), threshold.end()));
                return nullptr;
            }
            size_t script_size = 0;
            while (expr.size()) {
                if (!Const(",", expr)) {
                    error = strprintf("Multi: expected ',', got '%c'", expr[0]);
                    return nullptr;
                }
                auto arg = Expr(expr);
                auto pk = ParsePubkey(key_exp_index, arg, ctx, out, error);
                if (!pk) return nullptr;
                script_size += pk->GetSize() + 1;
                providers.emplace_back(std::move(pk));
                key_exp_index++;
            }
            if (providers.empty() || providers.size() > MAX_PUBKEYS_PER_MULTISIG) {
                error = strprintf("Cannot have %u keys in multisig; must have between 1 and %d keys, inclusive", providers.size(), MAX_PUBKEYS_PER_MULTISIG);
                return nullptr;
            } else if (thres < 1) {
                error = strprintf("Multisig threshold cannot be %d, must be at least 1", thres);
                return nullptr;
            } else if (thres > providers.size()) {
                error = strprintf("Multisig threshold cannot be larger than the number of keys; threshold is %d but only %u keys specified", thres, providers.size());
                return nullptr;
            }
            if (ctx == ParseScriptContext::TOP) {
                if (providers.size() > 3) {
                    error = strprintf("Cannot have %u pubkeys in bare multisig; only at most 3 pubkeys", providers.size());
                    return nullptr;
                }
            }
            if (ctx == ParseScriptContext::P2SH) {
                // This limits the maximum number of compressed pubkeys to 15.
                if (script_size + 3 > MAX_SCRIPT_ELEMENT_SIZE) {
                    error = strprintf("P2SH script is too large, %d bytes is larger than %d bytes", script_size + 3, MAX_SCRIPT_ELEMENT_SIZE);
                    return nullptr;
                }
            }
            return std::make_unique<MultisigDescriptor>(thres, std::move(providers), sorted_multi);
        } else if (Func("sortedmulti", expr) || Func("multi", expr)) {
            error = "Can only have multi/sortedmulti at top level, in sh(), or in wsh()";
            return nullptr;
        }
        if ((ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH) && Func("wpkh", expr)) {
            auto pubkey = ParsePubkey(key_exp_index, expr, ParseScriptContext::P2WPKH, out, error);
            if (!pubkey) return nullptr;
            key_exp_index++;
            return std::make_unique<WPKHDescriptor>(std::move(pubkey));
        } else if (Func("wpkh", expr)) {
            error = "Can only have wpkh() at top level or inside sh()";
            return nullptr;
        }
        if (ctx == ParseScriptContext::TOP && Func("sh", expr)) {
            auto desc = ParseScript(key_exp_index, expr, ParseScriptContext::P2SH, out, error);
            if (!desc || expr.size()) return nullptr;
            return std::make_unique<SHDescriptor>(std::move(desc));
        } else if (Func("sh", expr)) {
            error = "Can only have sh() at top level";
            return nullptr;
        }
        if ((ctx == ParseScriptContext::TOP || ctx == ParseScriptContext::P2SH) && Func("wsh", expr)) {
            auto desc = ParseScript(key_exp_index, expr, ParseScriptContext::P2WSH, out, error);
            if (!desc || expr.size()) return nullptr;
            return std::make_unique<WSHDescriptor>(std::move(desc));
        } else if (Func("wsh", expr)) {
            error = "Can only have wsh() at top level or inside sh()";
            return nullptr;
        }
        if (ctx == ParseScriptContext::TOP && Func("addr", expr)) {
            TxDestination dest = DecodeDestination(std::string(expr.begin(), expr.end()));
            if (!IsValidDestination(dest)) {
                error = "Address is not valid";
                return nullptr;
            }
            return std::make_unique<AddressDescriptor>(std::move(dest));
        } else if (Func("addr", expr)) {
            error = "Can only have addr() at top level";
            return nullptr;
        }
        if (ctx == ParseScriptContext::TOP && Func("tr", expr)) {
            auto arg = Expr(expr);
            auto internal_key = ParsePubkey(key_exp_index, arg, ParseScriptContext::P2TR, out, error);
            if (!internal_key) return nullptr;
            ++key_exp_index;
            std::vector<std::unique_ptr<DescriptorImpl>> subscripts; /// list of script subexpressions
            std::vector<int> depths; /// depth in the tree of each subexpression (same length subscripts)
            if (expr.size()) {
                if (!Const(",", expr)) {
                    error = strprintf("tr: expected ',', got '%c'", expr[0]);
                    return nullptr;
                }
                /** The path from the top of the tree to what we're currently processing.
                 * branches[i] == false: left branch in the i'th step from the top; true: right branch.
                 */
                std::vector<bool> branches;
                // Loop over all provided scripts. In every iteration exactly one script will be processed.
                // Use a do-loop because inside this if-branch we expect at least one script.
                do {
                    // First process all open braces.
                    while (Const("{", expr)) {
                        branches.push_back(false); // new left branch
                        if (branches.size() > TAPROOT_CONTROL_MAX_NODE_COUNT) {
                            error = strprintf("tr() supports at most %i nesting levels", TAPROOT_CONTROL_MAX_NODE_COUNT);
                            return nullptr;
                        }
                    }
                    // Process the actual script expression.
                    auto sarg = Expr(expr);
                    subscripts.emplace_back(ParseScript(key_exp_index, sarg, ParseScriptContext::P2TR, out, error));
                    if (!subscripts.back()) return nullptr;
                    depths.push_back(branches.size());
                    // Process closing braces; one is expected for every right branch we were in.
                    while (branches.size() && branches.back()) {
                        if (!Const("}", expr)) {
                            error = strprintf("tr(): expected '}' after script expression");
                            return nullptr;
                        }
                        branches.pop_back(); // move up one level after encountering '}'
                    }
                    // If after that, we're at the end of a left branch, expect a comma.
                    if (branches.size() && !branches.back()) {
                        if (!Const(",", expr)) {
                            error = strprintf("tr(): expected ',' after script expression");
                            return nullptr;
                        }
                        branches.back() = true; // And now we're in a right branch.
                    }
                } while (branches.size());
                // After we've explored a whole tree, we must be at the end of the expression.
                if (expr.size()) {
                    error = strprintf("tr(): expected ')' after script expression");
                    return nullptr;
                }
            }
            assert(TaprootBuilder::ValidDepths(depths));
            return std::make_unique<TRDescriptor>(std::move(internal_key), std::move(subscripts), std::move(depths));
        } else if (Func("tr", expr)) {
            error = "Can only have tr at top level";
            return nullptr;
        }
        if (ctx == ParseScriptContext::TOP && Func("raw", expr)) {
            std::string str(expr.begin(), expr.end());
            if (!IsHex(str)) {
                error = "Raw script is not hex";
                return nullptr;
            }
            auto bytes = ParseHex(str);
            return std::make_unique<RawDescriptor>(CScript(bytes.begin(), bytes.end()));
        } else if (Func("raw", expr)) {
            error = "Can only have raw() at top level";
            return nullptr;
        }
        if (ctx == ParseScriptContext::P2SH) {
            error = "A function is needed within P2SH";
            return nullptr;
        } else if (ctx == ParseScriptContext::P2WSH) {
            error = "A function is needed within P2WSH";
            return nullptr;
        }
        error = strprintf("%s is not a valid descriptor function", std::string(expr.begin(), expr.end()));
        return nullptr;
        */
}

/**
  | Parse a `descriptor` string. Included
  | private keys are put in `out`.
  | 
  | If the descriptor has a checksum, it
  | must be valid. If `require_checksum`
  | is set, the checksum is mandatory - otherwise
  | it is optional.
  | 
  | If a parse error occurs, or the checksum
  | is missing/invalid, or anything else
  | is wrong, `nullptr` is returned.
  |
  */
pub fn parse(
        descriptor:       &String,
        out:              &mut FlatSigningProvider,
        error:            &mut String,
        require_checksum: Option<bool>) -> Box<dyn Descriptor> {

    let require_checksum: bool = require_checksum.unwrap_or(false);
    
    todo!();
        /*
            Span<const char> sp{descriptor};
        if (!CheckChecksum(sp, require_checksum, error)) return nullptr;
        uint32_t key_exp_index = 0;
        auto ret = ParseScript(key_exp_index, sp, ParseScriptContext::TOP, out, error);
        if (sp.size() == 0 && ret) return std::unique_ptr<Descriptor>(std::move(ret));
        return nullptr;
        */
}

