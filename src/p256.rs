use super::{generate_seed, Ecdsa};
use crate::{
    didcore::{Config, Fingerprint, KeyFormat, JWK},
    AsymmetricKey, DIDCore, Document, Payload, VerificationMethod,
};
use p256::{
    ecdsa::{signature::Signer, signature::Verifier, Signature, SigningKey, VerifyKey},
    EncodedPoint,
};
use std::convert::TryFrom;

pub type P256KeyPair = AsymmetricKey<VerifyKey, SigningKey>;

impl std::fmt::Debug for P256KeyPair {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self.public_key))
    }
}

impl P256KeyPair {
    pub fn from_seed(seed: &[u8]) -> Self {
        let secret_seed = generate_seed(&seed.to_vec()).expect("invalid seed");

        let sk = SigningKey::new(&secret_seed).expect("Couldn't create key");
        let pk = VerifyKey::from(&sk);

        P256KeyPair {
            public_key: pk, //.to_encoded_point(false),
            secret_key: Some(sk),
        }
    }

    pub fn from_public_key(public_key: &[u8]) -> Self {
        let pk: Vec<u8> = match public_key.len() == 65 {
            true => public_key.to_vec(),
            false => {
                let mut pkk = public_key.to_vec();
                pkk.insert(0, 0x04);
                pkk
            }
        };
        P256KeyPair {
            secret_key: None, //.to_encoded_point(false),
            public_key: VerifyKey::from_encoded_point(&EncodedPoint::from_bytes(pk.as_slice()).expect("invalid key"))
                .expect("invalid point"),
        }
    }
}

impl Ecdsa for P256KeyPair {
    type Err = String;

    fn sign(&self, payload: Payload) -> Vec<u8> {
        match payload {
            Payload::Buffer(payload) => {
                let signature = match &self.secret_key {
                    Some(sig) => sig.sign(&payload),
                    None => panic!("secret key not found"),
                };
                signature.as_ref().to_vec()
            }
            _ => unimplemented!("payload type not supported for this key"),
        }
    }

    fn verify(&self, payload: Payload, signature: &[u8]) -> Result<(), Self::Err> {
        match payload {
            Payload::Buffer(payload) => match self
                .public_key
                .verify(&payload, &Signature::try_from(signature).unwrap())
                .is_ok()
            {
                true => Ok(()),
                false => Err("invalid signature".to_string()),
            },
            _ => unimplemented!("payload type not supported for this key"),
        }
    }
}

impl DIDCore for P256KeyPair {
    fn to_verification_method(&self, config: Config, controller: &str) -> Vec<VerificationMethod> {
        vec![VerificationMethod {
            id: format!("{}#{}", controller, self.fingerprint()),
            key_type: match config.use_jose_format {
                false => "UnsupportedVerificationMethod2020".into(),
                true => "JsonWebKey2020".into(),
            },
            controller: controller.to_string(),
            public_key: Some(match config.use_jose_format {
                false => {
                    KeyFormat::Base58(bs58::encode(self.public_key.to_encoded_point(false).as_bytes()).into_string())
                }
                true => KeyFormat::JWK(JWK {
                    key_type: "EC".into(),
                    curve: "P-256".into(),
                    x: Some(base64::encode_config(
                        self.public_key.to_encoded_point(false).as_bytes(),
                        base64::URL_SAFE_NO_PAD,
                    )),
                    y: None,
                    d: None,
                }),
            }),
            private_key: None,
        }]
    }

    fn to_did_document(&self, config: Config) -> crate::Document {
        let fingerprint = self.fingerprint();
        let controller = format!("did:key:{}", fingerprint.clone());

        let vm = self.to_verification_method(config, &controller);

        Document {
            context: "https://www.w3.org/ns/did/v1".to_string(),
            id: controller.to_string(),
            key_agreement: Some(vm.iter().map(|x| x.id.to_string()).collect()),
            authentication: Some(vec![vm[0].id.clone()]),
            assertion_method: Some(vec![vm[0].id.clone()]),
            capability_delegation: Some(vec![vm[0].id.clone()]),
            capability_invocation: Some(vec![vm[0].id.clone()]),
            verification_method: vm,
        }
    }
}

impl Fingerprint for P256KeyPair {
    fn fingerprint(&self) -> String {
        let codec: &[u8] = &[0x12, 0x0, 0x1];
        let data = [codec, self.public_key.to_encoded_point(false).as_ref()].concat();
        format!("z{}", bs58::encode(data).into_string())
    }
}

#[cfg(test)]
pub mod test {
    use crate::{DIDKey, DIDKeyType};

    use super::*;
    #[test]
    fn test_demo() {
        let key = P256KeyPair::from_seed(vec![].as_slice());
        let message = b"super secret message".to_vec();

        let signature = key.sign(Payload::Buffer(message.clone()));

        let is_valud = key.verify(Payload::Buffer(message), signature.as_slice());

        assert!(is_valud.map_or(false, |_| true));
    }

    #[test]
    fn did_document() {
        let key = DIDKey::generate_new(DIDKeyType::P256);

        let did_doc = key.to_did_document(Config {
            use_jose_format: false,
            serialize_secrets: true,
        });

        println!("{}", serde_json::to_string_pretty(&did_doc).unwrap())
    }
}
