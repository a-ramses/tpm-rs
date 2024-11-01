use crate::{
    platform::{CryptoRandom, TpmBuffers, TpmContextDeps},
    req_resp::RequestThenResponse,
};
use tpm2_rs_base::errors::TpmRcError;

/// The context that all command handler functions are given access to in order for them to process
/// their given command.
pub struct CommandHandler<Deps: TpmContextDeps> {
    /// Gives access to cryptographic operations.
    crypto: Deps::Crypto,
}

impl<Deps: TpmContextDeps> CommandHandler<Deps> {
    /// Creates a new [`TpmContext`] object that processes incoming TPM requests.
    pub fn new(crypto: Deps::Crypto) -> Self {
        Self { crypto }
    }
    /// Handles the [TpmCc::GetRandom] (`0x17B`) command.
    pub fn get_random(
        &mut self,
        request_response: RequestThenResponse<impl TpmBuffers>,
    ) -> Result<(), TpmRcError> {
        let mut request = request_response;
        let requested_bytes = request.read_be_u16().ok_or(TpmRcError::CommandSize)? as usize;

        let mut response = request.into_response();
        response
            .write_callback(requested_bytes, |buffer| {
                self.crypto.get_random_bytes(buffer)
            })
            .map_err(|_| TpmRcError::Memory)?;

        Ok(())
    }
}
