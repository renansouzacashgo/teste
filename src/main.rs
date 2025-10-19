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
    // let data_simple: [u8; 16] = [0, 0, 0, 0, 0, 0, 0, 0, 57, 190, 133, 229, 1, 0, 0, 0];

    // let data_simple_2 = vec![
    //     9, 73, 193, 244, 52, 125, 0, 0, 0, 0, 195, 157, 208, 0, 0, 0, 0,
    // ];

    // //2ASD4XtVMFHJn6PwwGVTDaP74gJR5D54JfxjtLoH7zuEzzPX7yRP1Y67qwfS14mi73e15AWqcAZQ9dCYAsa7qvJv
    // //other way with raydium
    // let data_complex_raydium = vec![
    //     237, 147, 128, 186, 174, 146, 115, 207, 176, 155, 152, 249, 92, 134, 24, 204, 179, 190, 90,
    //     79, 46, 68,
    // ];

    // // println!("Simple Data:");

    // let accounts_raydium = [
    //     Pubkey::from_str("2S4SJ9Ffyuvu246xc54buoJg6gZ7wQePoKFwSA3X7Vt3").unwrap(),
    //     Pubkey::from_str("4Gf1JjzqiwQopFQSuAEDi5Mo91njca33pkkNudDxGP7o").unwrap(),
    //     Pubkey::from_str("4UfgX8reB3eiKVxyeW1soxqAAPi6goc2JwtWGFVw47Dz").unwrap(),
    //     Pubkey::from_str("6ehEi3xc5DX3SVPiBpnRwPW3nQJBTzDynA26ce3AnYPp").unwrap(),
    //     Pubkey::from_str("7zesqXvg9WeVQCZk84gXAdYJxbrEtMnGyZ6z84yPtDdT").unwrap(),
    //     Pubkey::from_str("9CTxEyRStwTKLfVTS6c7rfQc7PTxY42YPdQcrHTv53Ao").unwrap(),
    //     Pubkey::from_str("At9hzW9QpTfZbjDQ1d1u742XK6kxRnera7iHuyz9g2MC").unwrap(),
    //     Pubkey::from_str("BRYfbo1FHerNTFwhPUu4x4PYHFYovVgBakesXYgKwNhR").unwrap(),
    //     Pubkey::from_str("CBie2Q8yT7tnbakYcGRDGYPjbmzVgMe5ZCEVXQamYsAq").unwrap(),
    //     Pubkey::from_str("CLJRTMaqkc2oq8jEWKAvshWuwBJLTSpZ6B9SVQ5k3Rb5").unwrap(),
    //     Pubkey::from_str("CtYsnSUcikqnLfGpmmDKrCWewXzL1YxpGRmyaZV2X3Fv").unwrap(),
    //     Pubkey::from_str("11111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
    //     Pubkey::from_str("h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood").unwrap(),
    //     Pubkey::from_str("5GWu2jYc3SDCnBGqbz6ZHdF8WYbt8WuYpD6aNZynrC7A").unwrap(),
    //     Pubkey::from_str("5GWu2jYc3SDCnBGqbz6ZHdF8WYbt8WuYpD6aNZynrC7A").unwrap(),
    // ];

    // let program_id_raydium = Pubkey::from_str("675kPX9MHTjS2zt1qfr1NYHuzeLXfQM9H24wFSUt1Mp8"); // Exemplo de PROGRA

    // let swap_transaction = match teste::decode_raydium_instruction(
    //     data_simple_2.to_vec(),
    //     accounts_raydium.to_vec(),
    //     program_id_raydium.unwrap(),
    // ) {
    //     Some(transaction) => {
    //         println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
    //     }
    //     None => {
    //         println!("❌ Erro: Falha ao decodificar a instrução Raydium.");
    //         return; // Ou retorne um valor padrão
    //     }
    // };

    // println!("{}", serde_json::to_string_pretty(&parse_swap_data(&data_simple).unwrap()).unwrap());
    // println!("{}", serde_json::to_string_pretty(&parse_swap_jupiter(&data_complex).unwrap()).unwrap());
    // println!("{}", serde_json::to_string_pretty(&parse_swap_raydium2(&data_simple_2).unwrap()).unwrap());

    //https://solscan.io/tx/3mbhjY6S8XpJENThAgfNP2gEp8UNmLXCk5hCNff7KkErWXvKi2X6JUaaNYapvR4EF9fCc41G4P5GKimeAaBouFvM
    // let data_complex_jupiter: [u8; 43] = [
    //     229, 23, 203, 151, 122, 227, 173, 42, 3, 0, 0, 0, 7, 100, 0, 1, 46, 100, 1, 2, 50, 100, 2,
    //     0, 1, 224, 83, 3, 0, 0, 0, 0, 170, 65, 84, 3, 0, 0, 0, 0, 0, 0, 0,
    // ];

    // let data_complex_jupiter=  vec![41, 108, 130, 211, 98, 89, 182, 101, 169, 49, 143, 75, 98, 95, 15, 68, 35, 233, 226, 250, 169, 21, 161, 56, 226, 38, 143, 187, 226, 60, 39, 63, 92, 158, 66, 32, 72, 241, 80, 72, 22, 27, 103, 124, 116, 170, 155, 100];
    // let data_complex_jupiter = vec![193, 32, 155, 51, 65, 214, 156, 129, 6, 2, 0, 0, 0, 7, 100, 0, 1, 72, 100, 1, 2, 213, 240, 86, 18, 134, 0, 0, 0, 208, 8, 105, 243, 3, 0, 0, 0, 84, 1, 85];

    // let program_id_jupiter = Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4"); // Exemplo de PROGRA

    // let accounts_jupiter = [
    //     Pubkey::from_str("9CVVvykRbB4zggp1vnZUKqTBKw1742XJ1FVE8rGfAixV").unwrap(),
    //     Pubkey::from_str("MjKAkypjNtPcFdyjc4KoderNKnmVuxvzWGxb94WfxZq").unwrap(),
    //     Pubkey::from_str("2ndNJTz8Xc7mPYnv5yd1oMF8HYFb9CA7BRRe1cccJrUj").unwrap(),
    //     Pubkey::from_str("8JpRt3vBn7VsoESnNrWU1EAGm6Ha8pvAVCvHVLLSrx7Q").unwrap(),
    //     Pubkey::from_str("9nnLbotNTcUhvbrsA6Mdkx45Sm82G35zo28AqUvjExn8").unwrap(),
    //     Pubkey::from_str("A8kEy5wWgdW4FG593fQJ5QPVbqx1wkfXw9c4L9bPo2CN").unwrap(),
    //     Pubkey::from_str("FGptqdxjahafaCzpZ1T6EDtCzYMv7Dyn5MgBLyB3VUFW").unwrap(),
    //     Pubkey::from_str("G55KMKRm78kjeswHep9gctN1Tj8KXh35Fs1jDdFFBUuZ").unwrap(),
    //     Pubkey::from_str("JAF1x1owVYpJQAU6iwZfmgRKR5iaQzqXQ9tzXd16DqPo").unwrap(),
    //     Pubkey::from_str("11111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("JUP6LkbZbjS1jKKwapdHNy74zcZ3tLUZoi5QNyVTaV4").unwrap(),
    //     Pubkey::from_str("h5NciPdMZ5QCB5BYETJMYBMpVx9ZuitR6HcVjyBhood").unwrap(),
    //     Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
    //     Pubkey::from_str("AVmoTthdrX6tKt4nDjco2D775W2YK3sDhxPcMmzUAmTY").unwrap(),
    //     Pubkey::from_str("D8cy77BBepLMngZx6ZukaTff5hCt1HrWyKk3Hnd9oitf").unwrap(),
    //     Pubkey::from_str("GjCj8ZMPr13p5XTJJALa7QoRJ1NUWgNcnmhWVgLsQEDr").unwrap(),
    //     Pubkey::from_str("J2nUHEAgZFRyuJbFjdqPrAa9gyWDuc7hErtDQHPhsYRp").unwrap(),
    // ];

    // let swap_transaction = match teste::decode_jupiter_instruction(
    //     data_complex_jupiter.to_vec(),
    //     accounts_jupiter.to_vec(),
    //     program_id_jupiter.unwrap(),
    // ) {
    //     Some(transaction) => {
    //         println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
    //     }
    //     None => {
    //         println!("❌ Erro: Falha ao decodificar a instrução Jupiter.");
    //         return; // Ou retorne um valor padrão
    //     }
    // };

    // // OKX TRANSACTION
    // let okx_data = vec![69, 164, 210, 89, 146, 214, 173, 67, 192, 36, 192, 158, 12, 0, 0, 0, 198, 82, 32, 15, 0, 0, 0, 0, 161, 153, 249, 14, 0, 0, 0, 0, 1, 0, 0, 0, 192, 36, 192, 158, 12, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 1, 0, 0, 0, 46, 1, 0, 0, 0, 100, 64, 66, 15, 0, 0, 0, 0, 0, 0, 50, 182, 1, 0, 0, 0, 0, 0];

    // // // // Defina o PROGRAM_ID (substitua pelo valor real)
    // // let program_id_okx = Pubkey::from_str("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma"); // Exemplo de PROGRA
    
    // let accounts_okx = [
    //     Pubkey::from_str("E5hTCL2cw2uN2ptS7dhecRJmCuyowsQKXo3maQdr3CFX").unwrap(),
    //     Pubkey::from_str("5eccg3ArzwoiRaqWEBeZLaKfx1PdVkZRY5YNFDbNTazD").unwrap(),
    //     Pubkey::from_str("7x4VcEX8aLd3kFsNWULTp1qFgVtDwyWSxpTGQkoMM6XX").unwrap(),
    //     Pubkey::from_str("8sHmeyfT23EZH2h4Mjg2qiS3hvzWM79VDAAZY57UoX43").unwrap(),
    //     Pubkey::from_str("6m2CDdhRgxpH4WjvdzxAYbGxwdGUz5MziiL5jek2kBma").unwrap(),
    //     Pubkey::from_str("85vdovHhkXnDi98EYMQmD2vXS82jRP1VDDXfkJ38pump").unwrap(),
    //     Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
    //     Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("jitodontfront11111111111JustUseJupiterU1tra").unwrap(),
    //     Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
    // ];

    // let swap_transaction = match teste::decode_okx_instruction(
    //     okx_data,
    //     accounts_okx.to_vec(),
    //     program_id_okx.unwrap(),
    // ) {
    //     Some(transaction) => {
    //         println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
    //     }
    //     None => {
    //         println!("❌ Erro: Falha ao decodificar a instrução OKX.");
    //         return; // Ou retorne um valor padrão
    //     }
    // };

    let accounts_raydiumlaunchpad = [
        Pubkey::from_str("Hck1nBKknZmW1a5JotomMvC7qmDinj5LqNV56T6WT1LB").unwrap(),
        Pubkey::from_str("WLHv2UAZm6z4KyaaELi5pjdbJh6RESMva1Rnn8pJVVh").unwrap(),
        Pubkey::from_str("6s1xP3hpbAfFoNtUNF8mfHsjr2Bd97JxFJRWLbL6aHuX").unwrap(),
        Pubkey::from_str("FfYek5vEz23cMkWsdJwG2oa6EphsvXSHrGpdALN4g6W1").unwrap(),
        Pubkey::from_str("9Y5bfAfB7jfPjiWH3tCnbGU6wUXLSXefpbfanvDUh4tp").unwrap(),
        Pubkey::from_str("7wPkxubdKKcFAxSq6xVHbwi1kwU4Ri8e9tic9Jw9a7qA").unwrap(),
        Pubkey::from_str("9VjogpTgTCbo3HJwLdUtmbFHZUSZWxghGBMwJr4tLHMx").unwrap(),
        Pubkey::from_str("3FEvw6MvoABy3sEyxTgx836RoJySJQ1PwcRcQcg7UWMm").unwrap(),
        Pubkey::from_str("6TZzcf8sqxpyR3sp72UEimVH6RT8Xt9TkiHCoNZhyZeL").unwrap(),
        Pubkey::from_str("8xQAYsMk7nfyafKimzeJFQnxkKSnnXzpXdBjMDAmbonk").unwrap(),
        Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
        Pubkey::from_str("2DPAtwB8L12vrMRExbLuyGnC7n2J5LNoZQSejeQGpwkr").unwrap(),
        Pubkey::from_str("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj").unwrap(),
    ];

    let raydiumlaunchpad_data = vec![250, 234, 13, 123, 213, 156, 19, 236, 64, 66, 15, 0, 0, 0, 0, 0, 161, 150, 246, 24, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0];

    let program_id_launchpad = Pubkey::from_str("LanMV9sAd7wArD4vJFi2qDdfnVhFxYSUg6eADduJ3uj"); // Exemplo de PROGRA


    let swap_transaction = match teste::decode_raydiumlaunchpad_instruction(
        raydiumlaunchpad_data,
        accounts_raydiumlaunchpad.to_vec(),
        program_id_launchpad.unwrap(),
    ) {
        Some(transaction) => {
            println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
        }
        None => {
            println!("❌ Erro: Falha ao decodificar a instrução OKX.");
            return; // Ou retorne um valor padrão
        }
    };

    // pumpswap teste

    // let data_pumpswap = vec![51, 230, 133, 164, 1, 127, 131, 173, 0, 9, 237, 122, 72, 0, 0, 0, 109, 58, 234, 56, 1, 0, 0, 0];

    // let program_id_pumpswap = Pubkey::from_str("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA"); // Exemplo de PROGRA

    // let accounts_pumpswap = [
    //     Pubkey::from_str("7RJu1AC6L2ocYohCv6Q6Fej2k5aWdsNxPbZH9SXkNUaL").unwrap(),
    //     Pubkey::from_str("FfZd4hbNCFBwfPq6pprY6LZBchqbrZgpDc2tehm8zqUc").unwrap(),
    //     Pubkey::from_str("ADyA8hdefvWN2dbGGWFotbzWxrAvLW83WG6QCVXvJKqw").unwrap(),
    //     Pubkey::from_str("AKzF8KmQ31t2exifCjSiyE6USuaRz73zY6aUd7PYDrCM").unwrap(),
    //     Pubkey::from_str("7TptSdvSXvEt3L6mhLJM3fA7u2FF465mkRocvvytSA4k").unwrap(),
    //     Pubkey::from_str("9wCHr6ZmnQ1aFvx5UQXHdhtEma5VTDGWdn6JUrmNGHjk").unwrap(),
    //     Pubkey::from_str("DWpvfqzGWuVy9jVSKSShdM2733nrEsnnhsUStYbkj6Nn").unwrap(),
    //     Pubkey::from_str("4R7mx3L4U3xfPpspFv7H28BGu4M1NgefYCwpNZsHKbbz").unwrap(),
    //     Pubkey::from_str("BB5dnY55FXS1e1NXqZDwCzgdYJdMCj3B92PU6Q5Fb6DT").unwrap(),
    //     Pubkey::from_str("12pHu2j2DDShyCVFU7vtSLXga74et9y83VD38mw6XYhB").unwrap(),
    //     Pubkey::from_str("ComputeBudget111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("jitodontfront111111111115111111111111165531").unwrap(),
    //     Pubkey::from_str("ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL").unwrap(),
    //     Pubkey::from_str("So11111111111111111111111111111111111111112").unwrap(),
    //     Pubkey::from_str("11111111111111111111111111111111").unwrap(),
    //     Pubkey::from_str("TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA").unwrap(),
    //     Pubkey::from_str("pAMMBay6oceH9fJKBRHGP5D4bD4sWpmSwMn52FMfXEA").unwrap(),
    //     Pubkey::from_str("H6uLH57BVuRLTN4zEpAxPr6E6afw8wnxZvfBZFYJzRgG").unwrap(),
    //     Pubkey::from_str("85vdovHhkXnDi98EYMQmD2vXS82jRP1VDDXfkJ38pump").unwrap(),
    //     Pubkey::from_str("JCRGumoE9Qi5BBgULTgdgTLjSgkCMSbF62ZZfGs84JeU").unwrap(),
    //     Pubkey::from_str("GS4CU59F31iL7aR2Q8zVS8DRrcRnXX1yjQ66TqNVQnaR").unwrap(),
    //     Pubkey::from_str("FMue64NAyahSTHkQk1BUvPmjxy8SZUmWNtgi7DNdCwBC").unwrap(),
    // ];

    // let swap_transaction = match teste::decode_pumpswap_instruction(
    //     data_pumpswap.to_vec(),
    //     accounts_pumpswap.to_vec(),
    //     program_id_pumpswap.unwrap(),
    // ) {
    //     Some(transaction) => {
    //         println!("{:?}", transaction); // ✅ Sucesso: pega o valor e imprime
    //     }
    //     None => {
    //         println!("❌ Erro: Falha ao decodificar a instrução Jupiter.");
    //         return; // Ou retorne um valor padrão
    //     }
    // };

}
