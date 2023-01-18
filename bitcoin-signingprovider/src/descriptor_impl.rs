crate::ix!();

/**
  | Base class for all Descriptor implementations.
  |
  */
pub struct DescriptorImpl {

    /**
      | Public key arguments for this descriptor
      | (size 1 for PK, PKH, WPKH; any size for
      | Multisig).
      |
      */
    pubkey_args:        Vec<Box<PubkeyProvider>>,

    /**
      | The string name of the descriptor function.
      |
      */
    name:               String,

    /**
      | The sub-descriptor arguments (empty for
      | everything but SH and WSH).
      |
      | In doc/descriptors.m this is referred to
      | as SCRIPT expressions sh(SCRIPT) and
      | wsh(SCRIPT), and distinct from KEY
      | expressions and ADDR expressions.
      |
      | Subdescriptors can only ever generate
      | a single script.
      */
    subdescriptor_args: Vec<Box<DescriptorImpl>>,
}

pub trait DescriptorImplInterface: MakeScripts {}

pub enum DescriptorImplStringType
{
    PUBLIC,
    PRIVATE,
    NORMALIZED,
}

impl ToStringExtra for DescriptorImpl {

    /**
      | Return a serialization of anything
      | except pubkey and script arguments,
      | to be prepended to those.
      |
      */
    fn to_string_extra(&self) -> String {
        
        todo!();
        /*
            return "";
        */
    }
}

impl ToStringSubScriptHelper for DescriptorImpl {

    fn to_string_subscript_helper(&self, 
        arg:   *const SigningProvider,
        ret:   &mut String,
        ty:    DescriptorImplStringType,
        cache: Option<*const DescriptorCache>) -> bool {

        todo!();
        /*
            size_t pos = 0;
            for (const auto& scriptarg : m_subdescriptor_args) {
                if (pos++) ret += ",";
                std::string tmp;
                if (!scriptarg->ToStringHelper(arg, tmp, type, cache)) return false;
                ret += std::move(tmp);
            }
            return true;
        */
    }
}

impl ToPrivateString for DescriptorImpl {
    fn to_private_string(&self, 
        arg: &SigningProvider,
        out: &mut String) -> bool {
        
        todo!();
        /*
            bool ret = ToStringHelper(&arg, out, StringType::PRIVATE);
            out = AddChecksum(out);
            return ret;
        */
    }
}

impl DescriptorImpl {

    pub fn new(
        pubkeys: Vec<Box<PubkeyProvider>>,
        name:    &String) -> Self {
    
        todo!();
        /*
        : pubkey_args(std::move(pubkeys)),
        : name(name),
        : subdescriptor_args(),

        
        */
    }
    
    pub fn new_with_script(
        pubkeys: Vec<Box<PubkeyProvider>>,
        script:  Box<DescriptorImpl>,
        name:    &String) -> Self {
    
        todo!();
        /*
        : pubkey_args(std::move(pubkeys)),
        : name(name),
        : subdescriptor_args(Vector(std::move(script))),

        
        */
    }
    
    pub fn new_with_scripts(
        pubkeys: Vec<Box<PubkeyProvider>>,
        scripts: Vec<Box<DescriptorImpl>>,
        name:    &String) -> Self {
    
        todo!();
        /*
        : pubkey_args(std::move(pubkeys)),
        : name(name),
        : subdescriptor_args(std::move(scripts)),

        
        */
    }
    
    pub fn to_string_helper(&self, 
        arg:   *const SigningProvider,
        out:   &mut String,
        ty:    DescriptorImplStringType,
        cache: Option<*const DescriptorCache>) -> bool {

        todo!();
        /*
            std::string extra = ToStringExtra();
            size_t pos = extra.size() > 0 ? 1 : 0;
            std::string ret = m_name + "(" + extra;
            for (const auto& pubkey : m_pubkey_args) {
                if (pos++) ret += ",";
                std::string tmp;
                switch (type) {
                    case StringType::NORMALIZED:
                        if (!pubkey->ToNormalizedString(*arg, tmp, cache)) return false;
                        break;
                    case StringType::PRIVATE:
                        if (!pubkey->ToPrivateString(*arg, tmp)) return false;
                        break;
                    case StringType::PUBLIC:
                        tmp = pubkey->ToString();
                        break;
                }
                ret += std::move(tmp);
            }
            std::string subscript;
            if (!ToStringSubScriptHelper(arg, subscript, type, cache)) return false;
            if (pos && subscript.size()) ret += ',';
            out = std::move(ret) + std::move(subscript) + ")";
            return true;
        */
    }
    
    pub fn expand_helper(&self, 
        pos:            i32,
        arg:            &SigningProvider,
        read_cache:     *const DescriptorCache,
        output_scripts: &mut Vec<Script>,
        out:            &mut FlatSigningProvider,
        write_cache:    *mut DescriptorCache) -> bool {
        
        todo!();
        /*
            std::vector<std::pair<CPubKey, KeyOriginInfo>> entries;
            entries.reserve(m_pubkey_args.size());

            // Construct temporary data in `entries`, `subscripts`, and `subprovider` to avoid producing output in case of failure.
            for (const auto& p : m_pubkey_args) {
                entries.emplace_back();
                if (!p->GetPubKey(pos, arg, entries.back().first, entries.back().second, read_cache, write_cache)) return false;
            }
            std::vector<CScript> subscripts;
            FlatSigningProvider subprovider;
            for (const auto& subarg : m_subdescriptor_args) {
                std::vector<CScript> outscripts;
                if (!subarg->ExpandHelper(pos, arg, read_cache, outscripts, subprovider, write_cache)) return false;
                assert(outscripts.size() == 1);
                subscripts.emplace_back(std::move(outscripts[0]));
            }
            out = Merge(std::move(out), std::move(subprovider));

            std::vector<CPubKey> pubkeys;
            pubkeys.reserve(entries.size());
            for (auto& entry : entries) {
                pubkeys.push_back(entry.first);
                out.origins.emplace(entry.first.GetID(), std::make_pair<CPubKey, KeyOriginInfo>(CPubKey(entry.first), std::move(entry.second)));
            }

            output_scripts = MakeScripts(pubkeys, MakeSpan(subscripts), out);
            return true;
        */
    }
}

impl GetOutputType for DescriptorImpl {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            return std::nullopt;
        */
    }
}

impl Expand for DescriptorImpl {

    fn expand(&self, 
        pos:            i32,
        provider:       &SigningProvider,
        output_scripts: &mut Vec<Script>,
        out:            &mut FlatSigningProvider,
        write_cache:    Option<*mut DescriptorCache>) -> bool {

        todo!();
        /*
            return ExpandHelper(pos, provider, nullptr, output_scripts, out, write_cache);
        */
    }
}

impl ExpandPrivate for DescriptorImpl {

    fn expand_private(&self, 
        pos:      i32,
        provider: &SigningProvider,
        out:      &mut FlatSigningProvider)  {
        
        todo!();
        /*
            for (const auto& p : m_pubkey_args) {
                CKey key;
                if (!p->GetPrivKey(pos, provider, key)) continue;
                out.keys.emplace(key.GetPubKey().GetID(), key);
            }
            for (const auto& arg : m_subdescriptor_args) {
                arg->ExpandPrivate(pos, provider, out);
            }
        */
    }
}

impl ExpandFromCache for DescriptorImpl {

    fn expand_from_cache(&self, 
        pos:            i32,
        read_cache:     &DescriptorCache,
        output_scripts: &mut Vec<Script>,
        out:            &mut FlatSigningProvider) -> bool {
        
        todo!();
        /*
            return ExpandHelper(pos, DUMMY_SIGNING_PROVIDER, &read_cache, output_scripts, out, nullptr);
        */
    }
}

impl IsRange for DescriptorImpl {
    fn is_range(&self) -> bool {
        
        todo!();
        /*
            for (const auto& pubkey : m_pubkey_args) {
                if (pubkey->IsRange()) return true;
            }
            for (const auto& arg : m_subdescriptor_args) {
                if (arg->IsRange()) return true;
            }
            return false;
        */
    }
}
    
impl IsSolvable for DescriptorImpl {

    fn is_solvable(&self) -> bool {
        
        todo!();
        /*
            for (const auto& arg : m_subdescriptor_args) {
                if (!arg->IsSolvable()) return false;
            }
            return true;
        */
    }
}

impl ToString for DescriptorImpl {
    fn to_string(&self) -> String {
        
        todo!();
        /*
            std::string ret;
            ToStringHelper(nullptr, ret, StringType::PUBLIC);
            return AddChecksum(ret);
        */
    }
}

impl ToNormalizedString for DescriptorImpl {
    fn to_normalized_string(&self, 
        arg:   &SigningProvider,
        out:   &mut String,
        cache: *const DescriptorCache) -> bool {
        
        todo!();
        /*
            bool ret = ToStringHelper(&arg, out, StringType::NORMALIZED, cache);
            out = AddChecksum(out);
            return ret;
        */
    }
}

