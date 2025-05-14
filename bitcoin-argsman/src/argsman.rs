// ---------------- [ File: bitcoin-argsman/src/argsman.rs ]
crate::ix!();

pub const BITCOIN_CONF_FILENAME:     &'static str = "bitcoin.conf";
pub const BITCOIN_SETTINGS_FILENAME: &'static str = "settings.json";

#[derive(Default)]
pub struct ArgsManager {
    pub cs_args: Arc<Mutex<ArgsManagerInner>>,
}

lazy_static!{
    pub static ref G_ARGS: Mutex<ArgsManager> = Mutex::new(ArgsManager::default());
}

impl ArgsManager {

    #[inline] pub fn add_command(&mut self, 
        cmd:  &str,
        help: &str)  {
        self.cs_args.lock().add_command(cmd,help)
    }

    #[inline] pub fn add_arg(&mut self, x: &ArgDescriptor)  {
        self.cs_args.lock().add_arg(x)
    }

    #[inline] pub fn add_hidden_args(&mut self, names: &Vec<&'static str>)  {
        self.cs_args.lock().add_hidden_args(names)
    }

    #[inline] pub fn setup_cli_args(&mut self) {
        self.cs_args.lock().setup_cli_args()
    }

    #[inline] pub fn setup_chain_params_base_options(&mut self) {
        self.cs_args.lock().setup_chain_params_base_options()
    }

    #[inline] pub fn clear_args(&mut self)  {
        self.cs_args.lock().clear_args()
    }

    #[inline] pub fn clear_path_cache(&mut self)  {
        self.cs_args.lock().clear_path_cache()
    }

    #[inline] pub fn get_arg_flags(&self, name: &str) -> Option<u32> {
        self.cs_args.lock().get_arg_flags(name)
    }

    #[inline] pub fn get_args(&self, str_arg: &str) -> Vec<String> {
        self.cs_args.lock().get_args(str_arg)
    }

    #[inline] pub fn get_arg(&self, 
        str_arg:     &str,
        str_default: &str) -> String {
        self.cs_args.lock().get_arg(str_arg,str_default)
    }

    #[inline] pub fn get_int_arg(&self, 
        str_arg:   &str,
        n_default: i64) -> i64 {
        self.cs_args.lock().get_int_arg(str_arg,n_default)
    }

    #[inline] pub fn get_bool_arg(&self, 
        str_arg: &str,
        default: bool) -> bool {
        self.cs_args.lock().get_bool_arg(str_arg,default)
    }

    #[inline] pub fn get_blocks_dir_path(&self) -> Box<Path> {
        self.cs_args.lock().get_blocks_dir_path()
    }

    #[inline] pub fn get_chain_name(&mut self) -> Result<String,StdException> {
        self.cs_args.lock().get_chain_name()
    }

    #[inline] pub fn get_command(&self) -> Option<ArgsManagerCommand> {
        self.cs_args.lock().get_command()
    }

    pub fn get_data_dir_base(&self) -> PathBuf {
        self.cs_args.lock().get_data_dir_base()
    }

    pub fn get_data_dir_net(&self) -> PathBuf {
        self.cs_args.lock().get_data_dir_net()
    }

    pub fn get_data_dir(&self, net_specific: bool) -> PathBuf {
        self.cs_args.lock().get_data_dir(net_specific)
    }

    pub fn get_help_message(&self) -> String {
        self.cs_args.lock().get_help_message()
    }

    pub fn get_settings_path(&self, 
        filepath: Option<&mut Box<Path>>,
        temp:     Option<bool>) -> bool {
        self.cs_args.lock().get_settings_path(filepath,temp)
    }

    pub fn get_setting(&self, arg: &str) -> SettingsValue {
        self.cs_args.lock().get_setting(arg)
    }

    pub fn get_settings_list(&self, arg: &str) -> Vec<SettingsValue> {
        self.cs_args.lock().get_settings_list(arg)
    }

    pub fn get_unrecognized_sections(&self) -> LinkedList<SectionInfo> {
        self.cs_args.lock().get_unrecognized_sections()
    }

    pub fn help_requested(&self) -> bool {
        self.cs_args.lock().help_requested()
    }

    pub fn setup_help_options(&mut self)  {
        self.cs_args.lock().setup_help_options()
    }

    pub fn log_args_prefix(&self, 
        prefix:  &str,
        section: &str,
        args:    &HashMap<String,Vec<SettingsValue>>)  {

        self.cs_args.lock().log_args_prefix(prefix,section,args)
    }

    pub fn log_args(&self)  {
        self.cs_args.lock().log_args()
    }

    pub fn parse_parameters(
        &mut self, 
        argv:  &Vec<String>,
        error: &mut String) -> bool {
        self.cs_args.lock().parse_parameters(argv,error)
    }

    pub fn is_arg_set(&self, str_arg: &str) -> bool {
        self.cs_args.lock().is_arg_set(str_arg)
    }

    pub fn is_arg_negated(&self, str_arg: &str) -> bool {
        self.cs_args.lock().is_arg_negated(str_arg)
    }

    pub fn read_config_stream<R: std::io::Read>(&mut self, 
        stream:              &mut std::io::BufReader<R>,
        filepath:            &str,
        error:               &mut String,
        ignore_invalid_keys: Option<bool>) -> bool {

        self.cs_args.lock()
            .read_config_stream(stream,filepath,error,ignore_invalid_keys)
    }

    pub fn read_config_files(&mut self, 
        error:               &mut String,
        ignore_invalid_keys: Option<bool>) -> bool {

        self.cs_args.lock()
            .read_config_files(error,ignore_invalid_keys)
    }

    pub fn select_config_network(&mut self, network: &str)  {
        self.cs_args.lock()
            .select_config_network(network)
    }

    pub fn soft_set_arg(&mut self, 
        str_arg:   &str,
        str_value: &str) -> bool {

        self.cs_args.lock()
            .soft_set_arg(str_arg, str_value)
    }

    pub fn force_set_arg(&mut self, 
        str_arg:   &str,
        str_value: &str)  {

        self.cs_args.lock()
            .force_set_arg(str_arg, str_value)
    }

    pub fn soft_set_bool_arg(&mut self, 
        str_arg: &str,
        value:   bool) -> bool {

        self.cs_args.lock()
            .soft_set_bool_arg(str_arg, value)
    }

    pub fn init_settings(&mut self, error: &mut String) -> Result<(),String> {

        self.cs_args.lock()
            .init_settings(error)
    }

    pub fn read_settings_file(&mut self, errors: Option<&mut Vec<String>>) -> bool {

        self.cs_args.lock()
            .read_settings_file(errors)
    }

    pub fn write_settings_file(&self, 
        mut errors: Option<&mut Vec<String>>) -> Result<bool,StdException> {

        self.cs_args.lock()
            .write_settings_file(errors)
    }

    pub fn use_default_section(&self, arg: &str) -> bool {

        self.cs_args.lock()
            .use_default_section(arg)

    }
}
