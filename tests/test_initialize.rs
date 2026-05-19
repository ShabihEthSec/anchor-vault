use {
    anchor_lang::{
        system_program::ID as SYSTEM_PROGRAM_ID, AccountDeserialize, InstructionData,
        ToAccountMetas,
    },
    litesvm::LiteSVM,
    solana_instruction::Instruction,
    solana_keypair::Keypair,
    solana_message::Message,
    solana_pubkey::Pubkey,
    solana_signer::Signer,
    solana_transaction::Transaction,
};

fn setup() -> (LiteSVM, Keypair) {
    let program_id = anchor_vault_q2_2026::id();
    let payer = Keypair::new();
    let mut svm = LiteSVM::new();
    let bytes = include_bytes!("../target/deploy/anchor_vault_q2_2026.so");
    svm.add_program(program_id, bytes).unwrap();
    svm.airdrop(&payer.pubkey(), 10_000_000_000).unwrap();
    (svm, payer)
}

#[test]
fn test_initialize_deposit_withdraw_close() {
    let (mut svm, payer) = setup();
    let user = payer.pubkey();

    let (vault_state_pda, state_bump) =
        Pubkey::find_program_address(&[b"state", user.as_ref()], &anchor_vault_q2_2026::id());

    let (vault_pda, vault_bump) = Pubkey::find_program_address(
        &[b"vault", vault_state_pda.as_ref()],
        &anchor_vault_q2_2026::id(),
    );

    let initialize_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Initialize {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Initialize {}.data(),
    };

    let message = Message::new(&[initialize_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    svm.send_transaction(transaction).unwrap();

    let vault_state_account = svm.get_account(&vault_state_pda).unwrap();
    let vault_state = anchor_vault_q2_2026::state::VaultState::try_deserialize(
        &mut vault_state_account.data.as_ref(),
    )
    .unwrap();

    assert_eq!(vault_state.vault_bump, vault_bump);
    assert_eq!(vault_state.state_bump, state_bump);

    let deposit_amount = 1_000_000_000;
    let user_balance_before_deposit = svm.get_balance(&user).unwrap();

    let deposit_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Deposit {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Deposit {
            amount: deposit_amount,
        }
        .data(),
    };

    let message = Message::new(&[deposit_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    svm.send_transaction(transaction).unwrap();

    let vault_balance_after_deposit = svm.get_balance(&vault_pda).unwrap();
    assert_eq!(vault_balance_after_deposit, deposit_amount);
    assert!(svm.get_balance(&user).unwrap() < user_balance_before_deposit);

    let zero_deposit_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Deposit {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Deposit { amount: 0 }.data(),
    };

    let message = Message::new(&[zero_deposit_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    assert!(svm.send_transaction(transaction).is_err());
    assert_eq!(svm.get_balance(&vault_pda).unwrap(), deposit_amount);

    let withdraw_amount = 500_000_000;

    let withdraw_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Withdraw {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Withdraw {
            amount: withdraw_amount,
        }
        .data(),
    };

    let message = Message::new(&[withdraw_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    svm.send_transaction(transaction).unwrap();

    let vault_balance_after_withdraw = svm.get_balance(&vault_pda).unwrap();
    assert_eq!(
        vault_balance_after_withdraw,
        deposit_amount - withdraw_amount
    );

    let over_withdraw_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Withdraw {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Withdraw {
            amount: vault_balance_after_withdraw + 1,
        }
        .data(),
    };

    let message = Message::new(&[over_withdraw_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    assert!(svm.send_transaction(transaction).is_err());
    assert_eq!(
        svm.get_balance(&vault_pda).unwrap(),
        vault_balance_after_withdraw
    );

    let user_balance_before_close = svm.get_balance(&user).unwrap();
    let remaining_vault_balance = svm.get_balance(&vault_pda).unwrap();

    let close_ix = Instruction {
        program_id: anchor_vault_q2_2026::id(),
        accounts: anchor_vault_q2_2026::accounts::Close {
            user,
            vault_state: vault_state_pda,
            vault: vault_pda,
            system_program: SYSTEM_PROGRAM_ID,
        }
        .to_account_metas(None),
        data: anchor_vault_q2_2026::instruction::Close {}.data(),
    };

    let message = Message::new(&[close_ix], Some(&payer.pubkey()));
    let recent_blockhash = svm.latest_blockhash();
    let transaction = Transaction::new(&[&payer], message, recent_blockhash);
    svm.send_transaction(transaction).unwrap();

    assert!(svm.get_account(&vault_state_pda).is_none());

    let user_balance_after_close = svm.get_balance(&user).unwrap();
    assert!(user_balance_after_close >= user_balance_before_close + remaining_vault_balance);
    assert!(svm.get_account(&vault_pda).is_none());
}
