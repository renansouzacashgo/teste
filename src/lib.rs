use carbon_core::account::AccountDecoder;
use carbon_core::deserialize::ArrangeAccounts;
use carbon_core::instruction::InstructionDecoder;
use carbon_core::{borsh, CarbonDeserialize};
use carbon_jupiter_swap_decoder::accounts::JupiterSwapAccount;
use carbon_jupiter_swap_decoder::instructions::exact_out_route::ExactOutRoute;
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
use carbon_okx_dex_decoder::instructions::{
    commission_sol_from_swap, commission_sol_proxy_swap, commission_sol_swap, commission_sol_swap2,
    commission_spl_from_swap, commission_spl_proxy_swap, from_swap_log, proxy_swap,
    OkxDexInstruction,
};
use carbon_okx_dex_decoder::OkxDexDecoder;
use carbon_pump_swap_decoder::accounts::PumpSwapAccount;
use carbon_pump_swap_decoder::instructions::buy::Buy;
use carbon_pump_swap_decoder::instructions::sell::Sell;
use carbon_pump_swap_decoder::instructions::PumpSwapInstruction;
use carbon_pump_swap_decoder::PumpSwapDecoder;
use carbon_pumpfun_decoder::accounts::PumpfunAccount;
use carbon_pumpfun_decoder::instructions::create::Create;
use carbon_pumpfun_decoder::instructions::PumpfunInstruction;
use carbon_pumpfun_decoder::PumpfunDecoder;
use carbon_raydium_amm_v4_decoder::instructions::swap_base_in::SwapBaseIn;
use carbon_raydium_amm_v4_decoder::instructions::swap_base_out::SwapBaseOut;
use carbon_raydium_amm_v4_decoder::instructions::RaydiumAmmV4Instruction;
use carbon_raydium_amm_v4_decoder::RaydiumAmmV4Decoder;

use carbon_raydium_launchpad_decoder::instructions::buy_exact_in::BuyExactIn;
use carbon_raydium_launchpad_decoder::instructions::buy_exact_out::BuyExactOut;
use carbon_raydium_launchpad_decoder::instructions::RaydiumLaunchpadInstruction;
use carbon_raydium_launchpad_decoder::RaydiumLaunchpadDecoder;
use serde::{Deserialize, Serialize};
use solana_sdk::instruction::{AccountMeta, Instruction};
use solana_sdk::pubkey::Pubkey;
use std::any::type_name;
use std::str::FromStr;

#[derive(Serialize, Deserialize, Debug)]
pub struct SwapTransaction {
    pub amm: Option<Pubkey>,
    pub in_amount: u64,
    pub out_amount: Option<u64>,
    pub mint_token_in: Option<Pubkey>,
    pub mint_token_out: Option<Pubkey>,
    pub mint_token_account_in: Option<Pubkey>,
    pub mint_token_account_out: Option<Pubkey>,
    pub user: Option<Pubkey>,
    pub mint: Option<Pubkey>,
    pub create_instruction_accounts:
        Option<carbon_pumpfun_decoder::instructions::create::CreateInstructionAccounts>,
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
                    amm: Some(arranged_accounts.amm),
                    in_amount: swap_data.amount_in,
                    out_amount: Some(swap_data.minimum_amount_out),
                    mint_token_in: None,
                    mint_token_out: None,
                    mint_token_account_in: Some(arranged_accounts.user_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.user_destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            RaydiumAmmV4Instruction::SwapBaseOut(ref swap_data) => {
                let arranged_accounts =
                    SwapBaseOut::arrange_accounts(&instruction.accounts).unwrap();
                let swap = SwapTransaction {
                    amm: Some(arranged_accounts.amm),
                    in_amount: swap_data.amount_out,
                    out_amount: Some(swap_data.amount_out),
                    mint_token_in: None,
                    mint_token_out: None,
                    mint_token_account_in: Some(arranged_accounts.user_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.user_destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
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

pub fn decode_pumpfun_instruction(
    data: Vec<u8>,
    accounts: Vec<Pubkey>,
    program_id: Pubkey,
) -> Option<SwapTransaction> {
    let decoder = PumpfunDecoder;

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
            PumpfunInstruction::Create(ref data) => {
                let arranged_accounts = Create::arrange_accounts(&instruction.accounts).unwrap();

                let swap: SwapTransaction = SwapTransaction {
                    amm: Some(arranged_accounts.program),
                    in_amount: 0,
                    out_amount: None,
                    mint_token_in: None,
                    mint_token_out: None,
                    mint_token_account_in: None,
                    mint_token_account_out: None,
                    user: Some(arranged_accounts.user),
                    mint: Some(arranged_accounts.mint),
                    create_instruction_accounts: Some(arranged_accounts),
                };

                return Some(swap);
            }
            PumpfunInstruction::Buy(ref data) => {
                let arranged_accounts =
                    carbon_pumpfun_decoder::instructions::buy::Buy::arrange_accounts(
                        &instruction.accounts,
                    )
                    .unwrap();

                let swap: SwapTransaction = SwapTransaction {
                    amm: Some(arranged_accounts.program),
                    in_amount: data.max_sol_cost,
                    out_amount: Some(data.amount),
                    mint_token_in: Some(
                        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
                    ),
                    mint_token_out: Some(arranged_accounts.mint),
                    mint_token_account_in: Some(arranged_accounts.associated_user),
                    mint_token_account_out: Some(arranged_accounts.associated_user),
                    user: Some(arranged_accounts.user),
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            PumpfunInstruction::Sell(ref data) => {
                let arranged_accounts =
                    carbon_pumpfun_decoder::instructions::sell::Sell::arrange_accounts(
                        &instruction.accounts,
                    )
                    .unwrap();

                let swap = SwapTransaction {
                    amm: Some(arranged_accounts.program),
                    in_amount: data.amount,
                    out_amount: Some(data.min_sol_output),
                    mint_token_in: Some(arranged_accounts.mint),
                    mint_token_out: Some(
                        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
                    ),
                    mint_token_account_in: Some(arranged_accounts.associated_user),
                    mint_token_account_out: Some(arranged_accounts.associated_user),
                    user: Some(arranged_accounts.user),
                    mint: None,
                    create_instruction_accounts: None,
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

pub fn decode_pumpswap_instruction(
    data: Vec<u8>,
    accounts: Vec<Pubkey>,
    program_id: Pubkey,
) -> Option<SwapTransaction> {
    let decoder = PumpSwapDecoder;

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
            // PumpSwapInstruction::Buy(ref data) => {
            //     let arranged_accounts = Buy::arrange_accounts(&instruction.accounts).unwrap();

            //     let swap: SwapTransaction = SwapTransaction {
            //         amm: Some(arranged_accounts.program),
            //         in_amount: data.base_amount_out,
            //         out_amount: Some(data.max_quote_amount_in),
            //         mint_token_in: Some(arranged_accounts.base_mint),
            //         mint_token_out: Some(arranged_accounts.quote_mint),
            //         mint_token_account_in: Some(arranged_accounts.user_base_token_account),
            //         mint_token_account_out: Some(arranged_accounts.user_quote_token_account),
            //     };

            //     return Some(swap);
            // }
            PumpSwapInstruction::Sell(ref data) => {
                let arranged_accounts = Sell::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: Some(arranged_accounts.program),
                    in_amount: data.base_amount_in,
                    out_amount: Some(data.min_quote_amount_out),
                    mint_token_in: Some(arranged_accounts.base_mint),
                    mint_token_out: Some(arranged_accounts.quote_mint),
                    mint_token_account_in: Some(arranged_accounts.user_base_token_account),
                    mint_token_account_out: Some(arranged_accounts.user_quote_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
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
                    amm: None,
                    in_amount: data.in_amount,
                    out_amount: Some(data.quoted_out_amount),
                    mint_token_in: None,
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.user_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.user_destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            JupiterSwapInstruction::SharedAccountsExactOutRoute(ref data) => {
                let arranged_accounts =
                    SharedAccountsExactOutRoute::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: data.out_amount,
                    out_amount: Some(data.out_amount),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            JupiterSwapInstruction::SharedAccountsRoute(ref data) => {
                let arranged_accounts =
                    SharedAccountsRoute::arrange_accounts(&instruction.accounts).unwrap();
                let swap = SwapTransaction {
                    amm: None,
                    in_amount: data.in_amount,
                    out_amount: Some(data.quoted_out_amount),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            JupiterSwapInstruction::SwapEvent(ref data) => {
                let swap = SwapTransaction {
                    amm: None,
                    in_amount: data.input_amount,
                    out_amount: Some(data.output_amount),
                    mint_token_in: Some(data.input_mint),
                    mint_token_out: Some(data.output_mint),
                    mint_token_account_in: None,
                    mint_token_account_out: None,
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };
                return Some(swap);
            }
            JupiterSwapInstruction::ExactOutRoute(ref data) => {
                let arranged_accounts =
                    ExactOutRoute::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: data.quoted_in_amount,
                    out_amount: Some(data.out_amount),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.user_destination_token_account),
                    mint_token_account_in: Some(arranged_accounts.user_source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
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
        Some(decoded_instruction) => match decoded_instruction.data {
            OkxDexInstruction::Swap(ref swap_data) => {
                let arranged_accounts = Swap::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::Swap2(ref swap_data) => {
                let arranged_accounts = Swap2::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSplSwap(ref swap_data) => {
                let arranged_accounts =
                    CommissionSplSwap::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSplSwap2(ref swap_data) => {
                let arranged_accounts =
                    CommissionSplSwap2::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSplFromSwap(ref swap_data) => {
                let arranged_accounts =
                    commission_spl_from_swap::CommissionSplFromSwap::arrange_accounts(
                        &instruction.accounts,
                    )
                    .unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.args.amount_in,
                    out_amount: Some(swap_data.args.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSplProxySwap(ref swap_data) => {
                let arranged_accounts =
                    commission_spl_proxy_swap::CommissionSplProxySwap::arrange_accounts(
                        &instruction.accounts,
                    )
                    .unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSolFromSwap(ref swap_data) => {
                let arranged_accounts =
                    commission_sol_from_swap::CommissionSolFromSwap::arrange_accounts(
                        &instruction.accounts,
                    )
                    .unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.args.amount_in,
                    out_amount: Some(swap_data.args.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSolProxySwap(ref swap_data) => {
                let arranged_accounts =
                    commission_sol_proxy_swap::CommissionSolProxySwap::arrange_accounts(
                        &instruction.accounts,
                    )
                    .unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSolSwap(ref swap_data) => {
                let arranged_accounts =
                    commission_sol_swap::CommissionSolSwap::arrange_accounts(&instruction.accounts)
                        .unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::CommissionSolSwap2(ref swap_data) => {
                let arranged_accounts = commission_sol_swap2::CommissionSolSwap2::arrange_accounts(
                    &instruction.accounts,
                )
                .unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::FromSwapLog(ref swap_data) => {
                let arranged_accounts =
                    from_swap_log::FromSwapLog::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.args.amount_in,
                    out_amount: Some(swap_data.args.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            OkxDexInstruction::ProxySwap(ref swap_data) => {
                let arranged_accounts =
                    proxy_swap::ProxySwap::arrange_accounts(&instruction.accounts).unwrap();

                let swap = SwapTransaction {
                    amm: None,
                    in_amount: swap_data.data.amount_in,
                    out_amount: Some(swap_data.data.expect_amount_out),
                    mint_token_in: Some(arranged_accounts.source_mint),
                    mint_token_out: Some(arranged_accounts.destination_mint),
                    mint_token_account_in: Some(arranged_accounts.source_token_account),
                    mint_token_account_out: Some(arranged_accounts.destination_token_account),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
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

pub fn decode_raydiumlaunchpad_instruction(
    okx_data: Vec<u8>,
    accounts: Vec<Pubkey>,
    program_id: Pubkey,
) -> Option<SwapTransaction> {
    let decoder = RaydiumLaunchpadDecoder;

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
        Some(decoded_instruction) => match decoded_instruction.data {
            RaydiumLaunchpadInstruction::BuyExactIn(ref swap_data) => {
                let arranged_accounts =
                    BuyExactIn::arrange_accounts(&instruction.accounts).unwrap();
                let swap = SwapTransaction {
                    amm: Some(arranged_accounts.program),
                    in_amount: swap_data.amount_in,
                    out_amount: Some(swap_data.minimum_amount_out),
                    mint_token_in: Some(arranged_accounts.base_token_mint),
                    mint_token_out: Some(arranged_accounts.quote_token_mint),
                    mint_token_account_in: Some(arranged_accounts.user_base_token),
                    mint_token_account_out: Some(arranged_accounts.user_quote_token),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
                };

                return Some(swap);
            }
            RaydiumLaunchpadInstruction::BuyExactOut(ref swap_data) => {
                let arranged_accounts =
                    BuyExactOut::arrange_accounts(&instruction.accounts).unwrap();
                let swap = SwapTransaction {
                    amm: Some(arranged_accounts.program),
                    in_amount: swap_data.amount_out,
                    out_amount: Some(swap_data.maximum_amount_in),
                    mint_token_in: Some(arranged_accounts.base_token_mint),
                    mint_token_out: Some(arranged_accounts.quote_token_mint),
                    mint_token_account_in: Some(arranged_accounts.user_base_token),
                    mint_token_account_out: Some(arranged_accounts.user_quote_token),
                    user: None,
                    mint: None,
                    create_instruction_accounts: None,
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
