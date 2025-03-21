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
struct TransactionDataRaydium2 {
    discriminator: u8,
    amount_in: u64,
    minimum_amount_out: u64,
}

fn main() {
    let data_simple: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 57, 190, 133, 229, 1, 0, 0, 0];

    let data_simple_2 = vec![
        9, 73, 193, 244, 52, 125, 0, 0, 0, 0, 195, 157, 208, 0, 0, 0, 0,
    ];

    //2ASD4XtVMFHJn6PwwGVTDaP74gJR5D54JfxjtLoH7zuEzzPX7yRP1Y67qwfS14mi73e15AWqcAZQ9dCYAsa7qvJv
    //other way with raydium
    let data_complex_raydium = vec![
        237, 147, 128, 186, 174, 146, 115, 207, 176, 155, 152, 249, 92, 134, 24, 204, 179, 190, 90,
        79, 46, 68,
    ];

    // println!("Simple Data:");

    let accounts_raydium = [
        Pubkey::from_str("2S4SJ9Ffyuvu246xc54buoJg6gZ7wQePoKFwSA3X7Vt3").unwrap(),
        Pubkey::from_str("4Gf1JjzqiwQopFQSuAEDi5Mo91njca33pkkNudDxGP7o").unwrap(),
        Pubkey::from_str("4UfgX8reB3eiKVxyeW1soxqAAPi6goc2JwtWGFVw47Dz").unwrap(),
        Pubkey::from_str("6ehEi3xc5DX3SVPiBpnRwPW3nQJBTzDynA26ce3AnYPp").unwrap(),
        Pubkey::from_str("7zesqXvg9WeVQCZk84gXAdYJxbrEtMnGyZ6z84yPtDdT").unwrap(),
        Pubkey::from_str("9CTxEyRStwTKLfVTS6c7rfQc7PTxY42YPdQcrHTv53Ao").unwrap(),
        Pubkey::from_str("At9hzW9QpTfZbjDQ1d1u742XK6kxRnera7iHuyz9g2MC").unwrap(),
        Pubkey::from_str("BRYfbo1FHerNTFwhPUu4x4PYHFYovVgBakesXYgKwNhR").unwrap(),
        Pubkey::from_str("CBie2Q8yT7tnbakYcGRDGYPjbmzVgMe5ZCEVXQamYsAq").unwrap(),
        Pubkey::from_str("CLJRTMaqkc2oq8jEWKAvshWuwBJLTSpZ6B9SVQ5k3Rb5").unwrap(),
        Pubkey::from_str("CtYsnSUcikqnLfGpmmDKrCWewXzL1YxpGRmyaZV2X3Fv").unwrap(),
        Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
        Pubkey::from_str("h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood").unwrap(),
        Pubkey::from_str("5GWu2jYc3SDCnBGqbz6ZHdF8WYbt8WuYpD6aNZynrC7A").unwrap(),
        Pubkey::from_str("5GWu2jYc3SDCnBGqbz6ZHdF8WYbt8WuYpD6aNZynrC7A").unwrap(),
    ];

    let program_id_raydium = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"); // Exemplo de PROGRA

    let swap_transaction = match teste::decode_raydium_instruction(
        data_simple_2.to_vec(),
        accounts_raydium.to_vec(),
        program_id_raydium.unwrap(),
    ) {
        Some(transaction) => {
            println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
        }
        None => {
            println!("❌ Erro: Falha ao decodificar a instrução Raydium.");
            return; // Ou retorne um valor padrão
        }
    };

    // println!("{}", serde_json::to_string_pretty(&parse_swap_data(&data_simple).unwrap()).unwrap());
    // println!("{}", serde_json::to_string_pretty(&parse_swap_jupiter(&data_complex).unwrap()).unwrap());
    // println!("{}", serde_json::to_string_pretty(&parse_swap_raydium2(&data_simple_2).unwrap()).unwrap());

    //https://solscan.io/tx/3mbhjY6S8XpJENThAgfNP2gEp8UNmLXCk5hCNff7KkErWXvKi2X6JUaaNYapvR4EF9fCc41G4P5GKimeAaBouFvM
    let data_complex_jupiter: [u8; 43] = [
        229, 23, 203, 151, 122, 227, 173, 42, 3, 0, 0, 0, 7, 100, 0, 1, 46, 100, 1, 2, 50, 100, 2,
        0, 1, 224, 83, 3, 0, 0, 0, 0, 170, 65, 84, 3, 0, 0, 0, 0, 0, 0, 0,
    ];

    let program_id_jupiter = Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"); // Exemplo de PROGRA

    let accounts_jupiter = [
        Pubkey::from_str("67aEBh9Eu4cEajDBMZ4PszgWaxZ4a6YPMsdfPMAGNdEz").unwrap(),
        Pubkey::from_str("AHDEyJPn3HpgpfoXaFgVn7qzwTKpz5CVkPVoDAPJLi2Y").unwrap(),
        Pubkey::from_str("EmqCo65C5i4aoTbtyPwW9iGfqmmPcxLATmyoPPjzGEJv").unwrap(),
        Pubkey::from_str("DeRSrVVXhwjE4URD8AsKqyZrzWVzEUeFEV3HNPcsx4cb").unwrap(),
        Pubkey::from_str("2uvJ73NVLvdnkdVYwwwDRmPb3R6gxc1MA2zY7g6ekYxz").unwrap(),
        Pubkey::from_str("4GCPz8jmdfuNYvz8AmVrGa2ZjUijS4wZK9skSDWQd9ne").unwrap(),
        Pubkey::from_str("3eY7p2T7dy4jkasJjRZDqgFBiz7TwyTc6sbUonWvrmPA").unwrap(),
        Pubkey::from_str("6LHmrpEGr8PtiJNJkGEC6QGQMo2E4XTQ3Cfgp4zCvT3n").unwrap(),
        Pubkey::from_str("CebN5WGQ4jvEPvsVU4EoHEpgzq1VV7AbicfhtW4xC9iM").unwrap(),
        Pubkey::from_str("EYtSBZAGbLPpo4A648kYYBHvJYkCBdZy1UzG8WFvj7X6").unwrap(),
        Pubkey::from_str("bjSFxomudZUNJ5azMhE481evSW8UWPnHxPmXQCF8zek").unwrap(),
        Pubkey::from_str("DttWaMuVvTiduZRnguLF7jNxTgiMBZ1hyAumKUiL2KRL").unwrap(),
        Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
        Pubkey::from_str("11111111111111111111111111111111").unwrap(),
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        Pubkey::from_str("h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood").unwrap(),
        Pubkey::from_str("AwBDLyCE4U9UGFAryq88uuw5Cp7ZGjWYgwGTAgKjpump").unwrap(),
        Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap(),
        Pubkey::from_str("C7Cx2pMLtjybS3mDKSfsBj4zQ3PRZGkKt7RCYTTbCSx2").unwrap(),
        Pubkey::from_str("4wTV1YmiEkRvAtNtsSGPtUrqRYQMe5SKy2uB4Jjaxnjf").unwrap(),
    ];

    let swap_transaction = match teste::decode_jupiter_instruction(
        data_complex_jupiter.to_vec(),
        accounts_jupiter.to_vec(),
        program_id_jupiter.unwrap(),
    ) {
        Some(transaction) => {
            println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
        }
        None => {
            println!("❌ Erro: Falha ao decodificar a instrução Raydium.");
            return; // Ou retorne um valor padrão
        }
    };

    // // OKX TRANSACTION
    // let okx_data = vec![
    //     173, 131, 78, 38, 150, 165, 123, 15, 255, 31, 39, 136, 0, 0, 0, 0, 43, 218, 27, 0, 0, 0, 0,
    //     0, 156, 132, 27, 0, 0, 0, 0, 0, 1, 0, 0, 0, 255, 31, 39, 136, 0, 0, 0, 0, 1, 0, 0, 0, 1, 0,
    //     0, 0, 1, 0, 0, 0, 4, 1, 0, 0, 0, 100, 85, 0, 0, 253, 154, 1, 0, 0, 0, 0, 0,
    // ];

    // // // Defina o PROGRAM_ID (substitua pelo valor real)
    // let program_id_okx = Pubkey::from_str("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma"); // Exemplo de PROGRA

    // let accounts_okx = [
    //     Pubkey::from_str("EmmTYzToovenVxzrFQ8og7NCZU5rG5etYgdBWxnRq6qm").unwrap(),
    //     Pubkey::from_str("ByEc832xaRvwSgmbdMUG57tBptzYq6EFW2emGFm4HJmv").unwrap(),
    //     Pubkey::from_str("BDgjLdYBpj1AKW97foiYM1qbXghwLUDDVXaRbbAc41WU").unwrap(),
    //     Pubkey::from_str("h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood").unwrap(),
    //     Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
    //     Pubkey::from_str("6Wzuv7vLc6Vq8HJcHwwSCE9SKcdJiuoJmJm3EMFkWERN").unwrap(),
    //     Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
    // ];

    // decode_okx_instruction(okx_data, accounts_okx.to_vec(), program_id_okx.unwrap());
}
