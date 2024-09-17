use aes::cipher::block_padding::{Pkcs7, UnpadError};
use aes::cipher::{BlockDecryptMut, BlockEncryptMut, KeyIvInit};
use aes::Aes256;
use log::trace;

use crate::{AES256IV, AES256KEY};

type Aes256CbcEnc = cbc::Encryptor<Aes256>;
type Aes256CbcDec = cbc::Decryptor<Aes256>;

pub fn encrypt_aes256(plain_text: &[u8]) -> Vec<u8> {
    trace!("Plain text: {plain_text:?}");
    let s = Aes256CbcEnc::new(AES256KEY.as_slice().into(), AES256IV.as_slice().into())
        .encrypt_padded_vec_mut::<Pkcs7>(plain_text);
    trace!("Encrypted text: {s:?}");
    s
}

pub fn decrypt_aes256(encrypt_text: &[u8]) -> Result<Vec<u8>, UnpadError> {
    Aes256CbcDec::new(AES256KEY.as_slice().into(), AES256IV.as_slice().into())
        .decrypt_padded_vec_mut::<Pkcs7>(&encrypt_text)
}
