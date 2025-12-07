use crate::curse::Curse;
use anyhow::Result;
use crate::curse::structs::fingerprint_structs::FingerprintMatches;

/// Calculate the CurseForge fingerprint for the `bytes` provided
///
/// CurseForge uses a modified version of [murmur2] where some bytes are stripped,
/// and the resulting bytes are hashes with seed `1`
pub fn cf_fingerprint(bytes: &[u8]) -> usize {
    // Implement CF's murmur2 modification
    let bytes = bytes
        .iter()
        .filter(|x| !matches!(x, 9 | 10 | 13 | 32))
        .copied()
        .collect::<Vec<u8>>();
    // Hash the contents using seed `1`
    murmur2::murmur2(&bytes, 1) as usize
}

impl Curse {
    pub async fn get_fingerprint_matches(
        &self,
        fingerprints: Vec<usize>,
    ) -> Result<FingerprintMatches> {
        #[derive(serde::Serialize)]
        #[serde(rename_all = "camelCase")]
        struct GetFingerprintMatchesRequestBody {
            fingerprints: Vec<usize>,
        }

        Ok(self
            .post(
                self.base_url.join("fingerprints")?,
                &GetFingerprintMatchesRequestBody { fingerprints },
            )
            .await?
            .data)
    }
}