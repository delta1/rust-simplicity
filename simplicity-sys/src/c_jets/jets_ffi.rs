/* This file has been automatically generated. */

use std::ffi::c_void;
use super::c_env::CElementsTxEnv;
use super::frame_ffi::CFrameItem;

extern "C" {
    pub fn add_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn add_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn add_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn add_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn all_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn all_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn all_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn all_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn and_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn and_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn and_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn and_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn annex_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn asset_amount_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn bip_0340_verify(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn build_tapbranch(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn build_tapleaf_simplicity(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn calculate_asset(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn calculate_confidential_token(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn calculate_explicit_token(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn calculate_issuance_entropy(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn ch_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn ch_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn ch_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn ch_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn check_lock_distance(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn check_lock_duration(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn check_lock_height(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn check_lock_time(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn check_sig_verify(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn complement_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn complement_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn complement_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn complement_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn current_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_annex_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_asset(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_index(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_issuance_asset_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_issuance_asset_proof(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_issuance_token_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_issuance_token_proof(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_new_issuance_contract(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_pegin(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_prev_outpoint(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_reissuance_blinding(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_reissuance_entropy(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_script_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_script_sig_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn current_sequence(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn decompress(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn decrement_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn decrement_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn decrement_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn decrement_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn div_mod_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn div_mod_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn div_mod_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn div_mod_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divide_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divide_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divide_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divide_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divides_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divides_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divides_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn divides_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn eq_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn eq_256(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn eq_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn eq_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn eq_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_add(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_invert(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_is_odd(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_is_zero(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_multiply(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_multiply_beta(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_negate(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_normalize(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_square(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn fe_square_root(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_add_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_add_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_add_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_add_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_decrement_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_decrement_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_decrement_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_decrement_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_increment_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_increment_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_increment_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_increment_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_multiply_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_multiply_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_multiply_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_multiply_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_subtract_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_subtract_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_subtract_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn full_subtract_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn ge_is_on_curve(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn ge_negate(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_add(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_double(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_ge_add(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_ge_add_ex(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_infinity(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_is_infinity(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_is_on_curve(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_negate(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_normalize(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_rescale(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_x_equiv(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn gej_y_is_odd(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn generate(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn genesis_block_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn high_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn high_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn high_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn high_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn increment_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn increment_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn increment_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn increment_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn input_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_amounts_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_annex_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_annexes_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_asset(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_outpoints_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_pegin(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_prev_outpoint(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_script_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_script_sig_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_script_sigs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_scripts_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_sequence(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_sequences_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn input_utxos_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn inputs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn internal_key(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn is_one_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_one_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_one_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_one_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_zero_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_zero_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_zero_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn is_zero_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn issuance(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_asset(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_asset_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_asset_amounts_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_asset_proof(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_blinding_entropy_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_entropy(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_range_proofs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_token(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_token_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_token_amounts_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuance_token_proof(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn issuances_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn le_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn le_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn le_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn le_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn linear_combination_1(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn linear_verify_1(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn lock_time(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn low_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn low_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn low_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn low_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn lt_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn lt_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn lt_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn lt_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn maj_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn maj_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn maj_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn maj_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn max_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn max_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn max_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn max_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn median_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn median_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn median_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn median_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn min_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn min_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn min_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn min_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn modulo_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn modulo_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn modulo_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn modulo_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn multiply_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn multiply_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn multiply_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn multiply_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn negate_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn negate_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn negate_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn negate_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn new_issuance_contract(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn nonce_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn num_inputs(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn num_outputs(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn one_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn one_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn one_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn one_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn or_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn or_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn or_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn or_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn outpoint_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_amount(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_amounts_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_asset(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_nonce(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_nonces_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_null_datum(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_range_proof(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_range_proofs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_script_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_scripts_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_surjection_proof(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn output_surjection_proofs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn outputs_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn parse_lock(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn parse_sequence(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn point_verify_1(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn reissuance_blinding(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn reissuance_entropy(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn scalar_add(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_invert(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_is_zero(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_multiply(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_multiply_lambda(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_negate(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_normalize(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scalar_square(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn scale(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn script_cmr(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn sha_256_block(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_1(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_128(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_2(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_256(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_4(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_512(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_add_buffer_511(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_finalize(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_ctx_8_init(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sha_256_iv(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn sig_all_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn some_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn some_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn some_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn some_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn subtract_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn subtract_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn subtract_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn subtract_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn tap_env_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tapleaf_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tapleaf_version(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tappath(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tappath_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tx_hash(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tx_is_final(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tx_lock_distance(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tx_lock_duration(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tx_lock_height(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn tx_lock_time(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn verify(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn version(dst: *mut CFrameItem, src: CFrameItem, env: *const CElementsTxEnv) -> bool;
    pub fn xor_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_xor_16(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_xor_32(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_xor_64(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
    pub fn xor_xor_8(dst: *mut CFrameItem, src: CFrameItem, env: *const c_void) -> bool;
}
