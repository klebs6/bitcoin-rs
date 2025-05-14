// ---------------- [ File: bitcoin-signingprovider/src/tr_descriptor.rs ]
crate::ix!();

/**
  | A parsed tr(...) descriptor.
  |
  */
pub struct TRDescriptor {
    base:   DescriptorImpl,
    depths: Vec<i32>,
}

impl TRDescriptor {

    pub fn make_scripts(&self, 
        keys:    &Vec<crate::PubKey>,
        scripts: &[Script],
        out:     &mut FlatSigningProvider) -> Vec<Script> {
        
        todo!();
        /*
            TaprootBuilder builder;
            assert(m_depths.size() == scripts.size());
            for (size_t pos = 0; pos < m_depths.size(); ++pos) {
                builder.Add(m_depths[pos], scripts[pos], TAPROOT_LEAF_TAPSCRIPT);
            }
            if (!builder.IsComplete()) return {};
            assert(keys.size() == 1);
            crate::XOnlyPubKey xpk(keys[0]);
            if (!xpk.IsFullyValid()) return {};
            builder.Finalize(xpk);
            WitnessV1Taproot output = builder.GetOutput();
            out.tr_spenddata[output].Merge(builder.GetSpendData());
            return Vector(GetScriptForDestination(output));
        */
    }
    
    pub fn to_string_sub_script_helper(&self, 
        arg:   *const SigningProvider,
        ret:   &mut String,
        ty:    DescriptorImplStringType,
        cache: Option<*const DescriptorCache>) -> bool {

        todo!();
        /*
            if (m_depths.empty()) return true;
            std::vector<bool> path;
            for (size_t pos = 0; pos < m_depths.size(); ++pos) {
                if (pos) ret += ',';
                while ((int)path.size() <= m_depths[pos]) {
                    if (path.size()) ret += '{';
                    path.push_back(false);
                }
                std::string tmp;
                if (!m_subdescriptor_args[pos]->ToStringHelper(arg, tmp, type, cache)) return false;
                ret += std::move(tmp);
                while (!path.empty() && path.back()) {
                    if (path.size() > 1) ret += '}';
                    path.pop_back();
                }
                if (!path.empty()) path.back() = true;
            }
            return true;
        */
    }
    
    pub fn new(
        internal_key: Box<PubkeyProvider>,
        descs:        Vec<Box<DescriptorImpl>>,
        depths:       Vec<i32>) -> Self {
    
        todo!();
        /*


            : DescriptorImpl(Vector(std::move(internal_key)), std::move(descs), "tr"), m_depths(std::move(depths))

            assert(m_subdescriptor_args.size() == m_depths.size());
        */
    }
}

impl GetOutputType for TRDescriptor {

    fn get_output_type(&self) -> Option<OutputType> {
        
        todo!();
        /*
            return OutputType::BECH32M;
        */
    }
}

impl IsSingleType for TRDescriptor {
    fn is_single_type(&self) -> bool {
        
        todo!();
        /*
            return true;
        */
    }
}
