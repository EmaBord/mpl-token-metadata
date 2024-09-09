#![cfg(feature = "test-bpf")]
pub mod utils;

use solana_program_test::*;
use token_metadata::{
    state::{Key, MasterEditionV2 as ProgramME},
    utils::try_from_slice_checked,
};
use utils::*;
mod serialization {

    use borsh::BorshDeserialize;

    use super::*;

    async fn setup(context: &mut ProgramTestContext) -> (Vec<u8>, Vec<u8>) {
        let test_metadata = Metadata::new();
        let test_master_edition = MasterEditionV2::new(&test_metadata);

        test_metadata.create_v3_default(context).await.unwrap();

        let _mint = get_mint(context, &test_master_edition.mint_pubkey).await;

        test_master_edition
            .create_v3(context, Some(10))
            .await
            .unwrap();

        let account = get_account(context, &test_metadata.pubkey).await;
        let me_account = get_account(context, &test_master_edition.pubkey).await;
        (account.data, me_account.data)
    }
    #[tokio::test]
    async fn success() {
        let mut context = program_test().start_with_context().await;
        let (_nft, master) = setup(&mut context).await;
        let otherbytes = master.clone();
        let _me: ProgramME = BorshDeserialize::deserialize(&mut &master[..]).unwrap();
        let _me2: ProgramME = try_from_slice_checked(&otherbytes, Key::MasterEditionV2, 0).unwrap();
        let _me2: ProgramME = try_from_slice_checked(&otherbytes, Key::MasterEditionV2, 0).unwrap();
    }
}
