use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::ArrangeAccounts;
use carbon_core::instruction::InstructionDecoder;
use carbon_core::{borsh, CarbonDeserialize};
use carbon_jupiter_swap_decoder::accounts::JupiterSwapAccount;
use carbon_jupiter_swap_decoder::instructions::route::Route;
use carbon_jupiter_swap_decoder::instructions::shared_accounts_exact_out_route::SharedAccountsExactOutRoute;
use carbon_jupiter_swap_decoder::instructions::shared_accounts_route::SharedAccountsRoute;
use carbon_jupiter_swap_decoder::instructions::swap_event::SwapEvent;
use carbon_jupiter_swap_decoder::instructions::JupiterSwapInstruction;
use carbon_jupiter_swap_decoder::types;
use carbon_jupiter_swap_decoder::JupiterSwapDecoder;
use carbon_okx_dex_decoder::accounts::OkxDexAccount;
use carbon_okx_dex_decoder::instructions::commission_spl_swap::CommissionSplSwap;
use carbon_okx_dex_decoder::instructions::commission_spl_swap2::CommissionSplSwap2;
use carbon_okx_dex_decoder::instructions::swap::Swap;
use carbon_okx_dex_decoder::instructions::swap2::Swap2;
use carbon_okx_dex_decoder::instructions::OkxDexInstruction;
use carbon_okx_dex_decoder::OkxDexDecoder;
use carbon_raydium_amm_v4_decoder::instructions::swap_base_in::SwapBaseIn;
use carbon_raydium_amm_v4_decoder::instructions::swap_base_out::SwapBaseOut;
use carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction;
use carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder;
use serde::{Deserialize, Serialize};
use serde_json::json;
use solana_sdk::account::Account;
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::convert::TryInto;
use std::mem::size_of;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapTransaction {
    pub in_amount: u64,
    pub out_amount: Option<u64>,
    pub mint_token_in: Option<Pubkey>,
    pub mint_token_out: Option<Pubkey>,
    pub mint_token_account_in: Option<Pubkey>,
    pub mint_token_account_out: Option<Pubkey>,
}

pub fn decode_raydium_instruction(
    data: Vec<u8>,
    accounts: Vec<Pubkey>,
    program_id: Pubkey,
) -> Option<SwapTransaction> {
    let decoder: RaydiumAmmV4Decoder = RaydiumAmmV4Decoder;

    let account_metas: Vec<AccountMeta> = accounts
        .iter()
        .map(|pubkey| AccountMeta {
            pubkey: *pubkey,
            is_signer: false,   // Defina como true se a conta for um signatário
            is_writable: false, // Defina como true se a conta for gravável
        })
        .collect();

    let instruction = Instruction {
        program_id,
        accounts: account_metas, // Adicione as contas necessárias
        data,                    // Adicione os dados da instrução
    };

    match decoder.decode_instruction(&instruction) {
        Some(decoded_instruction) => match decoded_instruction.data {
            RaydiumAmmV4Instruction::SwapBaseIn(ref swap_data) => {
                let arranged_accounts =
                    SwapBaseIn::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    in_amount: swap_data.amount_in,
                    out_amount: Some(swap_data.minimum_amount_out),
                    mint_token_in: None,
                    mint_token_out: None,
                    mint_token_account_in: Some(arranged_accounts.uer_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.uer_destination_token_account),
                };

                return Some(swap);
            }
            RaydiumAmmV4Instruction::SwapBaseOut(ref swap_data) => {
                let arranged_accounts =
                    SwapBaseOut::arrange_accounts(&instruction.accounts).unwrap();
                let swap = SwapTransaction {
                    in_amount: swap_data.amount_out,
                    out_amount: Some(swap_data.amount_out),
                    mint_token_in: None,
                    mint_token_out: None,
                    mint_token_account_in: Some(arranged_accounts.uer_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.uer_destination_token_account),
                };

                return Some(swap);
            }
            _ => {
                return None;
            }
        },
        None => {
            return None;
        }
    }
}

pub fn decode_jupiter_instruction(
    data: Vec<u8>,
    accounts: Vec<Pubkey>,
    program_id: Pubkey,
) -> Option<SwapTransaction> {
    let decoder = JupiterSwapDecoder;

    let account_metas: Vec<AccountMeta> = accounts
        .iter()
        .map(|pubkey| AccountMeta {
            pubkey: *pubkey,
            is_signer: false,   // Defina como true se a conta for um signatário
            is_writable: false, // Defina como true se a conta for gravável
        })
        .collect();

    let instruction = Instruction {
        program_id,
        accounts: account_metas, // Adicione as contas necessárias
        data: data,              // Adicione os dados da instrução
    };

    match decoder.decode_instruction(&instruction) {
        Some(decoded_instruction) => match decoded_instruction.data {
            JupiterSwapInstruction::Route(ref data) => {
                let arranged_accounts = Route::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    in_amount: data.in_amount,
                    out_amount: Some(data.quoted_out_amount),
                    mint_token_in: None,
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.user_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.user_destination_token_account),
                };

                return Some(swap);
            }
            JupiterSwapInstruction::SharedAccountsExactOutRoute(ref data) => {
                let arranged_accounts =
                    SharedAccountsExactOutRoute::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    in_amount: data.out_amount,
                    out_amount: Some(data.out_amount),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                };

                return Some(swap);
            }
            JupiterSwapInstruction::SharedAccountsRoute(ref data) => {
                let arranged_accounts =
                    SharedAccountsRoute::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    in_amount: data.in_amount,
                    out_amount: Some(data.quoted_out_amount),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                };

                return Some(swap);
            }
            JupiterSwapInstruction::SwapEvent(ref data) => {
                let swap = SwapTransaction {
                    in_amount: data.input_amount,
                    out_amount: Some(data.output_amount),
                    mint_token_in: Some(data.input_mint),
                    mint_token_out: Some(data.output_mint),
                    mint_token_account_in: None,
                    mint_token_account_out: None,
                };

                return Some(swap);
            }
            _ => {
                return None;
            }
        },
        None => {
            return None;
        }
    }
}

pub fn decode_okx_instruction(
    okx_data: Vec<u8>,
    accounts: Vec<Pubkey>,
    program_id: Pubkey,
) -> Option<SwapTransaction> {
    let decoder = OkxDexDecoder;

    let account_metas: Vec<AccountMeta> = accounts
        .iter()
        .map(|pubkey| AccountMeta {
            pubkey: *pubkey,
            is_signer: false,   // Defina como true se a conta for um signatário
            is_writable: false, // Defina como true se a conta for gravável
        })
        .collect();

    let instruction = Instruction {
        program_id,
        accounts: account_metas, // Adicione as contas necessárias
        data: okx_data,          // Adicione os dados da instrução
    };

    match decoder.decode_instruction(&instruction) {
        Some(decoded_instruction) => {
            println!("Instrução decodificada: {:?}", decoded_instruction);
            match decoded_instruction.data {
                OkxDexInstruction::Swap(ref swap_data) => {
                    let arranged_accounts = Swap::arrange_accounts(&instruction.accounts).unwrap();

                    let swap = SwapTransaction {
                        in_amount: swap_data.data.amount_in,
                        out_amount: Some(swap_data.data.expect_amount_out),
                        mint_token_in: Some(arranged_accounts.source_mint),
                        mint_token_out: Some(arranged_accounts.destination_mint),
                        mint_token_account_in: Some(arranged_accounts.source_token_account),
                        mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    };

                    return Some(swap);
                }
                OkxDexInstruction::Swap2(ref swap_data) => {
                    let arranged_accounts = Swap2::arrange_accounts(&instruction.accounts).unwrap();

                    let swap = SwapTransaction {
                        in_amount: swap_data.data.amount_in,
                        out_amount: Some(swap_data.data.expect_amount_out),
                        mint_token_in: Some(arranged_accounts.source_mint),
                        mint_token_out: Some(arranged_accounts.destination_mint),
                        mint_token_account_in: Some(arranged_accounts.source_token_account),
                        mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    };

                    return Some(swap);
                }
                OkxDexInstruction::CommissionSplSwap(ref swap_data) => {
                    let arranged_accounts =
                        CommissionSplSwap::arrange_accounts(&instruction.accounts).unwrap();

                    let swap = SwapTransaction {
                        in_amount: swap_data.data.amount_in,
                        out_amount: Some(swap_data.data.expect_amount_out),
                        mint_token_in: Some(arranged_accounts.source_mint),
                        mint_token_out: Some(arranged_accounts.destination_mint),
                        mint_token_account_in: Some(arranged_accounts.source_token_account),
                        mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    };

                    return Some(swap);
                }
                OkxDexInstruction::CommissionSplSwap2(ref swap_data) => {
                    let arranged_accounts =
                        CommissionSplSwap2::arrange_accounts(&instruction.accounts).unwrap();

                    let swap = SwapTransaction {
                        in_amount: swap_data.data.amount_in,
                        out_amount: Some(swap_data.data.expect_amount_out),
                        mint_token_in: Some(arranged_accounts.source_mint),
                        mint_token_out: Some(arranged_accounts.destination_mint),
                        mint_token_account_in: Some(arranged_accounts.source_token_account),
                        mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    };

                    return Some(swap);
                }
                _ => {
                    return None;
                }
            }
        }
        None => {
            return None;
        }
    }
}
