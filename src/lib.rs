use crate::p256::P256Key;
use bls12381::Bls12381KeyPair;
use ed25519::Ed25519KeyPair;
use std::convert::{TryFrom, TryInto};
use url::Url;
use x25519::X25519Key;

pub enum Payload {
    Buffer(Vec<u8>),
    BufferArray(Vec<Vec<u8>>),
}

impl From<&[u8]> for Payload {
    fn from(data: &[u8]) -> Self {
        Payload::Buffer(data.to_vec())
    }
}

impl From<Vec<u8>> for Payload {
    fn from(data: Vec<u8>) -> Self {
        Payload::Buffer(data)
    }
}

pub struct AsymmetricKey<P, S> {
    public_key: P,
    secret_key: Option<S>,
}

pub trait Ecdsa {
    type Err;

    fn sign(&self, payload: Payload) -> Vec<u8>;
    fn verify(&self, payload: Payload, signature: &[u8]) -> Result<(), Self::Err>;
}

pub trait Ecdh {
    fn key_exchange(&self, their_public: &Self) -> Vec<u8>;
}

pub(crate) fn generate_seed(initial_seed: &[u8]) -> Result<[u8; 32], &str> {
    let mut seed = [0u8; 32];
    if initial_seed.is_empty() || initial_seed.len() != 32 {
        getrandom::getrandom(&mut seed).expect("couldn't generate random seed");
    } else {
        seed = match initial_seed.try_into() {
            Ok(x) => x,
            Err(_) => return Err("invalid seed size"),
        };
    }
    Ok(seed)
}

pub enum DIDKey {
    Ed25519(Ed25519KeyPair),
    X25519(X25519Key),
    P256(P256Key),
    Bls12381G1(Bls12381KeyPair),
    Bls12381G2(Bls12381KeyPair),
}

pub enum DIDKeyType {
    Ed25519,
    X25519,
    P256,
    Bls12381G1,
    Bls12381G2,
}

impl DIDKey {
    pub fn resolve(did_uri: &str) -> Result<DIDKey, String> {
        DIDKey::try_from(did_uri.to_string())
    }

    pub fn fingerprint(&self) -> String {
        let codec: &[u8] = match self {
            DIDKey::Ed25519(_) => &[0xed, 0x1],
            DIDKey::X25519(_) => &[0xec, 0x1],
            DIDKey::P256(_) => &[0x12, 0x0, 0x1],
            DIDKey::Bls12381G1(_) => &[0xea, 0x1],
            DIDKey::Bls12381G2(_) => &[0xeb, 0x1],
        };
        let data = [codec, self.public_key().as_slice()].concat();
        format!("z{}", bs58::encode(data).into_string())
    }

    pub fn from_seed(key_type: DIDKeyType, seed: &[u8]) -> DIDKey {
        match key_type {
            DIDKeyType::Ed25519 => DIDKey::Ed25519(Ed25519KeyPair::from_seed(seed)),
            DIDKeyType::X25519 => DIDKey::X25519(X25519Key::from_seed(seed)),
            DIDKeyType::P256 => DIDKey::P256(P256Key::from_seed(seed)),
            DIDKeyType::Bls12381G1 => todo!(),
            DIDKeyType::Bls12381G2 => todo!(),
        }
    }

    pub fn from_public_key(key_type: DIDKeyType, seed: &[u8]) -> DIDKey {
        match key_type {
            DIDKeyType::Ed25519 => DIDKey::Ed25519(Ed25519KeyPair::from_public_key(seed)),
            DIDKeyType::X25519 => DIDKey::X25519(X25519Key::from_public_key(seed)),
            DIDKeyType::P256 => DIDKey::P256(P256Key::from_public_key(seed)),
            DIDKeyType::Bls12381G1 => todo!(),
            DIDKeyType::Bls12381G2 => todo!(),
        }
    }

    pub fn key_exchange(&self, key: &Self) -> Vec<u8> {
        match (self, key) {
            (DIDKey::X25519(sk), DIDKey::X25519(pk)) => sk.key_exchange(pk),
            (DIDKey::P256(_sk), DIDKey::P256(_pk)) => todo!(),
            _ => unimplemented!(),
        }
    }

    pub fn sign(&self, payload: Payload) -> Vec<u8> {
        match self {
            DIDKey::Ed25519(x) => x.sign(payload),
            DIDKey::P256(x) => x.sign(payload),
            DIDKey::Bls12381G1(x) => x.sign(payload),
            DIDKey::Bls12381G2(x) => x.sign(payload),
            _ => unimplemented!(),
        }
    }

    pub fn verify(&self, payload: Payload, signature: &Vec<u8>) -> bool {
        match self {
            DIDKey::Ed25519(x) => x.verify(payload, signature.as_slice()),
            DIDKey::P256(x) => x.verify(payload, signature.as_slice()),
            DIDKey::Bls12381G1(x) => x.verify(payload, signature.as_slice()),
            DIDKey::Bls12381G2(x) => x.verify(payload, signature.as_slice()),
            _ => unimplemented!(),
        }
        .map_or(false, |()| true)
    }

    pub fn public_key(&self) -> Vec<u8> {
        match self {
            DIDKey::Ed25519(x) => x.public_key.as_bytes().to_vec(),
            DIDKey::X25519(x) => x.public_key.to_bytes().to_vec(),
            DIDKey::P256(x) => x.public_key.to_encoded_point(false).as_bytes().to_vec(),
            DIDKey::Bls12381G1(x) => x.public_key.clone(),
            DIDKey::Bls12381G2(x) => x.public_key.clone(),
        }
    }

    pub fn secret_key(&self) -> Option<Vec<u8>> {
        match self {
            DIDKey::Ed25519(key) => (&key.secret_key).as_ref().map_or(None, |x| Some(x.to_bytes().to_vec())),
            DIDKey::X25519(key) => (&key.secret_key).as_ref().map_or(None, |x| Some(x.to_bytes().to_vec())),
            DIDKey::P256(key) => (&key.secret_key).as_ref().map_or(None, |x| Some(x.to_bytes().to_vec())),
            DIDKey::Bls12381G1(_) => todo!(),
            DIDKey::Bls12381G2(_) => todo!(),
        }
    }
}

impl TryFrom<String> for DIDKey {
    type Error = String;

    fn try_from(did_uri: String) -> Result<Self, Self::Error> {
        // let re = Regex::new(r"did:key:[\w]*#[\w]*\??[\w]*").unwrap();

        let url = match Url::parse(did_uri.as_ref()) {
            Ok(url) => url,
            Err(_) => return Err("couldn't parse DID URI".to_string()),
        };

        let pub_key = match url
            .fragment()
            .map_or(url.to_string().replace("did:key:", ""), |x| x.to_string())
            .strip_prefix("z")
        {
            Some(url) => {
                match bs58::decode(url).into_vec() {
                    Ok(url) => url,
                    Err(_) => return Err("invalid base58 encoded data in DID URI".to_string()),
                }
            }
            None => return Err("invalid URI data".to_string()),
        };

        return Ok(match pub_key[0..2] {
            [0xed, 0x1] => DIDKey::from_public_key(DIDKeyType::Ed25519, &pub_key[2..]),
            [0xec, 0x1] => DIDKey::from_public_key(DIDKeyType::X25519, &pub_key[2..]),
            [0xea, 0x1] => DIDKey::from_public_key(DIDKeyType::Bls12381G1, &pub_key[2..]),
            [0xeb, 0x1] => DIDKey::from_public_key(DIDKeyType::Bls12381G2, &pub_key[2..]),
            [0x12, 0x0] => DIDKey::from_public_key(DIDKeyType::P256, &pub_key[3..]),
            _ => unimplemented!("unsupported key type"),
        });
    }
}

pub mod bls12381;
pub mod ed25519;
pub mod p256;
pub mod x25519;

#[cfg(test)]
pub mod test {
    use crate::{DIDKey, Payload};

    use super::*;
    #[test]
    fn test_demo() {
        let secret_key = "6Lx39RyWn3syuozAe2WiPdAYn1ctMx17t8yrBMGFBmZy";
        let public_key = "6fioC1zcDPyPEL19pXRS2E4iJ46zH7xP6uSgAaPdwDrx";

        let sk = DIDKey::Ed25519(Ed25519KeyPair::from_seed(
            bs58::decode(secret_key).into_vec().unwrap().as_slice(),
        ));
        let message = b"super secret message";

        let signature = sk.sign(Payload::Buffer(message.to_vec()));

        let pk = DIDKey::Ed25519(Ed25519KeyPair::from_public_key(
            bs58::decode(public_key).into_vec().unwrap().as_slice(),
        ));
        let is_valid = pk.verify(Payload::Buffer(message.to_vec()), &signature);

        assert!(is_valid);
    }

    #[test]
    fn test_key_from_uri() {
        let uri = "did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL";

        let key = DIDKey::try_from(uri.to_string());

        assert!(matches!(key.unwrap(), DIDKey::Ed25519(_)));
    }

    #[test]
    fn test_key_from_uri_fragment() {
        let uri =
            "did:key:z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL#z6Mkk7yqnGF3YwTrLpqrW6PGsKci7dNqh1CjnvMbzrMerSeL";

        let key = DIDKey::try_from(uri.to_string());

        assert!(matches!(key.unwrap(), DIDKey::Ed25519(_)));
    }
}