use binding_macro::{cycles, service};
use protocol::traits::ServiceSDK;
use protocol::types::{Metadata, ServiceContext, METADATA_KEY};
use protocol::ProtocolResult;

pub struct MetadataService<SDK: ServiceSDK> {
    sdk: SDK,
}

#[service]
impl<SDK: 'static + ServiceSDK> MetadataService<SDK> {
    pub fn new(sdk: SDK) -> ProtocolResult<Self> {
        Ok(Self { sdk })
    }

    #[cycles(210_00)]
    #[read]
    fn get_metadata(&self, ctx: ServiceContext) -> ProtocolResult<Metadata> {
        let metadata: Metadata = self
            .sdk
            .get_value(&METADATA_KEY.to_owned())?
            .expect("Metadata should always be in the genesis block");
        Ok(metadata)
    }
}