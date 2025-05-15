// ---------------- [ File: bitcoin-signingprovider/src/descriptor.rs ]
crate::ix!();

/**
  | @brief
  | 
  | Interface for parsed descriptor objects.
  | 
  | Descriptors are strings that describe
  | a set of scriptPubKeys, together with
  | all information necessary to solve
  | them. By combining all information
  | into one, they avoid the need to separately
  | import keys and scripts.
  | 
  | Descriptors may be ranged, which occurs
  | when the public keys inside are specified
  | in the form of HD chains (xpubs).
  | 
  | Descriptors always represent public
  | information - public keys and scripts
  | - but in cases where private keys need
  | to be conveyed along with a descriptor,
  | they can be included inside by changing
  | public keys to private keys (WIF format),
  | and changing xpubs by xprvs.
  | 
  | Reference documentation about the
  | descriptor language can be found in
  | doc/descriptors.md.
  |
  */
pub trait Descriptor:

/*
   | Whether the expansion of this descriptor
   | depends on the position.
   |
   */
IsRange

/*
  | Whether this descriptor has all information
  | about signing ignoring lack of private
  | keys.
  | 
  | This is true for all descriptors except
  | ones that use `raw` or `addr` constructions.
  |
  */
+ IsSolvable

/*
  | Convert the descriptor back to a string,
  | undoing parsing.
  |
  | Get the descriptor string form.
  |
  */
+ ToString

/*
  | Whether this descriptor will return
  | one scriptPubKey or multiple (aka is
  | or is not combo)
  |
  */
+ IsSingleType

+ ToPrivateString
+ ToNormalizedString
+ Expand
+ ExpandFromCache
+ ExpandPrivate
+ GetOutputType { }

pub trait PubKeyProviderInterface:
    GetPubKey

    /*
      | Whether this represent multiple public
      | keys at different positions.
      |
      */
    + IsRange

    + GetSize
    + ToString
    + ToPrivateString
    + ToNormalizedString
    + GetPrivKey {}


pub trait IsRange {
    fn is_range(&self) -> bool;
}

pub trait IsSolvable   { fn is_solvable(&self)    -> bool; }
pub trait ToString     { fn to_string(&self)      -> String; }
pub trait IsSingleType { fn is_single_type(&self) -> bool; }

pub trait ToPrivateString {

    /**
      | Convert the descriptor to a private
      | string. This fails if the provided provider
      | does not have the relevant private keys.
      |
      ------------------
      | Get the descriptor string form including
      | private data (if available in provider).
      |
      */
    fn to_private_string(&self, 
            provider: &SigningProvider,
            out:      &mut String) -> bool;
}

pub trait Expand {

    /**
      | Expand a descriptor at a specified position.
      | 
      | -----------
      | @param[in] pos
      | 
      | The position at which to expand the descriptor.
      | If IsRange() is false, this is ignored.
      | ----------
      | @param[in] provider
      | 
      | The provider to query for private keys
      | in case of hardened derivation.
      | ----------
      | @param[out] output_scripts
      | 
      | The expanded scriptPubKeys.
      | ----------
      | @param[out] out
      | 
      | Scripts and public keys necessary for
      | solving the expanded scriptPubKeys
      | (may be equal to `provider`).
      | ----------
      | @param[out] write_cache
      | 
      | Cache data necessary to evaluate the
      | descriptor at this point without access
      | to private keys.
      |
      */
    fn expand(&self, 
            pos:            i32,
            provider:       &SigningProvider,
            output_scripts: &mut Vec<Script>,
            out:            &mut FlatSigningProvider,
            write_cache:    Option<*mut DescriptorCache>) -> bool;
}

pub trait ExpandFromCache {

    /**
      | Expand a descriptor at a specified position
      | using cached expansion data.
      | 
      | -----------
      | @param[in] pos
      | 
      | The position at which to expand the descriptor.
      | If IsRange() is false, this is ignored.
      | ----------
      | @param[in] read_cache
      | 
      | Cached expansion data.
      | ----------
      | @param[out] output_scripts
      | 
      | The expanded scriptPubKeys.
      | ----------
      | @param[out] out
      | 
      | Scripts and public keys necessary for
      | solving the expanded scriptPubKeys
      | (may be equal to `provider`).
      |
      */
    fn expand_from_cache(&self, 
            pos:            i32,
            read_cache:     &DescriptorCache,
            output_scripts: &mut Vec<Script>,
            out:            &mut FlatSigningProvider) -> bool;
}

pub trait ExpandPrivate {

    /**
      | Expand the private key for a descriptor
      | at a specified position, if possible.
      | 
      | -----------
      | @param[in] pos
      | 
      | The position at which to expand the descriptor.
      | If IsRange() is false, this is ignored.
      | ----------
      | @param[in] provider
      | 
      | The provider to query for the private
      | keys.
      | ----------
      | @param[out] out
      | 
      | Any private keys available for the specified
      | `pos`.
      |
      */
    fn expand_private(&self, 
            pos:      i32,
            provider: &SigningProvider,
            out:      &mut FlatSigningProvider);
}

pub trait GetOutputType {

    /**
      | @return
      | 
      | The OutputType of the scriptPubKey(s)
      | produced by this descriptor. Or nullopt
      | if indeterminate (multiple or none)
      |
      */
    fn get_output_type(&self) -> Option<OutputType>;
}

pub trait GetPubKey {

    /**
      | Derive a public key.
      | 
      | read_cache is the cache to read keys
      | from (if not nullptr)
      | 
      | write_cache is the cache to write keys
      | to (if not nullptr)
      | 
      | Caches are not exclusive but this is
      | not tested. Currently we use them exclusively
      |
      */
    fn get_pub_key(&self, 
        pos:         i32,
        arg:         &SigningProvider,
        key:         &mut crate::PubKey,
        info:        &mut KeyOriginInfo,
        read_cache:  *const DescriptorCache,
        write_cache: *mut DescriptorCache) -> bool;
}

pub trait GetSize {

    /**
      | Get the size of the generated public
      | key(s) in bytes (33 or 65).
      |
      */
    fn get_size(&self) -> usize;
}

pub trait ToNormalizedString {

    /**
      | Convert the descriptor to a normalized
      | string. Normalized descriptors have
      | the xpub at the last hardened step. This
      | fails if the provided provider does
      | not have the private keys to derive that
      | xpub.
      |
      ---------------------
      | Get the descriptor string form with
      | the xpub at the last hardened derivation
      |
      */
    fn to_normalized_string(&self, 
            provider:   &SigningProvider,
            out:   &mut String,
            cache: *const DescriptorCache) -> bool;
}

pub trait GetPrivKey {

    /**
      | Derive a private key, if private data
      | is available in arg.
      |
      */
    fn get_priv_key(&self, 
            pos: i32,
            arg: &SigningProvider,
            key: &mut Key) -> bool;
}

pub trait ToStringExtra {
    fn to_string_extra(&self) -> String;
}

pub trait ToStringSubScriptHelper {
    fn to_string_subscript_helper(&self, 
        arg:   *const SigningProvider,
        ret:   &mut String,
        ty:    DescriptorImplStringType,
        cache: Option<*const DescriptorCache>) -> bool;
}

pub trait MakeScripts {

    /**
      | A helper function to construct the scripts
      | for this descriptor.
      | 
      | This function is invoked once by ExpandHelper.
      | 
      | -----------
      | @param pubkeys
      | 
      | The evaluations of the m_pubkey_args
      | field.
      | ----------
      | @param scripts
      | 
      | The evaluations of m_subdescriptor_args
      | (one for each m_subdescriptor_args
      | element).
      | ----------
      | @param out
      | 
      | A FlatSigningProvider to put scripts
      | or public keys in that are necessary
      | to the solver.
      | 
      | The origin info of the provided pubkeys
      | is automatically added.
      | 
      | -----------
      | @return
      | 
      | A vector with scriptPubKeys for this
      | descriptor.
      |
      */
    fn make_scripts(&self, 
            pubkeys: &Vec<crate::PubKey>,
            scripts: &[Script],
            out:     &mut FlatSigningProvider) -> Vec<Script>;
}

//-------------------------------------------[.cpp/bitcoin/src/script/descriptor.h]

pub type ExtPubKeyMap = HashMap<u32,ExtPubKey>;


//-------------------------------------------[.cpp/bitcoin/src/script/descriptor.cpp]

/* ------------------- Checksum  ------------------- */

pub type KeyPath = Vec<u32>;

pub enum DeriveType {
    NO,
    UNHARDENED,
    HARDENED,
}
