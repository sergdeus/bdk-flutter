use super::*;
// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_create_blockchain__static_method__Api(
    port_: i64,
    config: *mut wire_BlockchainConfig,
) {
    wire_create_blockchain__static_method__Api_impl(port_, config)
}

#[no_mangle]
pub extern "C" fn wire_get_height__static_method__Api(
    port_: i64,
    blockchain: wire_BlockchainInstance,
) {
    wire_get_height__static_method__Api_impl(port_, blockchain)
}

#[no_mangle]
pub extern "C" fn wire_get_blockchain_hash__static_method__Api(
    port_: i64,
    blockchain_height: u32,
    blockchain: wire_BlockchainInstance,
) {
    wire_get_blockchain_hash__static_method__Api_impl(port_, blockchain_height, blockchain)
}

#[no_mangle]
pub extern "C" fn wire_estimate_fee__static_method__Api(
    port_: i64,
    target: u64,
    blockchain: wire_BlockchainInstance,
) {
    wire_estimate_fee__static_method__Api_impl(port_, target, blockchain)
}

#[no_mangle]
pub extern "C" fn wire_broadcast__static_method__Api(
    port_: i64,
    tx: *mut wire_uint_8_list,
    blockchain: wire_BlockchainInstance,
) {
    wire_broadcast__static_method__Api_impl(port_, tx, blockchain)
}

#[no_mangle]
pub extern "C" fn wire_create_transaction__static_method__Api(
    port_: i64,
    tx: *mut wire_uint_8_list,
) {
    wire_create_transaction__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_tx_txid__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_tx_txid__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_weight__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_weight__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_size__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_size__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_vsize__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_vsize__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_serialize_tx__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_serialize_tx__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_is_coin_base__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_is_coin_base__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_is_explicitly_rbf__static_method__Api(
    port_: i64,
    tx: *mut wire_uint_8_list,
) {
    wire_is_explicitly_rbf__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_is_lock_time_enabled__static_method__Api(
    port_: i64,
    tx: *mut wire_uint_8_list,
) {
    wire_is_lock_time_enabled__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_version__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_version__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_lock_time__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_lock_time__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_input__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_input__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_output__static_method__Api(port_: i64, tx: *mut wire_uint_8_list) {
    wire_output__static_method__Api_impl(port_, tx)
}

#[no_mangle]
pub extern "C" fn wire_serialize_psbt__static_method__Api(
    port_: i64,
    psbt_str: *mut wire_uint_8_list,
) {
    wire_serialize_psbt__static_method__Api_impl(port_, psbt_str)
}

#[no_mangle]
pub extern "C" fn wire_psbt_txid__static_method__Api(port_: i64, psbt_str: *mut wire_uint_8_list) {
    wire_psbt_txid__static_method__Api_impl(port_, psbt_str)
}

#[no_mangle]
pub extern "C" fn wire_extract_tx__static_method__Api(port_: i64, psbt_str: *mut wire_uint_8_list) {
    wire_extract_tx__static_method__Api_impl(port_, psbt_str)
}

#[no_mangle]
pub extern "C" fn wire_psbt_fee_rate__static_method__Api(
    port_: i64,
    psbt_str: *mut wire_uint_8_list,
) {
    wire_psbt_fee_rate__static_method__Api_impl(port_, psbt_str)
}

#[no_mangle]
pub extern "C" fn wire_psbt_fee_amount__static_method__Api(
    port_: i64,
    psbt_str: *mut wire_uint_8_list,
) {
    wire_psbt_fee_amount__static_method__Api_impl(port_, psbt_str)
}

#[no_mangle]
pub extern "C" fn wire_combine_psbt__static_method__Api(
    port_: i64,
    psbt_str: *mut wire_uint_8_list,
    other: *mut wire_uint_8_list,
) {
    wire_combine_psbt__static_method__Api_impl(port_, psbt_str, other)
}

#[no_mangle]
pub extern "C" fn wire_json_serialize__static_method__Api(
    port_: i64,
    psbt_str: *mut wire_uint_8_list,
) {
    wire_json_serialize__static_method__Api_impl(port_, psbt_str)
}

#[no_mangle]
pub extern "C" fn wire_tx_builder_finish__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    recipients: *mut wire_list_script_amount,
    utxos: *mut wire_list_out_point,
    unspendable: *mut wire_list_out_point,
    change_policy: i32,
    manually_selected_only: bool,
    fee_rate: *mut f32,
    fee_absolute: *mut u64,
    drain_wallet: bool,
    drain_to: *mut wire_Script,
    rbf: *mut wire_RbfValue,
    data: *mut wire_uint_8_list,
) {
    wire_tx_builder_finish__static_method__Api_impl(
        port_,
        wallet,
        recipients,
        utxos,
        unspendable,
        change_policy,
        manually_selected_only,
        fee_rate,
        fee_absolute,
        drain_wallet,
        drain_to,
        rbf,
        data,
    )
}

#[no_mangle]
pub extern "C" fn wire_bump_fee_tx_builder_finish__static_method__Api(
    port_: i64,
    txid: *mut wire_uint_8_list,
    fee_rate: f32,
    allow_shrinking: *mut wire_uint_8_list,
    wallet: wire_WalletInstance,
    enable_rbf: bool,
    n_sequence: *mut u32,
) {
    wire_bump_fee_tx_builder_finish__static_method__Api_impl(
        port_,
        txid,
        fee_rate,
        allow_shrinking,
        wallet,
        enable_rbf,
        n_sequence,
    )
}

#[no_mangle]
pub extern "C" fn wire_create_descriptor__static_method__Api(
    port_: i64,
    descriptor: *mut wire_uint_8_list,
    network: i32,
) {
    wire_create_descriptor__static_method__Api_impl(port_, descriptor, network)
}

#[no_mangle]
pub extern "C" fn wire_new_bip44_descriptor__static_method__Api(
    port_: i64,
    key_chain_kind: i32,
    secret_key: *mut wire_uint_8_list,
    network: i32,
) {
    wire_new_bip44_descriptor__static_method__Api_impl(port_, key_chain_kind, secret_key, network)
}

#[no_mangle]
pub extern "C" fn wire_new_bip44_public__static_method__Api(
    port_: i64,
    key_chain_kind: i32,
    public_key: *mut wire_uint_8_list,
    network: i32,
    fingerprint: *mut wire_uint_8_list,
) {
    wire_new_bip44_public__static_method__Api_impl(
        port_,
        key_chain_kind,
        public_key,
        network,
        fingerprint,
    )
}

#[no_mangle]
pub extern "C" fn wire_new_bip49_descriptor__static_method__Api(
    port_: i64,
    key_chain_kind: i32,
    secret_key: *mut wire_uint_8_list,
    network: i32,
) {
    wire_new_bip49_descriptor__static_method__Api_impl(port_, key_chain_kind, secret_key, network)
}

#[no_mangle]
pub extern "C" fn wire_new_bip49_public__static_method__Api(
    port_: i64,
    key_chain_kind: i32,
    public_key: *mut wire_uint_8_list,
    network: i32,
    fingerprint: *mut wire_uint_8_list,
) {
    wire_new_bip49_public__static_method__Api_impl(
        port_,
        key_chain_kind,
        public_key,
        network,
        fingerprint,
    )
}

#[no_mangle]
pub extern "C" fn wire_new_bip84_descriptor__static_method__Api(
    port_: i64,
    key_chain_kind: i32,
    secret_key: *mut wire_uint_8_list,
    network: i32,
) {
    wire_new_bip84_descriptor__static_method__Api_impl(port_, key_chain_kind, secret_key, network)
}

#[no_mangle]
pub extern "C" fn wire_new_bip84_public__static_method__Api(
    port_: i64,
    key_chain_kind: i32,
    public_key: *mut wire_uint_8_list,
    network: i32,
    fingerprint: *mut wire_uint_8_list,
) {
    wire_new_bip84_public__static_method__Api_impl(
        port_,
        key_chain_kind,
        public_key,
        network,
        fingerprint,
    )
}

#[no_mangle]
pub extern "C" fn wire_as_string_private__static_method__Api(
    port_: i64,
    descriptor: wire_BdkDescriptor,
) {
    wire_as_string_private__static_method__Api_impl(port_, descriptor)
}

#[no_mangle]
pub extern "C" fn wire_as_string__static_method__Api(port_: i64, descriptor: wire_BdkDescriptor) {
    wire_as_string__static_method__Api_impl(port_, descriptor)
}

#[no_mangle]
pub extern "C" fn wire_create_descriptor_secret__static_method__Api(
    port_: i64,
    network: i32,
    mnemonic: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
) {
    wire_create_descriptor_secret__static_method__Api_impl(port_, network, mnemonic, password)
}

#[no_mangle]
pub extern "C" fn wire_descriptor_secret_from_string__static_method__Api(
    port_: i64,
    secret: *mut wire_uint_8_list,
) {
    wire_descriptor_secret_from_string__static_method__Api_impl(port_, secret)
}

#[no_mangle]
pub extern "C" fn wire_extend_descriptor_secret__static_method__Api(
    port_: i64,
    secret: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
) {
    wire_extend_descriptor_secret__static_method__Api_impl(port_, secret, path)
}

#[no_mangle]
pub extern "C" fn wire_derive_descriptor_secret__static_method__Api(
    port_: i64,
    secret: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
) {
    wire_derive_descriptor_secret__static_method__Api_impl(port_, secret, path)
}

#[no_mangle]
pub extern "C" fn wire_as_secret_bytes__static_method__Api(
    port_: i64,
    secret: *mut wire_uint_8_list,
) {
    wire_as_secret_bytes__static_method__Api_impl(port_, secret)
}

#[no_mangle]
pub extern "C" fn wire_as_public__static_method__Api(port_: i64, secret: *mut wire_uint_8_list) {
    wire_as_public__static_method__Api_impl(port_, secret)
}

#[no_mangle]
pub extern "C" fn wire_create_derivation_path__static_method__Api(
    port_: i64,
    path: *mut wire_uint_8_list,
) {
    wire_create_derivation_path__static_method__Api_impl(port_, path)
}

#[no_mangle]
pub extern "C" fn wire_descriptor_public_from_string__static_method__Api(
    port_: i64,
    public_key: *mut wire_uint_8_list,
) {
    wire_descriptor_public_from_string__static_method__Api_impl(port_, public_key)
}

#[no_mangle]
pub extern "C" fn wire_create_descriptor_public__static_method__Api(
    port_: i64,
    xpub: *mut wire_uint_8_list,
    path: *mut wire_uint_8_list,
    derive: bool,
) {
    wire_create_descriptor_public__static_method__Api_impl(port_, xpub, path, derive)
}

#[no_mangle]
pub extern "C" fn wire_create_script__static_method__Api(
    port_: i64,
    raw_output_script: *mut wire_uint_8_list,
) {
    wire_create_script__static_method__Api_impl(port_, raw_output_script)
}

#[no_mangle]
pub extern "C" fn wire_create_address__static_method__Api(
    port_: i64,
    address: *mut wire_uint_8_list,
) {
    wire_create_address__static_method__Api_impl(port_, address)
}

#[no_mangle]
pub extern "C" fn wire_address_from_script__static_method__Api(
    port_: i64,
    script: *mut wire_Script,
    network: i32,
) {
    wire_address_from_script__static_method__Api_impl(port_, script, network)
}

#[no_mangle]
pub extern "C" fn wire_address_to_script_pubkey__static_method__Api(
    port_: i64,
    address: *mut wire_uint_8_list,
) {
    wire_address_to_script_pubkey__static_method__Api_impl(port_, address)
}

#[no_mangle]
pub extern "C" fn wire_payload__static_method__Api(port_: i64, address: *mut wire_uint_8_list) {
    wire_payload__static_method__Api_impl(port_, address)
}

#[no_mangle]
pub extern "C" fn wire_address_network__static_method__Api(
    port_: i64,
    address: *mut wire_uint_8_list,
) {
    wire_address_network__static_method__Api_impl(port_, address)
}

#[no_mangle]
pub extern "C" fn wire_create_wallet__static_method__Api(
    port_: i64,
    descriptor: wire_BdkDescriptor,
    change_descriptor: *mut wire_BdkDescriptor,
    network: i32,
    database_config: *mut wire_DatabaseConfig,
) {
    wire_create_wallet__static_method__Api_impl(
        port_,
        descriptor,
        change_descriptor,
        network,
        database_config,
    )
}

#[no_mangle]
pub extern "C" fn wire_get_address__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    address_index: *mut wire_AddressIndex,
) {
    wire_get_address__static_method__Api_impl(port_, wallet, address_index)
}

#[no_mangle]
pub extern "C" fn wire_get_internal_address__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    address_index: *mut wire_AddressIndex,
) {
    wire_get_internal_address__static_method__Api_impl(port_, wallet, address_index)
}

#[no_mangle]
pub extern "C" fn wire_sync_wallet__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    blockchain: wire_BlockchainInstance,
) {
    wire_sync_wallet__static_method__Api_impl(port_, wallet, blockchain)
}

#[no_mangle]
pub extern "C" fn wire_sync_wallet_thread__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    blockchain: wire_BlockchainInstance,
) {
    wire_sync_wallet_thread__static_method__Api_impl(port_, wallet, blockchain)
}

#[no_mangle]
pub extern "C" fn wire_get_balance__static_method__Api(port_: i64, wallet: wire_WalletInstance) {
    wire_get_balance__static_method__Api_impl(port_, wallet)
}

#[no_mangle]
pub extern "C" fn wire_list_unspent_outputs__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
) {
    wire_list_unspent_outputs__static_method__Api_impl(port_, wallet)
}

#[no_mangle]
pub extern "C" fn wire_get_transactions__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    include_raw: bool,
) {
    wire_get_transactions__static_method__Api_impl(port_, wallet, include_raw)
}

#[no_mangle]
pub extern "C" fn wire_sign__static_method__Api(
    port_: i64,
    wallet: wire_WalletInstance,
    psbt_str: *mut wire_uint_8_list,
    sign_options: *mut wire_SignOptions,
) {
    wire_sign__static_method__Api_impl(port_, wallet, psbt_str, sign_options)
}

#[no_mangle]
pub extern "C" fn wire_wallet_network__static_method__Api(port_: i64, wallet: wire_WalletInstance) {
    wire_wallet_network__static_method__Api_impl(port_, wallet)
}

#[no_mangle]
pub extern "C" fn wire_list_unspent__static_method__Api(port_: i64, wallet: wire_WalletInstance) {
    wire_list_unspent__static_method__Api_impl(port_, wallet)
}

#[no_mangle]
pub extern "C" fn wire_generate_seed_from_word_count__static_method__Api(
    port_: i64,
    word_count: i32,
) {
    wire_generate_seed_from_word_count__static_method__Api_impl(port_, word_count)
}

#[no_mangle]
pub extern "C" fn wire_generate_seed_from_string__static_method__Api(
    port_: i64,
    mnemonic: *mut wire_uint_8_list,
) {
    wire_generate_seed_from_string__static_method__Api_impl(port_, mnemonic)
}

#[no_mangle]
pub extern "C" fn wire_generate_seed_from_entropy__static_method__Api(
    port_: i64,
    entropy: *mut wire_uint_8_list,
) {
    wire_generate_seed_from_entropy__static_method__Api_impl(port_, entropy)
}

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_BdkDescriptor() -> wire_BdkDescriptor {
    wire_BdkDescriptor::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_BlockchainInstance() -> wire_BlockchainInstance {
    wire_BlockchainInstance::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_WalletInstance() -> wire_WalletInstance {
    wire_WalletInstance::new_with_null_ptr()
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_BdkDescriptor_0() -> *mut wire_BdkDescriptor {
    support::new_leak_box_ptr(wire_BdkDescriptor::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_address_index_0() -> *mut wire_AddressIndex {
    support::new_leak_box_ptr(wire_AddressIndex::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_blockchain_config_0() -> *mut wire_BlockchainConfig {
    support::new_leak_box_ptr(wire_BlockchainConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_database_config_0() -> *mut wire_DatabaseConfig {
    support::new_leak_box_ptr(wire_DatabaseConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_electrum_config_0() -> *mut wire_ElectrumConfig {
    support::new_leak_box_ptr(wire_ElectrumConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_esplora_config_0() -> *mut wire_EsploraConfig {
    support::new_leak_box_ptr(wire_EsploraConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_f32_0(value: f32) -> *mut f32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_rbf_value_0() -> *mut wire_RbfValue {
    support::new_leak_box_ptr(wire_RbfValue::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_rpc_config_0() -> *mut wire_RpcConfig {
    support::new_leak_box_ptr(wire_RpcConfig::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_rpc_sync_params_0() -> *mut wire_RpcSyncParams {
    support::new_leak_box_ptr(wire_RpcSyncParams::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_script_0() -> *mut wire_Script {
    support::new_leak_box_ptr(wire_Script::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sign_options_0() -> *mut wire_SignOptions {
    support::new_leak_box_ptr(wire_SignOptions::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sled_db_configuration_0() -> *mut wire_SledDbConfiguration {
    support::new_leak_box_ptr(wire_SledDbConfiguration::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_sqlite_db_configuration_0() -> *mut wire_SqliteDbConfiguration {
    support::new_leak_box_ptr(wire_SqliteDbConfiguration::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u32_0(value: u32) -> *mut u32 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u64_0(value: u64) -> *mut u64 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_u8_0(value: u8) -> *mut u8 {
    support::new_leak_box_ptr(value)
}

#[no_mangle]
pub extern "C" fn new_box_autoadd_user_pass_0() -> *mut wire_UserPass {
    support::new_leak_box_ptr(wire_UserPass::new_with_null_ptr())
}

#[no_mangle]
pub extern "C" fn new_list_out_point_0(len: i32) -> *mut wire_list_out_point {
    let wrap = wire_list_out_point {
        ptr: support::new_leak_vec_ptr(<wire_OutPoint>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_list_script_amount_0(len: i32) -> *mut wire_list_script_amount {
    let wrap = wire_list_script_amount {
        ptr: support::new_leak_vec_ptr(<wire_ScriptAmount>::new_with_null_ptr(), len),
        len,
    };
    support::new_leak_box_ptr(wrap)
}

#[no_mangle]
pub extern "C" fn new_uint_8_list_0(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: related functions

#[no_mangle]
pub extern "C" fn drop_opaque_BdkDescriptor(ptr: *const c_void) {
    unsafe {
        Arc::<BdkDescriptor>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_BdkDescriptor(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<BdkDescriptor>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_BlockchainInstance(ptr: *const c_void) {
    unsafe {
        Arc::<BlockchainInstance>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_BlockchainInstance(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<BlockchainInstance>::increment_strong_count(ptr as _);
        ptr
    }
}

#[no_mangle]
pub extern "C" fn drop_opaque_WalletInstance(ptr: *const c_void) {
    unsafe {
        Arc::<WalletInstance>::decrement_strong_count(ptr as _);
    }
}

#[no_mangle]
pub extern "C" fn share_opaque_WalletInstance(ptr: *const c_void) -> *const c_void {
    unsafe {
        Arc::<WalletInstance>::increment_strong_count(ptr as _);
        ptr
    }
}

// Section: impl Wire2Api

impl Wire2Api<RustOpaque<BdkDescriptor>> for wire_BdkDescriptor {
    fn wire2api(self) -> RustOpaque<BdkDescriptor> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<RustOpaque<BlockchainInstance>> for wire_BlockchainInstance {
    fn wire2api(self) -> RustOpaque<BlockchainInstance> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}
impl Wire2Api<RustOpaque<WalletInstance>> for wire_WalletInstance {
    fn wire2api(self) -> RustOpaque<WalletInstance> {
        unsafe { support::opaque_from_dart(self.ptr as _) }
    }
}
impl Wire2Api<AddressIndex> for wire_AddressIndex {
    fn wire2api(self) -> AddressIndex {
        match self.tag {
            0 => AddressIndex::New,
            1 => AddressIndex::LastUnused,
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Peek);
                AddressIndex::Peek {
                    index: ans.index.wire2api(),
                }
            },
            3 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Reset);
                AddressIndex::Reset {
                    index: ans.index.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<BlockchainConfig> for wire_BlockchainConfig {
    fn wire2api(self) -> BlockchainConfig {
        match self.tag {
            0 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Electrum);
                BlockchainConfig::Electrum {
                    config: ans.config.wire2api(),
                }
            },
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Esplora);
                BlockchainConfig::Esplora {
                    config: ans.config.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Rpc);
                BlockchainConfig::Rpc {
                    config: ans.config.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}

impl Wire2Api<RustOpaque<BdkDescriptor>> for *mut wire_BdkDescriptor {
    fn wire2api(self) -> RustOpaque<BdkDescriptor> {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RustOpaque<BdkDescriptor>>::wire2api(*wrap).into()
    }
}
impl Wire2Api<AddressIndex> for *mut wire_AddressIndex {
    fn wire2api(self) -> AddressIndex {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<AddressIndex>::wire2api(*wrap).into()
    }
}
impl Wire2Api<BlockchainConfig> for *mut wire_BlockchainConfig {
    fn wire2api(self) -> BlockchainConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<BlockchainConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<DatabaseConfig> for *mut wire_DatabaseConfig {
    fn wire2api(self) -> DatabaseConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<DatabaseConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<ElectrumConfig> for *mut wire_ElectrumConfig {
    fn wire2api(self) -> ElectrumConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<ElectrumConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<EsploraConfig> for *mut wire_EsploraConfig {
    fn wire2api(self) -> EsploraConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<EsploraConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<f32> for *mut f32 {
    fn wire2api(self) -> f32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<RbfValue> for *mut wire_RbfValue {
    fn wire2api(self) -> RbfValue {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RbfValue>::wire2api(*wrap).into()
    }
}
impl Wire2Api<RpcConfig> for *mut wire_RpcConfig {
    fn wire2api(self) -> RpcConfig {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RpcConfig>::wire2api(*wrap).into()
    }
}
impl Wire2Api<RpcSyncParams> for *mut wire_RpcSyncParams {
    fn wire2api(self) -> RpcSyncParams {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<RpcSyncParams>::wire2api(*wrap).into()
    }
}
impl Wire2Api<Script> for *mut wire_Script {
    fn wire2api(self) -> Script {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<Script>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SignOptions> for *mut wire_SignOptions {
    fn wire2api(self) -> SignOptions {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SignOptions>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SledDbConfiguration> for *mut wire_SledDbConfiguration {
    fn wire2api(self) -> SledDbConfiguration {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SledDbConfiguration>::wire2api(*wrap).into()
    }
}
impl Wire2Api<SqliteDbConfiguration> for *mut wire_SqliteDbConfiguration {
    fn wire2api(self) -> SqliteDbConfiguration {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<SqliteDbConfiguration>::wire2api(*wrap).into()
    }
}
impl Wire2Api<u32> for *mut u32 {
    fn wire2api(self) -> u32 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u64> for *mut u64 {
    fn wire2api(self) -> u64 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<u8> for *mut u8 {
    fn wire2api(self) -> u8 {
        unsafe { *support::box_from_leak_ptr(self) }
    }
}
impl Wire2Api<UserPass> for *mut wire_UserPass {
    fn wire2api(self) -> UserPass {
        let wrap = unsafe { support::box_from_leak_ptr(self) };
        Wire2Api::<UserPass>::wire2api(*wrap).into()
    }
}

impl Wire2Api<DatabaseConfig> for wire_DatabaseConfig {
    fn wire2api(self) -> DatabaseConfig {
        match self.tag {
            0 => DatabaseConfig::Memory,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Sqlite);
                DatabaseConfig::Sqlite {
                    config: ans.config.wire2api(),
                }
            },
            2 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Sled);
                DatabaseConfig::Sled {
                    config: ans.config.wire2api(),
                }
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<ElectrumConfig> for wire_ElectrumConfig {
    fn wire2api(self) -> ElectrumConfig {
        ElectrumConfig {
            url: self.url.wire2api(),
            socks5: self.socks5.wire2api(),
            retry: self.retry.wire2api(),
            timeout: self.timeout.wire2api(),
            stop_gap: self.stop_gap.wire2api(),
            validate_domain: self.validate_domain.wire2api(),
        }
    }
}
impl Wire2Api<EsploraConfig> for wire_EsploraConfig {
    fn wire2api(self) -> EsploraConfig {
        EsploraConfig {
            base_url: self.base_url.wire2api(),
            proxy: self.proxy.wire2api(),
            concurrency: self.concurrency.wire2api(),
            stop_gap: self.stop_gap.wire2api(),
            timeout: self.timeout.wire2api(),
        }
    }
}

impl Wire2Api<Vec<OutPoint>> for *mut wire_list_out_point {
    fn wire2api(self) -> Vec<OutPoint> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}
impl Wire2Api<Vec<ScriptAmount>> for *mut wire_list_script_amount {
    fn wire2api(self) -> Vec<ScriptAmount> {
        let vec = unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        };
        vec.into_iter().map(Wire2Api::wire2api).collect()
    }
}

impl Wire2Api<OutPoint> for wire_OutPoint {
    fn wire2api(self) -> OutPoint {
        OutPoint {
            txid: self.txid.wire2api(),
            vout: self.vout.wire2api(),
        }
    }
}
impl Wire2Api<RbfValue> for wire_RbfValue {
    fn wire2api(self) -> RbfValue {
        match self.tag {
            0 => RbfValue::RbfDefault,
            1 => unsafe {
                let ans = support::box_from_leak_ptr(self.kind);
                let ans = support::box_from_leak_ptr(ans.Value);
                RbfValue::Value(ans.field0.wire2api())
            },
            _ => unreachable!(),
        }
    }
}
impl Wire2Api<RpcConfig> for wire_RpcConfig {
    fn wire2api(self) -> RpcConfig {
        RpcConfig {
            url: self.url.wire2api(),
            auth_cookie: self.auth_cookie.wire2api(),
            auth_user_pass: self.auth_user_pass.wire2api(),
            network: self.network.wire2api(),
            wallet_name: self.wallet_name.wire2api(),
            sync_params: self.sync_params.wire2api(),
        }
    }
}
impl Wire2Api<RpcSyncParams> for wire_RpcSyncParams {
    fn wire2api(self) -> RpcSyncParams {
        RpcSyncParams {
            start_script_count: self.start_script_count.wire2api(),
            start_time: self.start_time.wire2api(),
            force_start_time: self.force_start_time.wire2api(),
            poll_rate_sec: self.poll_rate_sec.wire2api(),
        }
    }
}
impl Wire2Api<Script> for wire_Script {
    fn wire2api(self) -> Script {
        Script {
            internal: self.internal.wire2api(),
        }
    }
}
impl Wire2Api<ScriptAmount> for wire_ScriptAmount {
    fn wire2api(self) -> ScriptAmount {
        ScriptAmount {
            script: self.script.wire2api(),
            amount: self.amount.wire2api(),
        }
    }
}
impl Wire2Api<SignOptions> for wire_SignOptions {
    fn wire2api(self) -> SignOptions {
        SignOptions {
            trust_witness_utxo: self.trust_witness_utxo.wire2api(),
            assume_height: self.assume_height.wire2api(),
            allow_all_sighashes: self.allow_all_sighashes.wire2api(),
            remove_partial_sigs: self.remove_partial_sigs.wire2api(),
            try_finalize: self.try_finalize.wire2api(),
            sign_with_tap_internal_key: self.sign_with_tap_internal_key.wire2api(),
            allow_grinding: self.allow_grinding.wire2api(),
        }
    }
}
impl Wire2Api<SledDbConfiguration> for wire_SledDbConfiguration {
    fn wire2api(self) -> SledDbConfiguration {
        SledDbConfiguration {
            path: self.path.wire2api(),
            tree_name: self.tree_name.wire2api(),
        }
    }
}
impl Wire2Api<SqliteDbConfiguration> for wire_SqliteDbConfiguration {
    fn wire2api(self) -> SqliteDbConfiguration {
        SqliteDbConfiguration {
            path: self.path.wire2api(),
        }
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}
impl Wire2Api<UserPass> for wire_UserPass {
    fn wire2api(self) -> UserPass {
        UserPass {
            username: self.username.wire2api(),
            password: self.password.wire2api(),
        }
    }
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_BdkDescriptor {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockchainInstance {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_WalletInstance {
    ptr: *const core::ffi::c_void,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ElectrumConfig {
    url: *mut wire_uint_8_list,
    socks5: *mut wire_uint_8_list,
    retry: u8,
    timeout: *mut u8,
    stop_gap: u64,
    validate_domain: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_EsploraConfig {
    base_url: *mut wire_uint_8_list,
    proxy: *mut wire_uint_8_list,
    concurrency: *mut u8,
    stop_gap: u64,
    timeout: *mut u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_out_point {
    ptr: *mut wire_OutPoint,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_list_script_amount {
    ptr: *mut wire_ScriptAmount,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_OutPoint {
    txid: *mut wire_uint_8_list,
    vout: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RpcConfig {
    url: *mut wire_uint_8_list,
    auth_cookie: *mut wire_uint_8_list,
    auth_user_pass: *mut wire_UserPass,
    network: i32,
    wallet_name: *mut wire_uint_8_list,
    sync_params: *mut wire_RpcSyncParams,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RpcSyncParams {
    start_script_count: u64,
    start_time: u64,
    force_start_time: bool,
    poll_rate_sec: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_Script {
    internal: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_ScriptAmount {
    script: wire_Script,
    amount: u64,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SignOptions {
    trust_witness_utxo: bool,
    assume_height: *mut u32,
    allow_all_sighashes: bool,
    remove_partial_sigs: bool,
    try_finalize: bool,
    sign_with_tap_internal_key: bool,
    allow_grinding: bool,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SledDbConfiguration {
    path: *mut wire_uint_8_list,
    tree_name: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_SqliteDbConfiguration {
    path: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_UserPass {
    username: *mut wire_uint_8_list,
    password: *mut wire_uint_8_list,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AddressIndex {
    tag: i32,
    kind: *mut AddressIndexKind,
}

#[repr(C)]
pub union AddressIndexKind {
    New: *mut wire_AddressIndex_New,
    LastUnused: *mut wire_AddressIndex_LastUnused,
    Peek: *mut wire_AddressIndex_Peek,
    Reset: *mut wire_AddressIndex_Reset,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AddressIndex_New {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AddressIndex_LastUnused {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AddressIndex_Peek {
    index: u32,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_AddressIndex_Reset {
    index: u32,
}
#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockchainConfig {
    tag: i32,
    kind: *mut BlockchainConfigKind,
}

#[repr(C)]
pub union BlockchainConfigKind {
    Electrum: *mut wire_BlockchainConfig_Electrum,
    Esplora: *mut wire_BlockchainConfig_Esplora,
    Rpc: *mut wire_BlockchainConfig_Rpc,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockchainConfig_Electrum {
    config: *mut wire_ElectrumConfig,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockchainConfig_Esplora {
    config: *mut wire_EsploraConfig,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_BlockchainConfig_Rpc {
    config: *mut wire_RpcConfig,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DatabaseConfig {
    tag: i32,
    kind: *mut DatabaseConfigKind,
}

#[repr(C)]
pub union DatabaseConfigKind {
    Memory: *mut wire_DatabaseConfig_Memory,
    Sqlite: *mut wire_DatabaseConfig_Sqlite,
    Sled: *mut wire_DatabaseConfig_Sled,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DatabaseConfig_Memory {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DatabaseConfig_Sqlite {
    config: *mut wire_SqliteDbConfiguration,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_DatabaseConfig_Sled {
    config: *mut wire_SledDbConfiguration,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RbfValue {
    tag: i32,
    kind: *mut RbfValueKind,
}

#[repr(C)]
pub union RbfValueKind {
    RbfDefault: *mut wire_RbfValue_RbfDefault,
    Value: *mut wire_RbfValue_Value,
}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RbfValue_RbfDefault {}

#[repr(C)]
#[derive(Clone)]
pub struct wire_RbfValue_Value {
    field0: u32,
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

impl NewWithNullPtr for wire_BdkDescriptor {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_BlockchainInstance {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}

impl NewWithNullPtr for wire_WalletInstance {
    fn new_with_null_ptr() -> Self {
        Self {
            ptr: core::ptr::null(),
        }
    }
}
impl NewWithNullPtr for wire_AddressIndex {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_AddressIndex_Peek() -> *mut AddressIndexKind {
    support::new_leak_box_ptr(AddressIndexKind {
        Peek: support::new_leak_box_ptr(wire_AddressIndex_Peek {
            index: Default::default(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_AddressIndex_Reset() -> *mut AddressIndexKind {
    support::new_leak_box_ptr(AddressIndexKind {
        Reset: support::new_leak_box_ptr(wire_AddressIndex_Reset {
            index: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_BlockchainConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_BlockchainConfig_Electrum() -> *mut BlockchainConfigKind {
    support::new_leak_box_ptr(BlockchainConfigKind {
        Electrum: support::new_leak_box_ptr(wire_BlockchainConfig_Electrum {
            config: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_BlockchainConfig_Esplora() -> *mut BlockchainConfigKind {
    support::new_leak_box_ptr(BlockchainConfigKind {
        Esplora: support::new_leak_box_ptr(wire_BlockchainConfig_Esplora {
            config: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_BlockchainConfig_Rpc() -> *mut BlockchainConfigKind {
    support::new_leak_box_ptr(BlockchainConfigKind {
        Rpc: support::new_leak_box_ptr(wire_BlockchainConfig_Rpc {
            config: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_DatabaseConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_DatabaseConfig_Sqlite() -> *mut DatabaseConfigKind {
    support::new_leak_box_ptr(DatabaseConfigKind {
        Sqlite: support::new_leak_box_ptr(wire_DatabaseConfig_Sqlite {
            config: core::ptr::null_mut(),
        }),
    })
}

#[no_mangle]
pub extern "C" fn inflate_DatabaseConfig_Sled() -> *mut DatabaseConfigKind {
    support::new_leak_box_ptr(DatabaseConfigKind {
        Sled: support::new_leak_box_ptr(wire_DatabaseConfig_Sled {
            config: core::ptr::null_mut(),
        }),
    })
}

impl NewWithNullPtr for wire_ElectrumConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            url: core::ptr::null_mut(),
            socks5: core::ptr::null_mut(),
            retry: Default::default(),
            timeout: core::ptr::null_mut(),
            stop_gap: Default::default(),
            validate_domain: Default::default(),
        }
    }
}

impl Default for wire_ElectrumConfig {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_EsploraConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            base_url: core::ptr::null_mut(),
            proxy: core::ptr::null_mut(),
            concurrency: core::ptr::null_mut(),
            stop_gap: Default::default(),
            timeout: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_EsploraConfig {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_OutPoint {
    fn new_with_null_ptr() -> Self {
        Self {
            txid: core::ptr::null_mut(),
            vout: Default::default(),
        }
    }
}

impl Default for wire_OutPoint {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_RbfValue {
    fn new_with_null_ptr() -> Self {
        Self {
            tag: -1,
            kind: core::ptr::null_mut(),
        }
    }
}

#[no_mangle]
pub extern "C" fn inflate_RbfValue_Value() -> *mut RbfValueKind {
    support::new_leak_box_ptr(RbfValueKind {
        Value: support::new_leak_box_ptr(wire_RbfValue_Value {
            field0: Default::default(),
        }),
    })
}

impl NewWithNullPtr for wire_RpcConfig {
    fn new_with_null_ptr() -> Self {
        Self {
            url: core::ptr::null_mut(),
            auth_cookie: core::ptr::null_mut(),
            auth_user_pass: core::ptr::null_mut(),
            network: Default::default(),
            wallet_name: core::ptr::null_mut(),
            sync_params: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_RpcConfig {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_RpcSyncParams {
    fn new_with_null_ptr() -> Self {
        Self {
            start_script_count: Default::default(),
            start_time: Default::default(),
            force_start_time: Default::default(),
            poll_rate_sec: Default::default(),
        }
    }
}

impl Default for wire_RpcSyncParams {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_Script {
    fn new_with_null_ptr() -> Self {
        Self {
            internal: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_Script {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_ScriptAmount {
    fn new_with_null_ptr() -> Self {
        Self {
            script: Default::default(),
            amount: Default::default(),
        }
    }
}

impl Default for wire_ScriptAmount {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SignOptions {
    fn new_with_null_ptr() -> Self {
        Self {
            trust_witness_utxo: Default::default(),
            assume_height: core::ptr::null_mut(),
            allow_all_sighashes: Default::default(),
            remove_partial_sigs: Default::default(),
            try_finalize: Default::default(),
            sign_with_tap_internal_key: Default::default(),
            allow_grinding: Default::default(),
        }
    }
}

impl Default for wire_SignOptions {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SledDbConfiguration {
    fn new_with_null_ptr() -> Self {
        Self {
            path: core::ptr::null_mut(),
            tree_name: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SledDbConfiguration {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_SqliteDbConfiguration {
    fn new_with_null_ptr() -> Self {
        Self {
            path: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_SqliteDbConfiguration {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

impl NewWithNullPtr for wire_UserPass {
    fn new_with_null_ptr() -> Self {
        Self {
            username: core::ptr::null_mut(),
            password: core::ptr::null_mut(),
        }
    }
}

impl Default for wire_UserPass {
    fn default() -> Self {
        Self::new_with_null_ptr()
    }
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturn(ptr: support::WireSyncReturn) {
    unsafe {
        let _ = support::box_from_leak_ptr(ptr);
    };
}
