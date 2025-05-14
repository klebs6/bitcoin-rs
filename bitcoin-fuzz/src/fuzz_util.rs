// ---------------- [ File: bitcoin-fuzz/src/fuzz_util.rs ]
crate::ix!();

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/util.h]

pub fn call_one_of<Callables>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        callables:            Callables) -> usize {

    todo!();
        /*
            constexpr size_t call_size{sizeof...(callables)};
        const_assert(call_size >= 1);
        const size_t call_index{fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, call_size - 1)};

        size_t i{0};
        ((i++ == call_index ? callables() : c_void()), ...);
        return call_size;
        */
}

pub type Auto = i32;//TODO

pub fn pick_value<Collection>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        col:                  &mut Collection) -> &'static mut Auto {

    todo!();
        /*
            const auto sz = col.size();
        assert(sz >= 1);
        auto it = col.begin();
        std::advance(it, fuzzed_data_provider.ConsumeIntegralInRange<decltype(sz)>(0, sz - 1));
        return *it;
        */
}

#[inline] pub fn consume_random_length_byte_vector(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_length:           &Option<usize>) -> Vec<u8> {

    todo!();
        /*
            const std::string s = max_length ?
                                  fuzzed_data_provider.ConsumeRandomLengthString(*max_length) :
                                  fuzzed_data_provider.ConsumeRandomLengthString();
        return {s.begin(), s.end()};
        */
}

#[inline] pub fn consume_random_length_bit_vector(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_length:           &Option<usize>) -> Vec<bool> {

    todo!();
        /*
            return BytesToBits(ConsumeRandomLengthByteVector(fuzzed_data_provider, max_length));
        */
}

#[inline] pub fn consume_data_stream(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_length:           &Option<usize>) -> DataStream {

    todo!();
        /*
            return DataStream{ConsumeRandomLengthByteVector(fuzzed_data_provider, max_length), SER_NETWORK, INIT_PROTO_VERSION};
        */
}

#[inline] pub fn consume_random_length_string_vector(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_vector_size:      Option<usize>,
        max_string_length:    Option<usize>) -> Vec<String> {

    let max_vector_size:   usize = max_vector_size.unwrap_or(16);
    let max_string_length: usize = max_string_length.unwrap_or(16);

    todo!();
        /*
            const size_t n_elements = fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, max_vector_size);
        std::vector<std::string> r;
        for (size_t i = 0; i < n_elements; ++i) {
            r.push_back(fuzzed_data_provider.ConsumeRandomLengthString(max_string_length));
        }
        return r;
        */
}

#[inline] pub fn consume_random_length_integral_vector<T>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_vector_size:      Option<usize>) -> Vec<T> {

    let max_vector_size: usize = max_vector_size.unwrap_or(16);

    todo!();
        /*
            const size_t n_elements = fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, max_vector_size);
        std::vector<T> r;
        for (size_t i = 0; i < n_elements; ++i) {
            r.push_back(fuzzed_data_provider.ConsumeIntegral<T>());
        }
        return r;
        */
}

#[inline] pub fn consume_deserializable<T>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_length:           &Option<usize>) -> Option<T> {

    todo!();
        /*
            const std::vector<uint8_t> buffer = ConsumeRandomLengthByteVector(fuzzed_data_provider, max_length);
        DataStream ds{buffer, SER_NETWORK, INIT_PROTO_VERSION};
        T obj;
        try {
            ds >> obj;
        } catch (const std::ios_base::failure&) {
            return std::nullopt;
        }
        return obj;
        */
}

pub fn consume_weak_enum<WeakEnumType, const size: usize>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        all_types:            &[WeakEnumType; size]) -> WeakEnumType {

    todo!();
        /*
            return fuzzed_data_provider.ConsumeBool() ?
                   fuzzed_data_provider.PickValueInArray<WeakEnumType>(all_types) :
                   WeakEnumType(fuzzed_data_provider.ConsumeIntegral<typename std::underlying_type<WeakEnumType>::type>());
        */
}

#[inline] pub fn consume_opcode_type(fuzzed_data_provider: &mut FuzzedDataProvider) -> OpcodeType {
    
    todo!();
        /*
            return static_cast<opcodetype>(fuzzed_data_provider.ConsumeIntegralInRange<uint32_t>(0, MAX_OPCODE));
        */
}

#[inline] pub fn consume_script_num(fuzzed_data_provider: &mut FuzzedDataProvider) -> ScriptNum {
    
    todo!();
        /*
            return CScriptNum{fuzzed_data_provider.ConsumeIntegral<int64_t>()};
        */
}

#[inline] pub fn consume_uint160(fuzzed_data_provider: &mut FuzzedDataProvider) -> u160 {
    
    todo!();
        /*
            const std::vector<uint8_t> v160 = fuzzed_data_provider.ConsumeBytes<uint8_t>(160 / 8);
        if (v160.size() != 160 / 8) {
            return {};
        }
        return u160{v160};
        */
}

#[inline] pub fn consume_uint256(fuzzed_data_provider: &mut FuzzedDataProvider) -> u256 {
    
    todo!();
        /*
            const std::vector<uint8_t> v256 = fuzzed_data_provider.ConsumeBytes<uint8_t>(256 / 8);
        if (v256.size() != 256 / 8) {
            return {};
        }
        return uint256{v256};
        */
}

#[inline] pub fn consume_arith_uint256(fuzzed_data_provider: &mut FuzzedDataProvider) -> ArithU256 {
    
    todo!();
        /*
            return UintToArith256(ConsumeUInt256(fuzzed_data_provider));
        */
}

pub fn multiplication_overflow<T>(i: T, j: T) -> bool {

    todo!();
        /*
            const_assert(std::is_integral<T>::value, "Integral required.");
        if (std::numeric_limits<T>::is_signed) {
            if (i > 0) {
                if (j > 0) {
                    return i > (std::numeric_limits<T>::max() / j);
                } else {
                    return j < (std::numeric_limits<T>::min() / i);
                }
            } else {
                if (j > 0) {
                    return i < (std::numeric_limits<T>::min() / j);
                } else {
                    return i != 0 && (j < (std::numeric_limits<T>::max() / i));
                }
            }
        } else {
            return j != 0 && i > std::numeric_limits<T>::max() / j;
        }
        */
}

pub fn addition_overflow<T>(i: T, j: T) -> bool {

    todo!();
        /*
            const_assert(std::is_integral<T>::value, "Integral required.");
        if (std::numeric_limits<T>::is_signed) {
            return (i > 0 && j > std::numeric_limits<T>::max() - i) ||
                   (i < 0 && j < std::numeric_limits<T>::min() - i);
        }
        return std::numeric_limits<T>::max() - i < j;
        */
}

/**
  | Sets errno to a value selected from the
  | given std::array `errnos`.
  |
  */
pub fn set_fuzzed_errno_with_errnos<T, const SIZE: usize>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        errnos:               &[T;SIZE])  {

    todo!();
        /*
            errno = fuzzed_data_provider.PickValueInArray(errnos);
        */
}

/**
  | Sets a fuzzed errno in the range [0, 133
  | (EHWPOISON)]. Can be used from functions
  | emulating standard library functions
  | that set errno, or in other contexts
  | where the value of errno might be relevant
  | for the execution path that will be taken.
  |
  */
#[inline] pub fn set_fuzzed_errno(fuzzed_data_provider: &mut FuzzedDataProvider)  {
    
    todo!();
        /*
            errno = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, 133);
        */
}

/**
  | Returns a byte vector of specified size
  | regardless of the number of remaining
  | bytes available from the fuzzer. Pads
  | with zero value bytes if needed to achieve
  | the specified size.
  |
  */
#[inline] pub fn consume_fixed_length_byte_vector(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        length:               usize) -> Vec<u8> {
    
    todo!();
        /*
            std::vector<uint8_t> result(length);
        const std::vector<uint8_t> random_bytes = fuzzed_data_provider.ConsumeBytes<uint8_t>(length);
        if (!random_bytes.empty()) {
            std::memcpy(result.data(), random_bytes.data(), random_bytes.size());
        }
        return result;
        */
}

#[inline] pub fn consume_sub_net(fuzzed_data_provider: &mut FuzzedDataProvider) -> SubNet {
    
    todo!();
        /*
            return {ConsumeNetAddr(fuzzed_data_provider), fuzzed_data_provider.ConsumeIntegral<uint8_t>()};
        */
}

#[inline] pub fn consume_service(fuzzed_data_provider: &mut FuzzedDataProvider) -> Service {
    
    todo!();
        /*
            return {ConsumeNetAddr(fuzzed_data_provider), fuzzed_data_provider.ConsumeIntegral<uint16_t>()};
        */
}

#[inline] pub fn consume_address(fuzzed_data_provider: &mut FuzzedDataProvider) -> Address {
    
    todo!();
        /*
            return {ConsumeService(fuzzed_data_provider), ConsumeWeakEnum(fuzzed_data_provider, ALL_SERVICE_FLAGS), fuzzed_data_provider.ConsumeIntegral<uint32_t>()};
        */
}

pub fn consume_node<const ReturnUniquePtr: bool /* = false*/>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        node_id_in:           &Option<NodeId>) -> Auto {

    todo!();
        /*
            const NodeId node_id = node_id_in.value_or(fuzzed_data_provider.ConsumeIntegral<NodeId>());
        const ServiceFlags local_services = ConsumeWeakEnum(fuzzed_data_provider, ALL_SERVICE_FLAGS);
        const Socket socket = INVALID_SOCKET;
        const CAddress address = ConsumeAddress(fuzzed_data_provider);
        const uint64_t keyed_net_group = fuzzed_data_provider.ConsumeIntegral<uint64_t>();
        const uint64_t local_host_nonce = fuzzed_data_provider.ConsumeIntegral<uint64_t>();
        const CAddress addr_bind = ConsumeAddress(fuzzed_data_provider);
        const std::string addr_name = fuzzed_data_provider.ConsumeRandomLengthString(64);
        const ConnectionType conn_type = fuzzed_data_provider.PickValueInArray(ALL_CONNECTION_TYPES);
        const bool inbound_onion{conn_type == ConnectionType::INBOUND ? fuzzed_data_provider.ConsumeBool() : false};
        if constexpr (ReturnUniquePtr) {
            return std::make_unique<Node>(node_id, local_services, socket, address, keyed_net_group, local_host_nonce, addr_bind, addr_name, conn_type, inbound_onion);
        } else {
            return Node{node_id, local_services, socket, address, keyed_net_group, local_host_nonce, addr_bind, addr_name, conn_type, inbound_onion};
        }
        */
}

#[inline] pub fn consume_node_as_unique_ptr(
    fdp:        &mut FuzzedDataProvider,
    node_id_in: &Option<NodeId>) -> Box<Node> {

    todo!();
        /*
            return ConsumeNode<true>(fdp, node_id_in);
        */
}

///------------------
pub struct FuzzedFileProvider {
    fuzzed_data_provider: Rc<RefCell<FuzzedDataProvider>>,
    offset:               i64, // default = 0
}

impl FuzzedFileProvider {
    
    pub fn new(fuzzed_data_provider: &mut FuzzedDataProvider) -> Self {
    
        todo!();
        /*


            : m_fuzzed_data_provider{fuzzed_data_provider}
        */
    }
}

#[inline] pub fn consume_file(fuzzed_data_provider: &mut FuzzedDataProvider) -> FuzzedFileProvider {
    
    todo!();
        /*
            return {fuzzed_data_provider};
        */
}

pub struct FuzzedAutoFileProvider {
    fuzzed_data_provider: Rc<RefCell<FuzzedDataProvider>>,
    fuzzed_file_provider: FuzzedFileProvider,
}

impl FuzzedAutoFileProvider {

    pub fn new(fuzzed_data_provider: &mut FuzzedDataProvider) -> Self {
    
        todo!();
        /*
            : m_fuzzed_data_provider{fuzzed_data_provider}, m_fuzzed_file_provider{fuzzed_data_provider}
        */
    }
    
    pub fn open(&mut self) -> AutoFile {
        
        todo!();
        /*
            return {m_fuzzed_file_provider.open(), m_fuzzed_data_provider.ConsumeIntegral<int>(), m_fuzzed_data_provider.ConsumeIntegral<int>()};
        */
    }
}

#[inline] pub fn consume_auto_file(fuzzed_data_provider: &mut FuzzedDataProvider) -> FuzzedAutoFileProvider {
    
    todo!();
        /*
            return {fuzzed_data_provider};
        */
}

macro_rules! write_to_stream_case {
    ($type:ident, $consume:ident) => {
        /*
        
            [&] {                                   
                type o = consume;                   
                stream << o;                        
            }
        */
    }
}

pub fn write_to_stream<Stream>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        stream:               &mut Stream)  {

    todo!();
        /*
            while (fuzzed_data_provider.ConsumeBool()) {
            try {
                CallOneOf(
                    fuzzed_data_provider,
                    WRITE_TO_STREAM_CASE(bool, fuzzed_data_provider.ConsumeBool()),
                    WRITE_TO_STREAM_CASE(char, fuzzed_data_provider.ConsumeIntegral<char>()),
                    WRITE_TO_STREAM_CASE(int8_t, fuzzed_data_provider.ConsumeIntegral<int8_t>()),
                    WRITE_TO_STREAM_CASE(uint8_t, fuzzed_data_provider.ConsumeIntegral<uint8_t>()),
                    WRITE_TO_STREAM_CASE(int16_t, fuzzed_data_provider.ConsumeIntegral<int16_t>()),
                    WRITE_TO_STREAM_CASE(uint16_t, fuzzed_data_provider.ConsumeIntegral<uint16_t>()),
                    WRITE_TO_STREAM_CASE(int32_t, fuzzed_data_provider.ConsumeIntegral<int32_t>()),
                    WRITE_TO_STREAM_CASE(uint32_t, fuzzed_data_provider.ConsumeIntegral<uint32_t>()),
                    WRITE_TO_STREAM_CASE(int64_t, fuzzed_data_provider.ConsumeIntegral<int64_t>()),
                    WRITE_TO_STREAM_CASE(uint64_t, fuzzed_data_provider.ConsumeIntegral<uint64_t>()),
                    WRITE_TO_STREAM_CASE(std::string, fuzzed_data_provider.ConsumeRandomLengthString(32)),
                    WRITE_TO_STREAM_CASE(std::vector<char>, ConsumeRandomLengthIntegralVector<char>(fuzzed_data_provider)));
            } catch (const std::ios_base::failure&) {
                break;
            }
        }
        */
}

macro_rules! read_from_stream_case {
    ($type:ident) => {
        /*
        
            [&] {                           
                type o;                     
                stream >> o;                
            }
        */
    }
}

pub fn read_from_stream<Stream>(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        stream:               &mut Stream)  {

    todo!();
        /*
            while (fuzzed_data_provider.ConsumeBool()) {
            try {
                CallOneOf(
                    fuzzed_data_provider,
                    READ_FROM_STREAM_CASE(bool),
                    READ_FROM_STREAM_CASE(char),
                    READ_FROM_STREAM_CASE(int8_t),
                    READ_FROM_STREAM_CASE(uint8_t),
                    READ_FROM_STREAM_CASE(int16_t),
                    READ_FROM_STREAM_CASE(uint16_t),
                    READ_FROM_STREAM_CASE(int32_t),
                    READ_FROM_STREAM_CASE(uint32_t),
                    READ_FROM_STREAM_CASE(int64_t),
                    READ_FROM_STREAM_CASE(uint64_t),
                    READ_FROM_STREAM_CASE(std::string),
                    READ_FROM_STREAM_CASE(std::vector<char>));
            } catch (const std::ios_base::failure&) {
                break;
            }
        }
        */
}

pub struct FuzzedSock {

    base:                 Sock,

    fuzzed_data_provider: Rc<RefCell<FuzzedDataProvider>>,

    /**
      | Data to return when `MSG_PEEK` is used
      | as a `Recv()` flag.
      | 
      | If `MSG_PEEK` is used, then our `Recv()`
      | returns some random data as usual, but
      | on the next `Recv()` call we must return
      | the same data, thus we remember it here.
      |
      */
    peek_data:            RefCell<Option<u8>>,
}

#[inline] pub fn consume_sock(fuzzed_data_provider: &mut FuzzedDataProvider) -> FuzzedSock {
    
    todo!();
        /*
            return FuzzedSock{fuzzed_data_provider};
        */
}

//-------------------------------------------[.cpp/bitcoin/src/test/fuzz/util.cpp]
impl Drop for FuzzedSock {
    fn drop(&mut self) {
        todo!();
        /*
            // Sock::~Sock() will be called after FuzzedSock::~FuzzedSock() and it will call
        // Sock::Reset() (not FuzzedSock::Reset()!) which will call CloseSocket(m_socket).
        // Avoid closing an arbitrary file descriptor (m_socket is just a random very high number which
        // theoretically may concide with a real opened file descriptor).
        Reset();
        */
    }
}

impl FuzzedSock {

    pub fn new(fuzzed_data_provider: &mut FuzzedDataProvider) -> Self {
    
        todo!();
        /*


            : m_fuzzed_data_provider{fuzzed_data_provider}

        m_socket = fuzzed_data_provider.ConsumeIntegralInRange<Socket>(INVALID_SOCKET - 1, INVALID_SOCKET);
        */
    }
    
    pub fn assign_from(&mut self, other: Sock) -> &mut FuzzedSock {
        
        todo!();
        /*
            assert(false && "Move of Sock into FuzzedSock not allowed.");
        return *this;
        */
    }
    
    pub fn reset(&mut self)  {
        
        todo!();
        /*
            m_socket = INVALID_SOCKET;
        */
    }
    
    pub fn send(&self, 
        data:  *const c_void,
        len:   usize,
        flags: i32) -> isize {
        
        todo!();
        /*
            constexpr std::array send_errnos{
            EACCES,
            EAGAIN,
            EALREADY,
            EBADF,
            ECONNRESET,
            EDESTADDRREQ,
            EFAULT,
            EINTR,
            EINVAL,
            EISCONN,
            EMSGSIZE,
            ENOBUFS,
            ENOMEM,
            ENOTCONN,
            ENOTSOCK,
            EOPNOTSUPP,
            EPIPE,
            EWOULDBLOCK,
        };
        if (m_fuzzed_data_provider.ConsumeBool()) {
            return len;
        }
        const ssize_t r = m_fuzzed_data_provider.ConsumeIntegralInRange<ssize_t>(-1, len);
        if (r == -1) {
            SetFuzzedErrNo(m_fuzzed_data_provider, send_errnos);
        }
        return r;
        */
    }
    
    pub fn recv(&self, 
        buf:   *mut c_void,
        len:   usize,
        flags: i32) -> isize {
        
        todo!();
        /*
            // Have a permanent error at recv_errnos[0] because when the fuzzed data is exhausted
        // SetFuzzedErrNo() will always return the first element and we want to avoid Recv()
        // returning -1 and setting errno to EAGAIN repeatedly.
        constexpr std::array recv_errnos{
            ECONNREFUSED,
            EAGAIN,
            EBADF,
            EFAULT,
            EINTR,
            EINVAL,
            ENOMEM,
            ENOTCONN,
            ENOTSOCK,
            EWOULDBLOCK,
        };
        assert(buf != nullptr || len == 0);
        if (len == 0 || m_fuzzed_data_provider.ConsumeBool()) {
            const ssize_t r = m_fuzzed_data_provider.ConsumeBool() ? 0 : -1;
            if (r == -1) {
                SetFuzzedErrNo(m_fuzzed_data_provider, recv_errnos);
            }
            return r;
        }
        std::vector<uint8_t> random_bytes;
        bool pad_to_len_bytes{m_fuzzed_data_provider.ConsumeBool()};
        if (m_peek_data.has_value()) {
            // `MSG_PEEK` was used in the preceding `Recv()` call, return `m_peek_data`.
            random_bytes.assign({m_peek_data.value()});
            if ((flags & MSG_PEEK) == 0) {
                m_peek_data.reset();
            }
            pad_to_len_bytes = false;
        } else if ((flags & MSG_PEEK) != 0) {
            // New call with `MSG_PEEK`.
            random_bytes = m_fuzzed_data_provider.ConsumeBytes<uint8_t>(1);
            if (!random_bytes.empty()) {
                m_peek_data = random_bytes[0];
                pad_to_len_bytes = false;
            }
        } else {
            random_bytes = m_fuzzed_data_provider.ConsumeBytes<uint8_t>(
                m_fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, len));
        }
        if (random_bytes.empty()) {
            const ssize_t r = m_fuzzed_data_provider.ConsumeBool() ? 0 : -1;
            if (r == -1) {
                SetFuzzedErrNo(m_fuzzed_data_provider, recv_errnos);
            }
            return r;
        }
        std::memcpy(buf, random_bytes.data(), random_bytes.size());
        if (pad_to_len_bytes) {
            if (len > random_bytes.size()) {
                std::memset((char*)buf + random_bytes.size(), 0, len - random_bytes.size());
            }
            return len;
        }
        if (m_fuzzed_data_provider.ConsumeBool() && std::getenv("FUZZED_SOCKET_FAKE_LATENCY") != nullptr) {
            std::this_thread::sleep_for(milliseconds{2});
        }
        return random_bytes.size();
        */
    }
    
    pub fn connect(&self, 
        _0: *const std::net::SocketAddr,
        _1: libc::socklen_t) -> i32 {
        
        todo!();
        /*
            // Have a permanent error at connect_errnos[0] because when the fuzzed data is exhausted
        // SetFuzzedErrNo() will always return the first element and we want to avoid Connect()
        // returning -1 and setting errno to EAGAIN repeatedly.
        constexpr std::array connect_errnos{
            ECONNREFUSED,
            EAGAIN,
            ECONNRESET,
            EHOSTUNREACH,
            EINPROGRESS,
            EINTR,
            ENETUNREACH,
            ETIMEDOUT,
        };
        if (m_fuzzed_data_provider.ConsumeBool()) {
            SetFuzzedErrNo(m_fuzzed_data_provider, connect_errnos);
            return -1;
        }
        return 0;
        */
    }
    
    pub fn get_sock_opt(&self, 
        level:    i32,
        opt_name: i32,
        opt_val:  *mut c_void,
        opt_len:  *mut libc::socklen_t) -> i32 {
        
        todo!();
        /*
            constexpr std::array getsockopt_errnos{
            ENOMEM,
            ENOBUFS,
        };
        if (m_fuzzed_data_provider.ConsumeBool()) {
            SetFuzzedErrNo(m_fuzzed_data_provider, getsockopt_errnos);
            return -1;
        }
        if (opt_val == nullptr) {
            return 0;
        }
        std::memcpy(opt_val,
                    ConsumeFixedLengthByteVector(m_fuzzed_data_provider, *opt_len).data(),
                    *opt_len);
        return 0;
        */
    }
    
    pub fn wait(&self, 

        //milliseconds
        timeout:   Duration, 

        requested: libevent_sys::event,
        occurred:  Option<*mut libevent_sys::event>) -> bool {
        
        todo!();
        /*
            constexpr std::array wait_errnos{
            EBADF,
            EINTR,
            EINVAL,
        };
        if (m_fuzzed_data_provider.ConsumeBool()) {
            SetFuzzedErrNo(m_fuzzed_data_provider, wait_errnos);
            return false;
        }
        if (occurred != nullptr) {
            *occurred = m_fuzzed_data_provider.ConsumeBool() ? requested : 0;
        }
        return true;
        */
    }
    
    pub fn is_connected(&self, errmsg: &mut String) -> bool {
        
        todo!();
        /*
            if (m_fuzzed_data_provider.ConsumeBool()) {
            return true;
        }
        errmsg = "disconnected at random by the fuzzer";
        return false;
        */
    }
}

pub fn fill_node(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        node:                 &mut Node,
        init_version:         bool)  {
    
    todo!();
        /*
            const ServiceFlags remote_services = ConsumeWeakEnum(fuzzed_data_provider, ALL_SERVICE_FLAGS);
        const NetPermissionFlags permission_flags = ConsumeWeakEnum(fuzzed_data_provider, ALL_NET_PERMISSION_FLAGS);
        const int32_t version = fuzzed_data_provider.ConsumeIntegralInRange<int32_t>(MIN_PEER_PROTO_VERSION, std::numeric_limits<int32_t>::max());
        const bool filter_txs = fuzzed_data_provider.ConsumeBool();

        node.nServices = remote_services;
        node.m_permissionFlags = permission_flags;
        if (init_version) {
            node.nVersion = version;
            node.SetCommonVersion(std::min(version, PROTOCOL_VERSION));
        }
        if (node.m_tx_relay != nullptr) {
            LOCK(node.m_tx_relay->cs_filter);
            node.m_tx_relay->fRelayTxes = filter_txs;
        }
        */
}

pub fn consume_money(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max:                  &Option<Amount>) -> Amount {
    
    todo!();
        /*
            return fuzzed_data_provider.ConsumeIntegralInRange<CAmount>(0, max.value_or(MAX_MONEY));
        */
}

pub fn consume_time(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        min:                  &Option<i64>,
        max:                  &Option<i64>) -> i64 {

    todo!();
        /*
            // Avoid t=0 (1970-01-01T00:00:00Z) since SetMockTime(0) disables mocktime.
        static const int64_t time_min = ParseISO8601DateTime("1970-01-01T00:00:01Z");
        static const int64_t time_max = ParseISO8601DateTime("9999-12-31T23:59:59Z");
        return fuzzed_data_provider.ConsumeIntegralInRange<int64_t>(min.value_or(time_min), max.value_or(time_max));
        */
}

pub fn consume_transaction(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        prevout_txids:        &Option<Vec<u256>>,
        max_num_in:           Option<i32>,
        max_num_out:          Option<i32>) -> MutableTransaction {

    let max_num_in:  i32 = max_num_in.unwrap_or(10);
    let max_num_out: i32 = max_num_out.unwrap_or(10);
    
    todo!();
        /*
            CMutableTransaction tx_mut;
        const auto p2wsh_op_true = fuzzed_data_provider.ConsumeBool();
        tx_mut.nVersion = fuzzed_data_provider.ConsumeBool() ?
                              CTransaction::CURRENT_VERSION :
                              fuzzed_data_provider.ConsumeIntegral<int32_t>();
        tx_mut.nLockTime = fuzzed_data_provider.ConsumeIntegral<uint32_t>();
        const auto num_in = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, max_num_in);
        const auto num_out = fuzzed_data_provider.ConsumeIntegralInRange<int>(0, max_num_out);
        for (int i = 0; i < num_in; ++i) {
            const auto& txid_prev = prevout_txids ?
                                        PickValue(fuzzed_data_provider, *prevout_txids) :
                                        ConsumeUInt256(fuzzed_data_provider);
            const auto index_out = fuzzed_data_provider.ConsumeIntegralInRange<uint32_t>(0, max_num_out);
            const auto sequence = ConsumeSequence(fuzzed_data_provider);
            const auto script_sig = p2wsh_op_true ? CScript{} : ConsumeScript(fuzzed_data_provider);
            CScriptWitness script_wit;
            if (p2wsh_op_true) {
                script_wit.stack = std::vector<std::vector<uint8_t>>{WITNESS_STACK_ELEM_OP_TRUE};
            } else {
                script_wit = ConsumeScriptWitness(fuzzed_data_provider);
            }
            CTxIn in;
            in.prevout = OutPoint{txid_prev, index_out};
            in.nSequence = sequence;
            in.scriptSig = script_sig;
            in.scriptWitness = script_wit;

            tx_mut.vin.push_back(in);
        }
        for (int i = 0; i < num_out; ++i) {
            const auto amount = fuzzed_data_provider.ConsumeIntegralInRange<CAmount>(-10, 50 * COIN + 10);
            const auto script_pk = p2wsh_op_true ?
                                       P2WSH_OP_TRUE :
                                       ConsumeScript(fuzzed_data_provider, /* max_length */ 128, /* maybe_p2wsh */ true);
            tx_mut.vout.emplace_back(amount, script_pk);
        }
        return tx_mut;
        */
}

pub fn consume_script_witness(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_stack_elem_size:  Option<usize>) -> ScriptWitness {

    let max_stack_elem_size: usize = max_stack_elem_size.unwrap_or(32);
    
    todo!();
        /*
            CScriptWitness ret;
        const auto n_elements = fuzzed_data_provider.ConsumeIntegralInRange<size_t>(0, max_stack_elem_size);
        for (size_t i = 0; i < n_elements; ++i) {
            ret.stack.push_back(ConsumeRandomLengthByteVector(fuzzed_data_provider));
        }
        return ret;
        */
}

pub fn consume_script(
        fuzzed_data_provider: &mut FuzzedDataProvider,
        max_length:           Option<usize>,
        maybe_p2wsh:          Option<bool>) -> Script {

    let maybe_p2wsh: bool = maybe_p2wsh.unwrap_or(false);
    
    todo!();
        /*
            const std::vector<uint8_t> b = ConsumeRandomLengthByteVector(fuzzed_data_provider, max_length);
        CScript r_script{b.begin(), b.end()};
        if (maybe_p2wsh && fuzzed_data_provider.ConsumeBool()) {
            uint256 script_hash;
            CSHA256().Write(r_script.data(), r_script.size()).Finalize(script_hash.begin());
            r_script.clear();
            r_script << OP_0 << ToByteVector(script_hash);
        }
        return r_script;
        */
}

pub fn consume_sequence(fuzzed_data_provider: &mut FuzzedDataProvider) -> u32 {
    
    todo!();
        /*
            return fuzzed_data_provider.ConsumeBool() ?
                   fuzzed_data_provider.PickValueInArray({
                       CTxIn::SEQUENCE_FINAL,
                       CTxIn::SEQUENCE_FINAL - 1,
                       MAX_BIP125_RBF_SEQUENCE,
                   }) :
                   fuzzed_data_provider.ConsumeIntegral<uint32_t>();
        */
}

pub fn consume_tx_destination(fuzzed_data_provider: &mut FuzzedDataProvider) -> TxDestination {
    
    todo!();
        /*
            TxDestination tx_destination;
        const size_t call_size{CallOneOf(
            fuzzed_data_provider,
            [&] {
                tx_destination = CNoDestination{};
            },
            [&] {
                tx_destination = PKHash{ConsumeUInt160(fuzzed_data_provider)};
            },
            [&] {
                tx_destination = ScriptHash{ConsumeUInt160(fuzzed_data_provider)};
            },
            [&] {
                tx_destination = WitnessV0ScriptHash{ConsumeUInt256(fuzzed_data_provider)};
            },
            [&] {
                tx_destination = WitnessV0KeyHash{ConsumeUInt160(fuzzed_data_provider)};
            },
            [&] {
                tx_destination = WitnessV1Taproot{XOnlyPubKey{ConsumeUInt256(fuzzed_data_provider)}};
            },
            [&] {
                WitnessUnknown witness_unknown{};
                witness_unknown.version = fuzzed_data_provider.ConsumeIntegralInRange(2, 16);
                std::vector<uint8_t> witness_unknown_program_1{fuzzed_data_provider.ConsumeBytes<uint8_t>(40)};
                if (witness_unknown_program_1.size() < 2) {
                    witness_unknown_program_1 = {0, 0};
                }
                witness_unknown.length = witness_unknown_program_1.size();
                std::copy(witness_unknown_program_1.begin(), witness_unknown_program_1.end(), witness_unknown.program);
                tx_destination = witness_unknown;
            })};
        Assert(call_size == std::variant_size_v<TxDestination>);
        return tx_destination;
        */
}

pub fn consume_tx_mem_pool_entry(
    fuzzed_data_provider: &mut FuzzedDataProvider,
    tx:                   &Transaction) -> TxMemPoolEntry {
    
    todo!();
        /*
            // Avoid:
        // policy/feerate.cpp:28:34: runtime error: signed integer overflow: 34873208148477500 * 1000 cannot be represented in type 'long'
        //
        // Reproduce using CFeeRate(348732081484775, 10).GetFeePerK()
        const CAmount fee = std::min<CAmount>(ConsumeMoney(fuzzed_data_provider), std::numeric_limits<CAmount>::max() / static_cast<CAmount>(100000));
        assert(MoneyRange(fee));
        const int64_t time = fuzzed_data_provider.ConsumeIntegral<int64_t>();
        const unsigned int entry_height = fuzzed_data_provider.ConsumeIntegral<unsigned int>();
        const bool spends_coinbase = fuzzed_data_provider.ConsumeBool();
        const unsigned int sig_op_cost = fuzzed_data_provider.ConsumeIntegralInRange<unsigned int>(0, MAX_BLOCK_SIGOPS_COST);
        return CTxMemPoolEntry{MakeTransactionRef(tx), fee, time, entry_height, spends_coinbase, sig_op_cost, {}};
        */
}

pub fn contains_spent_input(
    tx:     &Transaction,
    inputs: &CoinsViewCache) -> bool {
    
    todo!();
        /*
            for (const CTxIn& tx_in : tx.vin) {
            const Coin& coin = inputs.AccessCoin(tx_in.prevout);
            if (coin.IsSpent()) {
                return true;
            }
        }
        return false;
        */
}

pub fn consume_net_addr(fuzzed_data_provider: &mut FuzzedDataProvider) -> NetAddr {
    
    todo!();
        /*
            const Network network = fuzzed_data_provider.PickValueInArray({Network::NET_IPV4, Network::NET_IPV6, Network::NET_INTERNAL, Network::NET_ONION});
        CNetAddr net_addr;
        if (network == Network::NET_IPV4) {
            in_addr v4_addr = {};
            v4_addr.s_addr = fuzzed_data_provider.ConsumeIntegral<uint32_t>();
            net_addr = CNetAddr{v4_addr};
        } else if (network == Network::NET_IPV6) {
            if (fuzzed_data_provider.remaining_bytes() >= 16) {
                in6_addr v6_addr = {};
                memcpy(v6_addr.s6_addr, fuzzed_data_provider.ConsumeBytes<uint8_t>(16).data(), 16);
                net_addr = CNetAddr{v6_addr, fuzzed_data_provider.ConsumeIntegral<uint32_t>()};
            }
        } else if (network == Network::NET_INTERNAL) {
            net_addr.SetInternal(fuzzed_data_provider.ConsumeBytesAsString(32));
        } else if (network == Network::NET_ONION) {
            net_addr.SetSpecial(fuzzed_data_provider.ConsumeBytesAsString(32));
        }
        return net_addr;
        */
}

impl FuzzedFileProvider {
    
    pub fn open(&mut self) -> *mut libc::FILE {
        
        todo!();
        /*
            SetFuzzedErrNo(m_fuzzed_data_provider);
        if (m_fuzzed_data_provider.ConsumeBool()) {
            return nullptr;
        }
        std::string mode;
        CallOneOf(
            m_fuzzed_data_provider,
            [&] {
                mode = "r";
            },
            [&] {
                mode = "r+";
            },
            [&] {
                mode = "w";
            },
            [&] {
                mode = "w+";
            },
            [&] {
                mode = "a";
            },
            [&] {
                mode = "a+";
            });
    #if defined _GNU_SOURCE && !defined __ANDROID__
        const cookie_io_functions_t io_hooks = {
            FuzzedFileProvider::read,
            FuzzedFileProvider::write,
            FuzzedFileProvider::seek,
            FuzzedFileProvider::close,
        };
        return fopencookie(this, mode.c_str(), io_hooks);
    #else
        (c_void)mode;
        return nullptr;
    #endif
        */
    }
    
    pub fn read(&mut self, 
        cookie: *mut c_void,
        buf:    *mut u8,
        size:   usize) -> isize {
        
        todo!();
        /*
            FuzzedFileProvider* fuzzed_file = (FuzzedFileProvider*)cookie;
        SetFuzzedErrNo(fuzzed_file->m_fuzzed_data_provider);
        if (buf == nullptr || size == 0 || fuzzed_file->m_fuzzed_data_provider.ConsumeBool()) {
            return fuzzed_file->m_fuzzed_data_provider.ConsumeBool() ? 0 : -1;
        }
        const std::vector<uint8_t> random_bytes = fuzzed_file->m_fuzzed_data_provider.ConsumeBytes<uint8_t>(size);
        if (random_bytes.empty()) {
            return 0;
        }
        std::memcpy(buf, random_bytes.data(), random_bytes.size());
        if (AdditionOverflow(fuzzed_file->m_offset, (int64_t)random_bytes.size())) {
            return fuzzed_file->m_fuzzed_data_provider.ConsumeBool() ? 0 : -1;
        }
        fuzzed_file->m_offset += random_bytes.size();
        return random_bytes.size();
        */
    }
    
    pub fn write(&mut self, 
        cookie: *mut c_void,
        buf:    *const u8,
        size:   usize) -> isize {
        
        todo!();
        /*
            FuzzedFileProvider* fuzzed_file = (FuzzedFileProvider*)cookie;
        SetFuzzedErrNo(fuzzed_file->m_fuzzed_data_provider);
        const ssize_t n = fuzzed_file->m_fuzzed_data_provider.ConsumeIntegralInRange<ssize_t>(0, size);
        if (AdditionOverflow(fuzzed_file->m_offset, (int64_t)n)) {
            return fuzzed_file->m_fuzzed_data_provider.ConsumeBool() ? 0 : -1;
        }
        fuzzed_file->m_offset += n;
        return n;
        */
    }
    
    pub fn seek(&mut self, 
        cookie: *mut c_void,
        offset: *mut i64,
        whence: i32) -> i32 {
        
        todo!();
        /*
            assert(whence == SEEK_SET || whence == SEEK_CUR || whence == SEEK_END);
        FuzzedFileProvider* fuzzed_file = (FuzzedFileProvider*)cookie;
        SetFuzzedErrNo(fuzzed_file->m_fuzzed_data_provider);
        int64_t new_offset = 0;
        if (whence == SEEK_SET) {
            new_offset = *offset;
        } else if (whence == SEEK_CUR) {
            if (AdditionOverflow(fuzzed_file->m_offset, *offset)) {
                return -1;
            }
            new_offset = fuzzed_file->m_offset + *offset;
        } else if (whence == SEEK_END) {
            const int64_t n = fuzzed_file->m_fuzzed_data_provider.ConsumeIntegralInRange<int64_t>(0, 4096);
            if (AdditionOverflow(n, *offset)) {
                return -1;
            }
            new_offset = n + *offset;
        }
        if (new_offset < 0) {
            return -1;
        }
        fuzzed_file->m_offset = new_offset;
        *offset = new_offset;
        return fuzzed_file->m_fuzzed_data_provider.ConsumeIntegralInRange<int>(-1, 0);
        */
    }
    
    pub fn close(&mut self, cookie: *mut c_void) -> i32 {
        
        todo!();
        /*
            FuzzedFileProvider* fuzzed_file = (FuzzedFileProvider*)cookie;
        SetFuzzedErrNo(fuzzed_file->m_fuzzed_data_provider);
        return fuzzed_file->m_fuzzed_data_provider.ConsumeIntegralInRange<int>(-1, 0);
        */
    }
}
