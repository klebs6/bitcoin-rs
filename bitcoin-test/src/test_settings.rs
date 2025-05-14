// ---------------- [ File: bitcoin-test/src/test_settings.rs ]
crate::ix!();



//-------------------------------------------[.cpp/bitcoin/src/test/settings_tests.cpp]

#[inline] pub fn write_text(
        path: &Path,
        text: &String)  {
    
    todo!();
        /*
            fsbridge::ofstream file;
        file.open(path);
        file << text;
        */
}

#[cfg(test)]
#[fixture(BasicTestingSetup)]
pub mod settings_tests {

    #[test] fn read_write() {
        todo!();
        /*
        
            fs::path path = m_args.GetDataDirBase() / "settings.json";

            WriteText(path, R"({
                "string": "string",
                "num": 5,
                "bool": true,
                "null": null
            })");

            std::map<std::string, SettingsValue> expected{
                {"string", "string"},
                {"num", 5},
                {"bool", true},
                {"null", {}},
            };

            // Check file read.
            std::map<std::string, SettingsValue> values;
            std::vector<std::string> errors;
            BOOST_CHECK(ReadSettings(path, values, errors));
            BOOST_CHECK_EQUAL_COLLECTIONS(values.begin(), values.end(), expected.begin(), expected.end());
            BOOST_CHECK(errors.empty());

            // Check no errors if file doesn't exist.
            fs::remove(path);
            BOOST_CHECK(ReadSettings(path, values, errors));
            BOOST_CHECK(values.empty());
            BOOST_CHECK(errors.empty());

            // Check duplicate keys not allowed
            WriteText(path, R"({
                "dupe": "string",
                "dupe": "dupe"
            })");
            BOOST_CHECK(!ReadSettings(path, values, errors));
            std::vector<std::string> dup_keys = {strprintf("Found duplicate key dupe in settings file %s", fs::PathToString(path))};
            BOOST_CHECK_EQUAL_COLLECTIONS(errors.begin(), errors.end(), dup_keys.begin(), dup_keys.end());

            // Check non-kv json files not allowed
            WriteText(path, R"("non-kv")");
            BOOST_CHECK(!ReadSettings(path, values, errors));
            std::vector<std::string> non_kv = {strprintf("Found non-object value \"non-kv\" in settings file %s", fs::PathToString(path))};
            BOOST_CHECK_EQUAL_COLLECTIONS(errors.begin(), errors.end(), non_kv.begin(), non_kv.end());

            // Check invalid json not allowed
            WriteText(path, R"(invalid json)");
            BOOST_CHECK(!ReadSettings(path, values, errors));
            std::vector<std::string> fail_parse = {strprintf("Unable to parse settings file %s", fs::PathToString(path))};
            BOOST_CHECK_EQUAL_COLLECTIONS(errors.begin(), errors.end(), fail_parse.begin(), fail_parse.end());

        */
    }

    /**
      | Check settings struct contents against
      | expected json strings.
      |
      */
    pub fn check_values(
            settings:   &Settings,
            single_val: &String,
            list_val:   &String)  {
        
        todo!();
            /*
                SettingsValue single_value = GetSetting(settings, "section", "name", false, false);
                SettingsValue list_value(SettingsValue::VARR);
                for (const auto& item : GetSettingsList(settings, "section", "name", false)) {
                    list_value.push_back(item);
                }
                BOOST_CHECK_EQUAL(single_value.write().c_str(), single_val);
                BOOST_CHECK_EQUAL(list_value.write().c_str(), list_val);
            */
    }

    /**
      | Simple settings merge test case.
      |
      */
    #[test] fn simple() {
        todo!();
        /*
        
            Settings settings;
            settings.command_line_options["name"].push_back("val1");
            settings.command_line_options["name"].push_back("val2");
            settings.ro_config["section"]["name"].push_back(2);

            // The last given arg takes precedence when specified via commandline.
            CheckValues(settings, R"("val2")", R"(["val1","val2",2])");

            Settings settings2;
            settings2.ro_config["section"]["name"].push_back("val2");
            settings2.ro_config["section"]["name"].push_back("val3");

            // The first given arg takes precedence when specified via config file.
            CheckValues(settings2, R"("val2")", R"(["val2","val3"])");

        */
    }

    /**
      | Confirm that a high priority setting
      | overrides a lower priority setting even if
      | the high priority setting is null. This
      | behavior is useful for a high priority
      | setting source to be able to effectively
      | reset any setting back to its default
      | value.
      */
    #[test] fn null_override() {
        todo!();
        /*
        
            Settings settings;
            settings.command_line_options["name"].push_back("value");
            BOOST_CHECK_EQUAL(R"("value")", GetSetting(settings, "section", "name", false, false).write().c_str());
            settings.forced_settings["name"] = {};
            BOOST_CHECK_EQUAL(R"(null)", GetSetting(settings, "section", "name", false, false).write().c_str());

        */
    }

    /**
      | Test different ways settings can be merged,
      | and verify results. This test can be used
      | to confirm that updates to settings code
      | don't change behavior unintentionally.
      */
    pub struct MergeTestingSetup {
        base: BasicTestingSetup,
    }

    pub mod merge_testing_setup {

        /**
          | Max number of actions to sequence
          | together. Can decrease this when
          | debugging to make test results easier
          | to understand.
          */
        pub const MAX_ACTIONS: i32 = 3;

        pub enum Action { 
            END, 
            SET, 
            NEGATE, 
            SECTION_SET, 
            SECTION_NEGATE 
        }

        pub type ActionList = [Action; MAX_ACTIONS];
    }

    impl MergeTestingSetup {

        /**
          | Enumerate all possible test configurations.
          |
          */
        pub fn for_each_merge_setup<Fn>(&mut self, fn_: Fn)  {
        
            todo!();
            /*
                ActionList arg_actions = {};
                    // command_line_options do not have sections. Only iterate over SET and NEGATE
                    ForEachNoDup(arg_actions, SET, NEGATE, [&]{
                        ActionList conf_actions = {};
                        ForEachNoDup(conf_actions, SET, SECTION_NEGATE, [&]{
                            for (bool force_set : {false, true}) {
                                for (bool ignore_default_section_config : {false, true}) {
                                    fn(arg_actions, conf_actions, force_set, ignore_default_section_config);
                                }
                            }
                        });
                    });
            */
        }
    }

    /**
      | Regression test covering different ways
      | config settings can be merged. The test
      | parses and merges settings, representing
      | the results as strings that get compared
      | against an expected hash. To debug, the
      | result strings can be dumped to a file (see
      | comments below).
      */
    #[test] fn merge_testing_setup() {
        todo!();
        /*
        
            CHash256 out_sha;
            FILE* out_file = nullptr;
            if (const char* out_path = getenv("SETTINGS_MERGE_TEST_OUT")) {
                out_file = fsbridge::fopen(out_path, "w");
                if (!out_file) throw std::system_error(errno, std::generic_category(), "fopen failed");
            }

            const std::string& network = CBaseChainParams::MAIN;
            ForEachMergeSetup([&](const ActionList& arg_actions, const ActionList& conf_actions, bool force_set,
                                  bool ignore_default_section_config) {
                std::string desc;
                int value_suffix = 0;
                Settings settings;

                const std::string& name = ignore_default_section_config ? "wallet" : "server";
                auto push_values = [&](Action action, const char* value_prefix, const std::string& name_prefix,
                                       std::vector<SettingsValue>& dest) {
                    if (action == SET || action == SECTION_SET) {
                        for (int i = 0; i < 2; ++i) {
                            dest.push_back(value_prefix + ToString(++value_suffix));
                            desc += " " + name_prefix + name + "=" + dest.back().get_str();
                        }
                    } else if (action == NEGATE || action == SECTION_NEGATE) {
                        dest.push_back(false);
                        desc += " " + name_prefix + "no" + name;
                    }
                };

                if (force_set) {
                    settings.forced_settings[name] = "forced";
                    desc += " " + name + "=forced";
                }
                for (Action arg_action : arg_actions) {
                    push_values(arg_action, "a", "-", settings.command_line_options[name]);
                }
                for (Action conf_action : conf_actions) {
                    bool use_section = conf_action == SECTION_SET || conf_action == SECTION_NEGATE;
                    push_values(conf_action, "c", use_section ? network + "." : "",
                        settings.ro_config[use_section ? network : ""][name]);
                }

                desc += " || ";
                desc += GetSetting(settings, network, name, ignore_default_section_config, /* get_chain_name= */ false).write();
                desc += " |";
                for (const auto& s : GetSettingsList(settings, network, name, ignore_default_section_config)) {
                    desc += " ";
                    desc += s.write();
                }
                desc += " |";
                if (OnlyHasDefaultSectionSetting(settings, network, name)) desc += " ignored";
                desc += "\n";

                out_sha.Write(MakeUCharSpan(desc));
                if (out_file) {
                    BOOST_REQUIRE(fwrite(desc.data(), 1, desc.size(), out_file) == desc.size());
                }
            });

            if (out_file) {
                if (fclose(out_file)) throw std::system_error(errno, std::generic_category(), "fclose failed");
                out_file = nullptr;
            }

            unsigned char out_sha_bytes[CSHA256::OUTPUT_SIZE];
            out_sha.Finalize(out_sha_bytes);
            std::string out_sha_hex = HexStr(out_sha_bytes);

            // If check below fails, should manually dump the results with:
            //
            //   SETTINGS_MERGE_TEST_OUT=results.txt ./test_bitcoin --run_test=settings_tests/Merge
            //
            // And verify diff against previous results to make sure the changes are expected.
            //
            // Results file is formatted like:
            //
            //   <input> || GetSetting() | GetSettingsList() | OnlyHasDefaultSectionSetting()
            BOOST_CHECK_EQUAL(out_sha_hex, "79db02d74e3e193196541b67c068b40ebd0c124a24b3ecbe9cbf7e85b1c4ba7a");

        */
    }
}
