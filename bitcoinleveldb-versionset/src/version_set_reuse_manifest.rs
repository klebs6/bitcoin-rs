// ---------------- [ File: bitcoinleveldb-versionset/src/version_set_reuse_manifest.rs ]
crate::ix!();

impl ReuseManifest for VersionSet {

    fn reuse_manifest(
        &mut self, 
        dscname: &str,
        dscbase: &str) -> bool {
        
        todo!();
        /*
            if (!options_->reuse_logs) {
        return false;
      }
      FileType manifest_type;
      uint64_t manifest_number;
      uint64_t manifest_size;
      if (!ParseFileName(dscbase, &manifest_number, &manifest_type) ||
          manifest_type != kDescriptorFile ||
          !env_->GetFileSize(dscname, &manifest_size).ok() ||
          // Make new compacted MANIFEST if old one is too big
          manifest_size >= TargetFileSize(options_)) {
        return false;
      }

      assert(descriptor_file_ == nullptr);
      assert(descriptor_log_ == nullptr);
      Status r = env_->NewAppendableFile(dscname, &descriptor_file_);
      if (!r.ok()) {
        Log(options_->info_log, "Reuse MANIFEST: %s\n", r.ToString().c_str());
        assert(descriptor_file_ == nullptr);
        return false;
      }

      Log(options_->info_log, "Reusing MANIFEST %s\n", dscname.c_str());
      descriptor_log_ = new LogWriter(descriptor_file_, manifest_size);
      manifest_file_number_ = manifest_number;
      return true;
        */
    }
}
