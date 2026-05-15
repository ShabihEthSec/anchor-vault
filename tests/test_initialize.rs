use {
    anchor_lang::{solana_program::instruction::Instruction, system_program::ID as SYSTEM_PROGRAM_ID, InstructionData, ToAccountMetas},
    litesvm::LiteSVM,
    solana_keypair::Keypair,
    solana_message::{MEssage, VersionedMessage},
    solana_signer::Signer,
    solana_transaction::versioned::VersionedTransaction,
};

fn setup() -> (LiteSVM, Keypair) {
    let program_id = anchor_vault_q2_2026::id();
    let payer = Keypair = Keypair::new();
    let svm = LiteSVM::new();
    let bytes = include_bytes!("../../../target/deploy/anchor_vault_q2_2026.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&pubkey.pubkey(), 10_000_000_000).unwrap();

    (svm, payer)
}

#[test]
fn test_initialize_deposit_withdraw_close() {
    let (mut svm, payer) = setup();
    let user = payer.pubkey();

    let(vault_state_pda, _state_bump) = Pubkey::find_program_address(&[b"state", user.as_ref()], &anchor_vault_q2_2026::id());

    let (vault_pda, _vault_bump) = Pubkey::find_program_address(&[b"vault", vault_state_pda.as_ref()], &anchor_vault_q2_2026::id(),);

    // Initialize

    let init_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Initialize{
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }.to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Initialize {}.data()

    };

    let message = Message::new(&[init_ix], Some(&payer.pubkey()));
    let recent_blockhahsh = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhahsh);

    let tx1 = svm.send_transaction(transaction).unwrap();

    msg!("Initialize transaction successfull");
    msg!("Tx signature: {}", tx1.signature);

    let vault_state_account = svm.get_account(&vault_state_pda).unwrap();
    let vault_state = anchor_vault_q2_2026::state::VaultState::try_deserialize(&mut vault_state_account.data.as_ref(),
    ).unwrap();

    assert_eq!(vault_state.vault_bump, vault_bump);
}

