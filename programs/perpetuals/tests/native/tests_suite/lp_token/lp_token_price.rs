use {
    crate::{test_instructions, utils},
    maplit::hashmap,
    perpetuals::{instructions::SetCustomOraclePriceParams, state::cortex::Cortex},
};

const USDC_DECIMALS: u8 = 6;
const ETH_DECIMALS: u8 = 9;

pub async fn lp_token_price() {
    let test_setup = utils::TestSetup::new(
        vec![utils::UserParam {
            name: "alice",
            token_balances: hashmap! {
                "usdc" => utils::scale(100_000, USDC_DECIMALS),
                "eth" => utils::scale(50, ETH_DECIMALS),
            },
        }],
        vec![
            utils::MintParam {
                name: "usdc",
                decimals: USDC_DECIMALS,
            },
            utils::MintParam {
                name: "eth",
                decimals: ETH_DECIMALS,
            },
        ],
        vec!["admin_a", "admin_b", "admin_c"],
        "usdc",
        "usdc",
        6,
        "ADRENA",
        "main_pool",
        vec![
            utils::SetupCustodyWithLiquidityParams {
                setup_custody_params: utils::SetupCustodyParams {
                    mint_name: "usdc",
                    is_stable: true,
                    is_virtual: false,
                    target_ratio: utils::ratio_from_percentage(50.0),
                    min_ratio: utils::ratio_from_percentage(0.0),
                    max_ratio: utils::ratio_from_percentage(100.0),
                    initial_price: utils::scale(1, USDC_DECIMALS),
                    initial_conf: utils::scale_f64(0.01, USDC_DECIMALS),
                    pricing_params: None,
                    permissions: None,
                    fees: None,
                    borrow_rate: None,
                },
                liquidity_amount: utils::scale(15_000, USDC_DECIMALS),
                payer_user_name: "alice",
            },
            utils::SetupCustodyWithLiquidityParams {
                setup_custody_params: utils::SetupCustodyParams {
                    mint_name: "eth",
                    is_stable: false,
                    is_virtual: false,
                    target_ratio: utils::ratio_from_percentage(50.0),
                    min_ratio: utils::ratio_from_percentage(0.0),
                    max_ratio: utils::ratio_from_percentage(100.0),
                    initial_price: utils::scale(1_500, ETH_DECIMALS),
                    initial_conf: utils::scale(10, ETH_DECIMALS),
                    pricing_params: None,
                    permissions: None,
                    fees: None,
                    borrow_rate: None,
                },
                liquidity_amount: utils::scale(10, ETH_DECIMALS),
                payer_user_name: "alice",
            },
        ],
        utils::scale(1_000_000, Cortex::LM_DECIMALS),
        utils::scale(1_000_000, Cortex::LM_DECIMALS),
        utils::scale(1_000_000, Cortex::LM_DECIMALS),
        utils::scale(1_000_000, Cortex::LM_DECIMALS),
    )
    .await;

    let admin_a = test_setup.get_multisig_member_keypair_by_name("admin_a");

    let multisig_signers = test_setup.get_multisig_signers();

    // Check LP token price after pool setup
    assert_eq!(
        test_instructions::get_lp_token_price(
            &mut test_setup.program_test_ctx.borrow_mut(),
            &test_setup.payer_keypair,
            &test_setup.pool_pda,
            &test_setup.lp_token_mint_pda,
        )
        .await
        .unwrap(),
        1_051_947
    );

    // Increase asset price and check that lp token price increase
    {
        // Makes ETH price to increase of 10%
        {
            let eth_test_oracle_pda = test_setup.custodies_info[1].custom_oracle_pda;
            let eth_custody_pda = test_setup.custodies_info[1].custody_pda;

            let publish_time =
                utils::get_current_unix_timestamp(&mut test_setup.program_test_ctx.borrow_mut())
                    .await;

            test_instructions::set_custom_oracle_price(
                &mut test_setup.program_test_ctx.borrow_mut(),
                admin_a,
                &test_setup.payer_keypair,
                &test_setup.pool_pda,
                &eth_custody_pda,
                &eth_test_oracle_pda,
                SetCustomOraclePriceParams {
                    price: utils::scale(1_650, ETH_DECIMALS),
                    expo: -(ETH_DECIMALS as i32),
                    conf: utils::scale(10, ETH_DECIMALS),
                    publish_time,
                },
                &multisig_signers,
            )
            .await
            .unwrap();
        }

        assert_eq!(
            test_instructions::get_lp_token_price(
                &mut test_setup.program_test_ctx.borrow_mut(),
                &test_setup.payer_keypair,
                &test_setup.pool_pda,
                &test_setup.lp_token_mint_pda,
            )
            .await
            .unwrap(),
            1_105_177
        );
    }

    // Decrease asset price and check that lp token price decrease
    {
        // Makes ETH price to decrease of 20%
        {
            let eth_test_oracle_pda = test_setup.custodies_info[1].custom_oracle_pda;
            let eth_custody_pda = test_setup.custodies_info[1].custody_pda;

            let publish_time =
                utils::get_current_unix_timestamp(&mut test_setup.program_test_ctx.borrow_mut())
                    .await;

            test_instructions::set_custom_oracle_price(
                &mut test_setup.program_test_ctx.borrow_mut(),
                admin_a,
                &test_setup.payer_keypair,
                &test_setup.pool_pda,
                &eth_custody_pda,
                &eth_test_oracle_pda,
                SetCustomOraclePriceParams {
                    price: utils::scale(1_320, ETH_DECIMALS),
                    expo: -(ETH_DECIMALS as i32),
                    conf: utils::scale(10, ETH_DECIMALS),
                    publish_time,
                },
                &multisig_signers,
            )
            .await
            .unwrap();
        }

        assert_eq!(
            test_instructions::get_lp_token_price(
                &mut test_setup.program_test_ctx.borrow_mut(),
                &test_setup.payer_keypair,
                &test_setup.pool_pda,
                &test_setup.lp_token_mint_pda,
            )
            .await
            .unwrap(),
            988_072
        );
    }
}
